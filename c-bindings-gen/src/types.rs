use std::collections::HashMap;

use proc_macro2::{TokenTree, Span};

// We have a few main types -
// * Unit Enums, which are "mirrored" by creating a #[repr(C)] version and exposing it.
// * Traits, which map to a void* and a jump table,
// * Opaque Structs, which are exposed as-is either directly or via a typedef if they are generic
//   (using our own concrete traits, which are enumerated below).
//   All pub methods are exposed and pub fields are exposed via geters/setters.

// The following types are used purely to build our known types maps - they break down all the
// types we need to resolve to include the given object, and no more.

pub fn first_seg_self<'a>(t: &'a syn::Type) -> Option<impl Iterator<Item=&syn::PathSegment> + 'a> {
	match t {
		syn::Type::Path(p) => {
			if p.qself.is_some() || p.path.leading_colon.is_some() {
				return None;
			}
			let mut segs = p.path.segments.iter();
			let ty = segs.next().unwrap();
			if !ty.arguments.is_empty() { return None; }
			if format!("{}", ty.ident) == "Self" {
				Some(segs)
			} else { None }
		},
		_ => None,
	}
}

pub fn get_single_remaining_path_seg<'a, I: Iterator<Item=&'a syn::PathSegment>>(segs: &mut I) -> Option<&'a syn::Ident> {
	if let Some(ty) = segs.next() {
		if !ty.arguments.is_empty() { unimplemented!(); }
		if segs.next().is_some() { return None; }
		Some(&ty.ident)
	} else { None }
}

pub fn assert_single_path_seg<'a>(p: &'a syn::Path) -> &'a syn::Ident {
	if p.leading_colon.is_some() { unimplemented!(); }
	get_single_remaining_path_seg(&mut p.segments.iter()).unwrap()
}

pub fn single_ident_generic_path_to_ident(p: &syn::Path) -> Option<&syn::Ident> {
	if p.segments.len() == 1 {
		Some(&p.segments.iter().next().unwrap().ident)
	} else { None }
}

#[derive(Debug, PartialEq)]
pub enum ExportStatus {
	Export,
	NoExport,
	TestOnly,
}
pub fn export_status(attrs: &[syn::Attribute]) -> ExportStatus {
	for attr in attrs.iter() {
		let tokens_clone = attr.tokens.clone();
		let mut token_iter = tokens_clone.into_iter();
		if let Some(token) = token_iter.next() {
			match token {
				TokenTree::Punct(c) if c.as_char() == '=' => {
					// Really not sure where syn gets '=' from here -
					// it somehow represents '///' or '//!'
				},
				TokenTree::Group(g) => {
					if format!("{}", single_ident_generic_path_to_ident(&attr.path).unwrap()) == "cfg" {
						if let TokenTree::Ident(i) = g.stream().into_iter().next().unwrap() {
							if i == "test" || i == "feature" {
								// If its cfg(feature(...)) we assume its test-only
								return ExportStatus::TestOnly;
							}
						}
					}
					continue; // eg #[derive()]
				},
				_ => unimplemented!(),
			}
		} else { continue; }
		match token_iter.next().unwrap() {
			TokenTree::Literal(lit) => {
				let line = format!("{}", lit);
				if line.contains("(C-not exported)") {
					return ExportStatus::NoExport;
				}
			},
			_ => unimplemented!(),
		}
	}
	ExportStatus::Export
}

pub fn assert_simple_bound(bound: &syn::TraitBound) {
	if bound.paren_token.is_some() || bound.lifetimes.is_some() { unimplemented!(); }
	if let syn::TraitBoundModifier::Maybe(_) = bound.modifier { unimplemented!(); }
}

pub struct GenericTypes<'a> {
	typed_generics: HashMap<&'a syn::Ident, (String, Option<&'a syn::Ident>)>,
}
impl<'a> GenericTypes<'a> {
	pub fn new() -> Self {
		Self { typed_generics: HashMap::new(), }
	}

	pub fn learn_generics(&mut self, generics: &'a syn::Generics, types: &'a TypeResolver<'a>) -> bool {
		for generic in generics.params.iter() {
			match generic {
				syn::GenericParam::Type(type_param) => {
					if type_param.bounds.len() > 1 { return false; }
					let bound = type_param.bounds.iter().next().unwrap();
					if let syn::TypeParamBound::Trait(trait_bound) = bound {
						assert_simple_bound(&trait_bound);
						let mut path = types.resolve_path(&trait_bound.path);
						let new_ident = if path != "std::ops::Deref" {
							path = "crate::".to_string() + &path;
							Some(assert_single_path_seg(&trait_bound.path))
						} else { None };
						self.typed_generics.insert(&type_param.ident, (path, new_ident));
					}
				},
				_ => {},
			}
		}
		if let Some(wh) = &generics.where_clause {
			for pred in wh.predicates.iter() {
				if let syn::WherePredicate::Type(t) = pred {
					if let syn::Type::Path(p) = &t.bounded_ty {
						if p.qself.is_some() { return false; }
						if p.path.leading_colon.is_some() { return false; }
						let mut p_iter = p.path.segments.iter();
						let gen = self.typed_generics.get_mut(&p_iter.next().unwrap().ident).unwrap();
						if gen.0 != "std::ops::Deref" { return false; }
						if &format!("{}", p_iter.next().unwrap().ident) != "Target" { return false; }
						if t.bounds.len() != 1 { return false; }
						if let syn::TypeParamBound::Trait(trait_bound) = t.bounds.iter().next().unwrap() {
							assert_simple_bound(&trait_bound);
							*gen = ("crate::".to_string() + &types.resolve_path(&trait_bound.path),
								Some(single_ident_generic_path_to_ident(&trait_bound.path).unwrap()));
						}
					} else { return false; }
				}
			}
		}
		for (_, (_, ident)) in self.typed_generics.iter() {
			if ident.is_none() { return false; }
		}
		true
	}

	pub fn maybe_resolve_ident<'b>(&'b self, ident: &syn::Ident) -> Option<&'b String> {
		self.typed_generics.get(ident).map(|(a, _)| a)
	}
	pub fn maybe_resolve_path<'b>(&'b self, path: &syn::Path) -> Option<(&'b String, &'a syn::Ident)> {
		if let Some(ident) = path.get_ident() {
			self.typed_generics.get(ident).map(|(a, b)| (a, b.unwrap()))
		} else { None }
	}
}

#[derive(PartialEq)]
// The type of declaration and the object itself
pub enum DeclType<'a> {
	MirroredEnum,
	Trait(&'a syn::ItemTrait),
	StructImported(String),
	EnumIgnored,
}

pub struct TypeResolver<'a> {
	module_path: &'a str,
	imports: HashMap<syn::Ident, String>,
	// ident -> is-mirrored-enum
	declared: HashMap<syn::Ident, DeclType<'a>>,
}

impl<'a> TypeResolver<'a> {
	pub fn new(module_path: &'a str) -> Self {
		let mut imports = HashMap::new();
		// Add primitives to the "imports" list:
		imports.insert(syn::Ident::new("bool", Span::call_site()), "bool".to_string());
		imports.insert(syn::Ident::new("u64", Span::call_site()), "u64".to_string());
		imports.insert(syn::Ident::new("u32", Span::call_site()), "u32".to_string());
		imports.insert(syn::Ident::new("u16", Span::call_site()), "u16".to_string());
		imports.insert(syn::Ident::new("u8", Span::call_site()), "u8".to_string());
		imports.insert(syn::Ident::new("usize", Span::call_site()), "usize".to_string());

		// These are here to allow us to print native Rust types in trait fn impls even if we don't
		// have C mappings:
		imports.insert(syn::Ident::new("Result", Span::call_site()), "Result".to_string());
		imports.insert(syn::Ident::new("Option", Span::call_site()), "Option".to_string());
		Self { module_path, imports, declared: HashMap::new() }
	}

	// *** Well know type definitions ***
	/// Returns true we if can just skip passing this to C entirely
	fn skip_path(&self, full_path: &str) -> bool {
		full_path == "bitcoin::secp256k1::Secp256k1"
	}

	fn is_primitive(&self, full_path: &str) -> bool {
		match full_path {
			"bool" => true,
			"u64" => true,
			"u32" => true,
			"u16" => true,
			"u8" => true,
			"usize" => true,
			_ => false,
		}
	}
	fn c_type_from_path<'b>(&self, full_path: &'b str, is_ref: bool) -> Option<&'b str> {
		if !is_ref && self.is_primitive(full_path) {
			return Some(full_path);
		}
		match full_path {
			"bitcoin::secp256k1::key::PublicKey"  => Some("crate::c_types::PublicKey"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some("crate::c_types::SecretKey"),
			"bitcoin::blockdata::script::Script" => Some("crate::c_types::Script"),
			"bitcoin::blockdata::transaction::Transaction" => Some("crate::c_types::Transaction"),
			"bitcoin::network::constants::Network" => Some("crate::bitcoin::network::Network"),
			"bitcoin::blockdata::block::BlockHeader" if is_ref  => Some("*const [u8; 80]"),
			"bitcoin::blockdata::block::BlockHeader" if !is_ref => Some("[u8; 80]"),
			"bitcoin::hash_types::Txid" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::hash_types::Txid" if !is_ref => Some("[u8; 32]"),
			"bitcoin::hash_types::BlockHash" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::hash_types::BlockHash" if !is_ref => Some("[u8; 32]"),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some("*const std::os::raw::c_char"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some("*const crate::ln::features::InitFeatures"),
			"ln::features::InitFeatures" => Some("crate::ln::features::InitFeatures"),
			"util::config::UserConfig" if !is_ref => Some("crate::util::config::UserConfig"),

			// List of traits we map (possibly during processing of other files):
			"util::logger::Logger" => Some("crate::util::logger::Logger"),
			"chain::chaininterface::BroadcasterInterface" => Some("crate::chain::chaininterface::BroadcasterInterface"),
			"chain::chaininterface::FeeEstimator" => Some("crate::chain::chaininterface::FeeEstimator"),
			"chain::keysinterface::KeysInterface" => Some("crate::chain::keysinterface::KeysInterface"),
			"ln::channelmonitor::ManyChannelMonitor" => Some("crate::ln::channelmonitor::ManyChannelMonitor"),
			"ln::msgs::ChannelMessageHandler" if is_ref => Some("*const crate::ln::msgs::ChannelMessageHandler"),
			"ln::msgs::ChannelMessageHandler" => Some("crate::ln::msgs::ChannelMessageHandler"),
			"ln::msgs::RoutingMessageHandler" if is_ref => Some("*const crate::ln::msgs::RoutingMessageHandler"),
			"ln::msgs::RoutingMessageHandler" => Some("crate::ln::msgs::RoutingMessageHandler"),
			_ => {
				eprintln!("    Type {} (ref: {}) unresolvable to C", full_path, is_ref);
				None
			},
		}
	}

	fn is_known_container(&self, full_path: &str) -> bool {
		full_path == "std::sync::Arc"
	}
	fn from_c_conversion_new_var_from_path_prefix(&self, full_path: &str) -> Option<&str> {
		match full_path {
			"std::sync::Arc" => Some("std::sync::Arc::new("),
			_ => None,
		}
	}

	fn from_c_conversion_prefix_from_path(&self, full_path: &str, is_ref: bool) -> Option<&str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"bitcoin::secp256k1::key::PublicKey" => Some(""),
			"bitcoin::secp256k1::key::SecretKey" if is_ref => unimplemented!(),
			"bitcoin::secp256k1::key::SecretKey" => Some(""),
			"bitcoin::blockdata::script::Script" => Some(""),
			"bitcoin::blockdata::transaction::Transaction" => Some(""),
			"bitcoin::network::constants::Network" => Some(""),
			"bitcoin::blockdata::block::BlockHeader" => Some("::bitcoin::consensus::encode::deserialize(&"),
			"bitcoin::hash_types::Txid" => Some("::bitcoin::hash_types::Txid::from_slice(&"),
			"bitcoin::hash_types::BlockHash" => Some("::bitcoin::hash_types::BlockHash::from_slice(&"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if !is_ref => Some("*unsafe { Box::from_raw("),
			"util::config::UserConfig" if !is_ref => Some("*unsafe { Box::from_raw("),

			// List of traits we map (possibly during processing of other files):
			"crate::chain::chaininterface::BroadcasterInterface" => Some(""),
			"crate::chain::chaininterface::FeeEstimator" => Some(""),
			"crate::chain::keysinterface::KeysInterface" => Some(""),
			"crate::ln::channelmonitor::ManyChannelMonitor" => Some(""),
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable from C", full_path);
				None
			},
		}
	}
	fn from_c_conversion_suffix_from_path(&self, full_path: &str) -> Option<&str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"bitcoin::secp256k1::key::PublicKey" => Some(".into_rust()"),
			"bitcoin::secp256k1::key::SecretKey" => Some(".into_rust()"),
			"bitcoin::blockdata::script::Script" => Some(".into_bitcoin()"),
			"bitcoin::blockdata::transaction::Transaction" => Some(".into_bitcoin()"),
			"bitcoin::network::constants::Network" => Some(".into_bitcoin()"),
			"bitcoin::blockdata::block::BlockHeader" => Some("[..]).unwrap()"),
			"bitcoin::hash_types::Txid" => Some("[..]).unwrap()"),
			"bitcoin::hash_types::BlockHash" => Some("[..]).unwrap()"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" => Some(".inner as *mut _) }"),
			"util::config::UserConfig" => Some(".inner as *mut lightning::util::config::UserConfig) }"),

			// List of traits we map (possibly during processing of other files):
			"crate::chain::chaininterface::BroadcasterInterface" => Some(""),
			"crate::chain::chaininterface::FeeEstimator" => Some(""),
			"crate::chain::keysinterface::KeysInterface" => Some(""),
			"crate::ln::channelmonitor::ManyChannelMonitor" => Some(""),
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable from C", full_path);
				None
			},
		}
	}

	fn to_c_conversion_new_var_from_path(&self, full_path: &str) -> Option<(&str, &str)> {
		if self.is_primitive(full_path) {
			return None;
		}
		match full_path {
			"bitcoin::blockdata::script::Script" => Some(("::bitcoin::consensus::encode::serialize(", ")")),
			"bitcoin::blockdata::transaction::Transaction" => Some(("::bitcoin::consensus::encode::serialize(", ")")),
			"bitcoin::hash_types::Txid" => None,

			// Override the default since Records contain an fmt with a lifetime:
			// TODO: We should include the other record fields
			"util::logger::Record" => Some(("std::ffi::CString::new(format!(\"{}\", ", ".args)).unwrap()")),
			_ => None,
		}
	}

	fn to_c_conversion_inline_prefix_from_path(&self, full_path: &str, is_ref: bool) -> Option<&str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"bitcoin::secp256k1::key::PublicKey" if is_ref => Some("crate::c_types::PublicKey::from_rust("),
			"bitcoin::secp256k1::key::PublicKey" => Some("crate::c_types::PublicKey::from_rust(&"),
			"bitcoin::blockdata::script::Script" => Some("crate::c_types::Script::from_slice(&c_"),
			"bitcoin::blockdata::transaction::Transaction" => Some("crate::c_types::Transaction::from_slice(&c_"),
			"bitcoin::hash_types::Txid" => Some(""),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some("c_"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some("Box::into_raw(Box::new(crate::ln::features::InitFeatures { inner: "),
			"ln::features::InitFeatures" if !is_ref => Some("crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new("),

			// List of traits we map (possibly during processing of other files):
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			_ => {
				eprintln!("    Type {} (is_ref: {}) unconvertable to C", full_path, is_ref);
				None
			},
		}
	}
	fn to_c_conversion_inline_suffix_from_path(&self, full_path: &str, is_ref: bool) -> Option<&str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"bitcoin::secp256k1::key::PublicKey" => Some(")"),
			"bitcoin::blockdata::script::Script" => Some(")"),
			"bitcoin::blockdata::transaction::Transaction" => Some(")"),
			"bitcoin::hash_types::Txid" => Some(".into_inner()"),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some(".as_ptr()"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some(" }))"),
			"ln::features::InitFeatures" => Some(")) }"),

			// List of traits we map (possibly during processing of other files):
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable to C", full_path);
				None
			},
		}
	}

	fn process_use_intern<W: std::io::Write>(&mut self, w: &mut W, u: &syn::UseTree, partial_path: &str) {
		match u {
			syn::UseTree::Path(p) => {
				let new_path = format!("{}::{}", partial_path, p.ident);
				self.process_use_intern(w, &p.tree, &new_path);
			},
			syn::UseTree::Name(n) => {
				let full_path = format!("{}::{}", partial_path, n.ident);
				if full_path.starts_with("bitcoin::") {
					write!(w, "use {} as ln{};\n", full_path, n.ident).unwrap();
				}
				self.imports.insert(n.ident.clone(), full_path);
			},
			syn::UseTree::Group(g) => {
				for i in g.items.iter() {
					self.process_use_intern(w, i, partial_path);
				}
			},
			syn::UseTree::Rename(r) => {
				let full_path = format!("{}::{}", partial_path, r.ident);
				self.imports.insert(r.rename.clone(), full_path);
			},
			syn::UseTree::Glob(_) => {
				eprintln!("Ignoring * use for {} - this may result in resolution failures", partial_path);
			},
		}
	}
	pub fn process_use<W: std::io::Write>(&mut self, w: &mut W, u: &syn::ItemUse) {
		if let syn::Visibility::Public(_) = u.vis {
			// We actually only use these for #[cfg(fuzztarget)]
			eprintln!("Ignoring pub(use) tree!");
			return;
		}
		match &u.tree {
			syn::UseTree::Path(p) => {
				let new_path = format!("{}", p.ident);
				self.process_use_intern(w, &p.tree, &new_path);
			},
			_ => unimplemented!(),
		}
		if u.leading_colon.is_some() { unimplemented!() }
	}

	pub fn mirrored_enum_declared(&mut self, ident: &syn::Ident) {
		eprintln!("{} mirrored", ident);
		self.declared.insert(ident.clone(), DeclType::MirroredEnum);
	}
	pub fn enum_ignored(&mut self, ident: &syn::Ident) {
		self.declared.insert(ident.clone(), DeclType::EnumIgnored);
	}
	pub fn struct_imported(&mut self, ident: &syn::Ident, named: String) {
		eprintln!("Imported {} as {}", ident, named);
		self.declared.insert(ident.clone(), DeclType::StructImported(named));
	}
	pub fn trait_declared(&mut self, ident: &syn::Ident, t: &'a syn::ItemTrait) {
		eprintln!("Trait {} created", ident);
		self.declared.insert(ident.clone(), DeclType::Trait(t));
	}
	pub fn get_declared_type(&'a self, ident: &syn::Ident) -> Option<&'a DeclType> {
		self.declared.get(ident)
	}

	pub fn maybe_resolve_path(&self, p: &syn::Path) -> Option<String> {
		if p.leading_colon.is_some() {
			//format!("{}", p.segments);
			return None;
		} else if let Some(id) = p.get_ident() {
			if let Some(imp) = self.imports.get(id) {
				Some(imp.clone())
			} else if self.declared.get(id).is_some() {
				Some(self.module_path.to_string() + "::" + &format!("{}", id))
			} else { None }
		} else {
			if p.segments.len() == 1 {
				let seg = p.segments.iter().next().unwrap();
				if let Some(imp) = self.imports.get(&seg.ident) {
					return Some(imp.clone());
				} else if self.declared.get(&seg.ident).is_some() {
					return Some(self.module_path.to_string() + "::" + &format!("{}", seg.ident));
				} else { return None; }
			}
			let mut seg_iter = p.segments.iter();
			let first_seg = seg_iter.next().unwrap();
			let remaining: String = seg_iter.map(|seg| {
				if let syn::PathArguments::None = seg.arguments {
					format!("{}", seg.ident)
				} else {
					format!("{}", seg.ident)
				}
			}).collect();
			if let Some(imp) = self.imports.get(&first_seg.ident) {
				if remaining != "" {
					Some(imp.clone() + "::" + &remaining)
				} else {
					Some(imp.clone())
				}
			} else { None }
		}
	}
	pub fn resolve_path(&self, p: &syn::Path) -> String {
		self.maybe_resolve_path(p).unwrap()
	}

	fn print_rust_path<W: std::io::Write>(&self, w: &mut W, path: &syn::Path) {
		if let Some(resolved) = self.maybe_resolve_path(&path) {
			if self.is_primitive(&resolved) {
				write!(w, "{}", path.get_ident().unwrap()).unwrap();
			} else {
				if resolved.starts_with("ln::") || resolved.starts_with("chain::") || resolved.starts_with("util::") {
					write!(w, "lightning::{}", resolved).unwrap();
				} else {
					write!(w, "{}", resolved).unwrap(); // XXX: Probably doens't work, get_ident().unwrap()
				}
			}
			if let syn::PathArguments::AngleBracketed(args) = &path.segments.iter().last().unwrap().arguments {
				self.print_rust_generic_arg(w, args.args.iter());
			}
		} else {
			if path.leading_colon.is_some() {
				write!(w, "::").unwrap();
			}
			for (idx, seg) in path.segments.iter().enumerate() {
				if idx != 0 { write!(w, "::").unwrap(); }
				write!(w, "{}", seg.ident).unwrap();
				if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
					self.print_rust_generic_arg(w, args.args.iter());
				}
			}
		}
	}
	pub fn print_rust_generic_param<'b, W: std::io::Write>(&self, w: &mut W, generics: impl Iterator<Item=&'b syn::GenericParam>) {
		let mut had_params = false;
		for (idx, arg) in generics.enumerate() {
			if idx != 0 { write!(w, ", ").unwrap(); } else { write!(w, "<").unwrap(); }
			had_params = true;
			match arg {
				syn::GenericParam::Lifetime(lt) => write!(w, "'{}", lt.lifetime.ident).unwrap(),
				syn::GenericParam::Type(t) => {
					write!(w, "{}", t.ident).unwrap();
					if t.colon_token.is_some() { write!(w, ":").unwrap(); }
					for (idx, bound) in t.bounds.iter().enumerate() {
						if idx != 0 { write!(w, " + ").unwrap(); }
						match bound {
							syn::TypeParamBound::Trait(tb) => {
								if tb.paren_token.is_some() || tb.lifetimes.is_some() { unimplemented!(); }
								self.print_rust_path(w, &tb.path);
							},
							_ => unimplemented!(),
						}
					}
					if t.eq_token.is_some() || t.default.is_some() { unimplemented!(); }
				},
				_ => unimplemented!(),
			}
		}
		if had_params { write!(w, ">").unwrap(); }
	}

	pub fn print_rust_generic_arg<'b, W: std::io::Write>(&self, w: &mut W, generics: impl Iterator<Item=&'b syn::GenericArgument>) {
		write!(w, "<").unwrap();
		for (idx, arg) in generics.enumerate() {
			if idx != 0 { write!(w, ", ").unwrap(); }
			match arg {
				syn::GenericArgument::Type(t) => self.print_rust_type(w, t),
				_ => unimplemented!(),
			}
		}
		write!(w, ">").unwrap();
	}
	pub fn print_rust_type<W: std::io::Write>(&self, w: &mut W, t: &syn::Type) {
//eprintln!("lk: {:?}", t);
		match t {
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}
				self.print_rust_path(w, &p.path);
			},
			syn::Type::Reference(r) => {
				write!(w, "&").unwrap();
				if let Some(lft) = &r.lifetime {
					write!(w, "'{} ", lft.ident).unwrap();
				}
				if r.mutability.is_some() {
					write!(w, "mut ").unwrap();
				}
				self.print_rust_type(w, &*r.elem);
			},
			syn::Type::Array(a) => {
				write!(w, "[").unwrap();
				self.print_rust_type(w, &a.elem);
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						write!(w, "; {}]", i).unwrap();
					} else { unimplemented!(); }
				} else { unimplemented!(); }
			}
			syn::Type::Slice(s) => {
				write!(w, "[").unwrap();
				self.print_rust_type(w, &s.elem);
				write!(w, "]").unwrap();
			},
			syn::Type::Tuple(s) => {
				write!(w, "(").unwrap();
				for (idx, t) in s.elems.iter().enumerate() {
					if idx != 0 { write!(w, ", ").unwrap(); }
					self.print_rust_type(w, &t);
				}
				write!(w, ")").unwrap();
			},
			_ => unimplemented!(),
		}
	}

	/// Returns true we if can just skip passing this to C entirely
	pub fn skip_arg(&self, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		match t {
			syn::Type::Path(p) => {
				if p.qself.is_some() { unimplemented!(); }
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						return self.skip_path(resolved.0);
					}
				}
				if let Some(full_path) = self.maybe_resolve_path(&p.path) {
					self.skip_path(&full_path)
				} else { false }
			},
			syn::Type::Reference(r) => {
				self.skip_arg(&*r.elem, generics)
			},
			_ => false,
		}
	}

	fn print_to_c_conversion_inline_prefix_intern<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool) {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_to_c_conversion_inline_prefix_intern(w, &*r.elem, generics, true, ptr_for_ref);
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}
				let ident = single_ident_generic_path_to_ident(&p.path).unwrap();

				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0));
						if let Some(c_type) = self.to_c_conversion_inline_prefix_from_path(&resolved.0, is_ref) {
							write!(w, "{}", c_type).unwrap();
							return;
						} else { unimplemented!(); }
					}
				}

				if let Some(c_type) = self.to_c_conversion_inline_prefix_from_path(&self.resolve_path(&p.path), is_ref) {
					write!(w, "{}", c_type).unwrap();
				} else if let Some(_) = self.imports.get(ident) {
					// to_c_conversion_inline_prefix_from_path has to have succeeded:
					unimplemented!();
				} else if let Some(decl_type) = self.declared.get(ident) {
					match decl_type {
						DeclType::MirroredEnum => write!(w, "{}::from_ln(", ident).unwrap(),
						DeclType::StructImported(name) if is_ref && ptr_for_ref =>
							write!(w, "Box::into_raw(Box::new({} {{ inner: ", name).unwrap(),
						DeclType::StructImported(name) if is_ref =>
							write!(w, "&{} {{ inner: ", name).unwrap(),
						DeclType::StructImported(name) if !is_ref =>
							write!(w, "{} {{ inner: Box::into_raw(Box::new(", name).unwrap(),
						_ => {},
					}
				} else { unimplemented!(); }
			},
			syn::Type::Array(a) => {
				// We assume all arrays contain only primitive types.
				// This may result in some outputs not compiling.
				if !is_ref {
					if let syn::Expr::Lit(l) = &a.len {
						if let syn::Lit::Int(i) = &l.lit {
							if i.base10_digits() != "32" {
								unimplemented!();
							}
						} else { unimplemented!(); }
					} else { unimplemented!(); }
					write!(w, "crate::c_types::ThirtyTwoBytes {{ data: ").unwrap();
				}
			},
			syn::Type::Slice(s) => {
				if let syn::Type::Path(p) = &*s.elem {
					let resolved = self.resolve_path(&p.path);
					assert!(self.is_primitive(&resolved));
					write!(w, "c_").unwrap();
				} else { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}
	pub fn print_to_c_conversion_inline_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		self.print_to_c_conversion_inline_prefix_intern(w, t, generics, false, ptr_for_ref);
	}
	pub fn print_to_c_conversion_inline_suffix_intern<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool) {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_to_c_conversion_inline_suffix_intern(w, &*r.elem, generics, true, ptr_for_ref);
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}
				let ident = single_ident_generic_path_to_ident(&p.path).unwrap();

				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0));
						if let Some(c_type) = self.to_c_conversion_inline_suffix_from_path(&resolved.0, is_ref) {
							write!(w, "{}", c_type).unwrap();
							return;
						} else { unimplemented!(); }
					}
				}

				if let Some(c_type) = self.to_c_conversion_inline_suffix_from_path(&self.resolve_path(&p.path), is_ref) {
					write!(w, "{}", c_type).unwrap();
				} else if let Some(_) = self.imports.get(ident) {
					// to_c_conversion_inline_suffix_from_path has to have succeeded:
					unimplemented!();
				} else if let Some(decltype) = self.declared.get(ident) {
					match decltype {
						DeclType::MirroredEnum => write!(w, ")").unwrap(),
						DeclType::StructImported(_) if is_ref && ptr_for_ref => write!(w, " }} ))").unwrap(),
						DeclType::StructImported(_) if is_ref => write!(w, " }}").unwrap(),
						DeclType::StructImported(_) if !is_ref => write!(w, ")) }}").unwrap(),
						_ => {},
					}
				} else { unimplemented!(); }
			},
			syn::Type::Array(_) => {
				// We assume all arrays contain only primitive types.
				// This may result in some outputs not compiling.
				if !is_ref {
					write!(w, " }}").unwrap();
				}
			},
			syn::Type::Slice(s) => {
				if let syn::Type::Path(p) = &*s.elem {
					let resolved = self.resolve_path(&p.path);
					assert!(self.is_primitive(&resolved));
				} else { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}
	pub fn print_to_c_conversion_inline_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		self.print_to_c_conversion_inline_suffix_intern(w, t, generics, false, ptr_for_ref);
	}
	pub fn print_to_c_conversion_new_var<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_to_c_conversion_new_var(w, ident, &*r.elem, generics)
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}
				let ty_ident = assert_single_path_seg(&p.path);

				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0));
						if let Some((prefix, suffix)) = self.to_c_conversion_new_var_from_path(&resolved.0) {
							write!(w, "let c_{} = {}{}{};", ident, prefix, ident, suffix).unwrap();
							return true;
						} else { return false; }
					}
				}

				if let Some((prefix, suffix)) = self.to_c_conversion_new_var_from_path(&self.resolve_path(&p.path)) {
					write!(w, "let c_{} = {}{}{};", ident, prefix, ident, suffix).unwrap();
					true
				} else if let Some(_) = self.imports.get(ty_ident) {
					// to_c_conversion_new_var_from_path, above should have handled this
					false
				} else if self.declared.get(ty_ident).is_some() {
					false
				} else { unimplemented!(); }
			},
			syn::Type::Array(_) => {
				// We assume all arrays contain only primitive types.
				// This may result in some outputs not compiling.
				false
			},
			syn::Type::Slice(s) => {
				if let syn::Type::Path(p) = &*s.elem {
					let resolved = self.resolve_path(&p.path);
					assert!(self.is_primitive(&resolved));
					write!(w, "let c_{} = crate::c_types::{}slice::from_slice({});", ident, resolved, ident).unwrap();
					true
				} else { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}

	pub fn print_from_c_conversion_new_var<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_from_c_conversion_new_var(w, ident, &*r.elem, generics)
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}

				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0));
						if let Some(rust_type) = self.from_c_conversion_new_var_from_path_prefix(&resolved.0) {
							write!(w, "let rust_{} = {}{});", ident, rust_type, ident).unwrap();
							return true;
						} else { return false; }
					}
				}

				let resolved_path = self.resolve_path(&p.path);
				if self.is_known_container(&resolved_path) {
					let container_create = self.from_c_conversion_new_var_from_path_prefix(&resolved_path).unwrap();
					write!(w, "let rust_{} = {}{});", ident, container_create, ident).unwrap();
					if let syn::PathArguments::AngleBracketed(args) = &p.path.segments.iter().next().unwrap().arguments {
						if args.args.len() != 1 { unimplemented!(); }
						if let syn::GenericArgument::Type(t) = args.args.iter().next().unwrap() {
							// We can't requre a conversion for the inner type
							assert!(!self.print_from_c_conversion_new_var(w, ident, t, generics));
						} else { unimplemented!(); }
					} else { unimplemented!(); }
					true
				} else {
					let ty_ident = single_ident_generic_path_to_ident(&p.path).unwrap();
					if let Some(full_path) = self.imports.get(ty_ident) {
						if let Some(rust_type) = self.from_c_conversion_new_var_from_path_prefix(&full_path) {
							write!(w, "let rust_{} = {}{});", ident, rust_type, ident).unwrap();
							true
						} else { false }
					} else if self.declared.get(ty_ident).is_some() {
						false
					} else { unimplemented!(); }
				}
			},
			syn::Type::Array(_) => {
				// We assume all arrays contain only primitive types.
				// This may result in some outputs not compiling.
				false
			},
			_ => unimplemented!(),
		}
	}

	pub fn print_from_c_conversion_prefix_intern<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool) {
//eprintln!("printing from c type {:?}", t);
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_from_c_conversion_prefix_intern(w, &*r.elem, generics, true)
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}

				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0));
						if let Some(c_type) = self.from_c_conversion_prefix_from_path(&resolved.0, is_ref) {
							write!(w, "{}", c_type).unwrap();
							return;
						} else if let Some(decl_type) = self.declared.get(&resolved.1) {
							match decl_type {
								DeclType::StructImported(_) if !is_ref => write!(w, "*unsafe {{ Box::from_raw(").unwrap(),
								DeclType::StructImported(_) => {},
								DeclType::EnumIgnored => unimplemented!(),
								DeclType::MirroredEnum => {},
								DeclType::Trait(_) => {},
							}
							return;
						} else { unimplemented!(); }
					}
				}

				let resolved_path = self.resolve_path(&p.path);
				if self.is_known_container(&resolved_path) {
					write!(w, "rust_").unwrap();
				} else {
					let ident = single_ident_generic_path_to_ident(&p.path).unwrap();
					if let Some(full_path) = self.imports.get(ident) {
						if let Some(c_type) = self.from_c_conversion_prefix_from_path(&full_path, is_ref) {
							write!(w, "{}", c_type).unwrap();
						} else { unimplemented!(); }
					} else if let Some(decl_type) = self.declared.get(ident) {
						match decl_type {
							DeclType::StructImported(_) if !is_ref => write!(w, "*unsafe {{ Box::from_raw(").unwrap(),
							DeclType::StructImported(_) => {},
							DeclType::MirroredEnum => {},
							_ => unimplemented!(),
						}
					} else { unimplemented!(); }
				}
			},
			syn::Type::Array(_) => {
				// We assume all arrays contain only primitive types.
				// This may result in some outputs not compiling.
				if is_ref {
					write!(w, "unsafe {{ &*").unwrap();
				}
			},
			_ => unimplemented!(),
		}
	}
	pub fn print_from_c_conversion_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_from_c_conversion_prefix_intern(w, t, generics, false);
	}

	pub fn print_from_c_conversion_suffix_intern<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool) {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_from_c_conversion_suffix_intern(w, &*r.elem, generics, true)
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}

				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0));
						if let Some(c_type) = self.from_c_conversion_suffix_from_path(&resolved.0) {
							write!(w, "{}", c_type).unwrap();
							return;
						} else if let Some(decl_type) = self.declared.get(&resolved.1) {
							match decl_type {
								DeclType::StructImported(_) if !is_ref => write!(w, ".inner) }}").unwrap(),
								DeclType::StructImported(_) => {},
								DeclType::EnumIgnored => unimplemented!(),
								DeclType::MirroredEnum => {},
								DeclType::Trait(_) => {},
							}
							return;
						} else { unimplemented!(); }
					}
				}

				let resolved_path = self.resolve_path(&p.path);
				if self.is_known_container(&resolved_path) {
					write!(w, "").unwrap();
				} else {
					let ident = single_ident_generic_path_to_ident(&p.path).unwrap();
					if let Some(full_path) = self.imports.get(ident) {
						if let Some(c_type) = self.from_c_conversion_suffix_from_path(&full_path) {
							write!(w, "{}", c_type).unwrap();
						} else { unimplemented!(); }
					} else if let Some(decl_type) = self.declared.get(ident) {
						match decl_type {
							DeclType::StructImported(_) if !is_ref => write!(w, ".inner as *mut _) }}").unwrap(),
							DeclType::StructImported(_) => {},
							DeclType::MirroredEnum => write!(w, ".to_ln()").unwrap(),
							_ => unimplemented!(),
						}
					} else { unimplemented!(); }
				}
			},
			syn::Type::Array(_) => {
				// We assume all arrays contain only primitive types.
				// This may result in some outputs not compiling.
				if is_ref {
					write!(w, "}}").unwrap();
				} else {
					write!(w, ".data").unwrap();
				}
			},
			_ => unimplemented!(),
		}
	}
	pub fn print_from_c_conversion_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_from_c_conversion_suffix_intern(w, t, generics, false);
	}

	fn print_c_ident_intern<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
		let full_path = match self.maybe_resolve_path(&syn::Path::from(ident.clone())) {
			Some(path) => path, None => return false };
		if let Some(c_type) = self.c_type_from_path(&full_path, is_ref) {
			write!(w, "{}", c_type).unwrap();
			true
		} else if let Some(decl_type) = self.declared.get(ident) {
			if *decl_type == DeclType::MirroredEnum && is_ref {
				return false;
			}
			if let DeclType::StructImported(name) = decl_type {
				if is_mut { unimplemented!(); }
				if is_ref && ptr_for_ref {
					write!(w, "*const {}", name).unwrap();
				} else if is_ref {
					write!(w, "&{}", name).unwrap();
				} else {
					write!(w, "{}", name).unwrap();
				}
			} else {
				if is_ref && ptr_for_ref {
					write!(w, "*{}", if !is_mut {"const"} else {""}).unwrap();
				} else if is_ref {
					write!(w, "&").unwrap();
				}
				if is_ref && is_mut {
					write!(w, "mut {}", ident).unwrap();
				} else if is_ref {
					write!(w, " {}", ident).unwrap();
				} else {
					write!(w, "{}", ident).unwrap();
				}
			}
			true
		} else { false }
	}
	fn print_c_type_intern<W: std::io::Write>(&self, generics: Option<&GenericTypes>, w: &mut W, t: &syn::Type, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
		match t {
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					return false;
				}
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						if self.is_known_container(&resolved.0) { return false; }
						return self.print_c_ident_intern(w, &resolved.1, is_ref, is_mut, ptr_for_ref);
					}
				}
				if let Some(full_path) = self.maybe_resolve_path(&p.path) {
					if self.is_known_container(&full_path) {
						if let syn::PathArguments::AngleBracketed(args) = &p.path.segments.iter().next().unwrap().arguments {
							if args.args.len() != 1 { return false; }
							if let syn::GenericArgument::Type(t) = args.args.iter().next().unwrap() {
								return self.print_c_type_intern(generics, w, t, false, false, ptr_for_ref);
							} else { return false; }
						} else { return false; }
					}
				}
				if p.path.leading_colon.is_some() { return false; }
				if let Some(ident) = single_ident_generic_path_to_ident(&p.path) {
					self.print_c_ident_intern(w, &ident, is_ref, is_mut, ptr_for_ref)
				} else { false }
			},
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { return false; }
				self.print_c_type_intern(generics, w, &*r.elem, true, r.mutability.is_some(), ptr_for_ref)
			},
			syn::Type::Array(a) => {
				if is_ref && is_mut {
					write!(w, "*mut [").unwrap();
					if !self.print_c_type_intern(generics, w, &a.elem, false, false, ptr_for_ref) { return false; }
				} else if is_ref {
					write!(w, "*const [").unwrap();
					if !self.print_c_type_intern(generics, w, &a.elem, false, false, ptr_for_ref) { return false; }
				} else {
					write!(w, "crate::c_types::ThirtyTwoBytes").unwrap();
					let mut typecheck = Vec::new();
					if !self.print_c_type_intern(generics, &mut typecheck, &a.elem, false, false, ptr_for_ref) { return false; }
					if typecheck[..] != ['u' as u8, '8' as u8] { return false; }
				}
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						if !is_ref {
							if i.base10_digits() != "32" {
								false
							} else { true }
						} else {
							write!(w, "; {}]", i).unwrap();
							true
						}
					} else { false }
				} else { false }
			}
			syn::Type::Slice(s) => {
				if !is_ref || is_mut { return false; }
				if let syn::Type::Path(p) = &*s.elem {
					let resolved = self.resolve_path(&p.path);
					if !self.is_primitive(&resolved) { return false; }
					write!(w, "crate::c_types::{}slice", resolved).unwrap();
					true
				} else { false }
			},
			_ => false,
		}
	}
	pub fn print_c_type<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		assert!(self.print_c_type_intern(generics, w, t, false, false, ptr_for_ref));
	}
	pub fn understood_c_path(&self, p: &syn::Path) -> bool {
		if p.leading_colon.is_some() { return false; }
		if let Some(ident) = single_ident_generic_path_to_ident(p) {
			self.print_c_ident_intern(&mut std::io::sink(), ident, false, false, false)
		} else { false }
	}
	pub fn understood_c_type(&self, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		self.print_c_type_intern(generics, &mut std::io::sink(), t, false, false, false)
	}
}
