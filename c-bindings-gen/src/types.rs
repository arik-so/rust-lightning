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
						let mut iter = g.stream().into_iter();
						if let TokenTree::Ident(i) = iter.next().unwrap() {
							if i == "any" {
								// #[cfg(any(test, feature = ""))]
								if let TokenTree::Group(g) = iter.next().unwrap() {
									if let TokenTree::Ident(i) = g.stream().into_iter().next().unwrap() {
										if i == "test" || i == "feature" {
											// If its cfg(feature(...)) we assume its test-only
											return ExportStatus::TestOnly;
										}
									}
								}
							} else if i == "test" || i == "feature" {
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
	StructIgnored,
	EnumIgnored,
}

pub struct CrateTypes<'a> {
	pub structs: HashMap<String, &'a syn::ItemStruct>,
	pub traits: HashMap<String, &'a syn::ItemTrait>,
	pub trait_impls: HashMap<String, Vec<&'a syn::Ident>>,
}

pub struct TypeResolver<'a> {
	pub orig_crate: &'a str,
	pub module_path: &'a str,
	imports: HashMap<syn::Ident, String>,
	// ident -> is-mirrored-enum
	declared: HashMap<syn::Ident, DeclType<'a>>,
	pub crate_types: &'a CrateTypes<'a>,
}

impl<'a> TypeResolver<'a> {
	pub fn new(orig_crate: &'a str, module_path: &'a str, crate_types: &'a CrateTypes<'a>) -> Self {
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
		imports.insert(syn::Ident::new("Vec", Span::call_site()), "Vec".to_string());
		imports.insert(syn::Ident::new("Option", Span::call_site()), "Option".to_string());
		Self { orig_crate, module_path, imports, declared: HashMap::new(), crate_types }
	}

	// *************************************************
	// *** Well know type and conversion definitions ***
	// *************************************************

	/// Returns true we if can just skip passing this to C entirely
	fn skip_path(&self, full_path: &str) -> bool {
		full_path == "bitcoin::secp256k1::Secp256k1"
	}
	/// Returns true we if can just skip passing this to C entirely
	fn no_arg_path_to_rust(&self, full_path: &str) -> &str {
		if full_path == "bitcoin::secp256k1::Secp256k1" {
			"&bitcoin::secp256k1::Secp256k1::new()"
		} else { unimplemented!(); }
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
eprintln!("ctfp: {}", full_path);
		match full_path {
			"Result" => Some("crate::c_types::CResult"),
			"Vec" if !is_ref => Some("crate::c_types::CVec"),
			"Option" => Some(""),

			"[u8; 32]" if !is_ref => Some("crate::c_types::ThirtyTwoBytes"),
			"[u8; 3]" if !is_ref => Some("crate::c_types::ThreeBytes"), // Used for RGB values

			"bitcoin::secp256k1::key::PublicKey" => Some("crate::c_types::PublicKey"),
			"bitcoin::secp256k1::Signature" => Some("crate::c_types::Signature"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some("crate::c_types::SecretKey"),
			"bitcoin::blockdata::script::Script" => Some("crate::c_types::Script"),
			"bitcoin::blockdata::transaction::Transaction" => Some("crate::c_types::Transaction"),
			"bitcoin::blockdata::transaction::OutPoint" => Some("crate::chain::transaction::OutPoint"),
			"bitcoin::OutPoint" => Some("crate::chain::transaction::OutPoint"),
			"bitcoin::network::constants::Network" => Some("crate::bitcoin::network::Network"),
			"bitcoin::blockdata::block::BlockHeader" if is_ref  => Some("*const [u8; 80]"),
			"bitcoin::blockdata::block::BlockHeader" if !is_ref => Some("[u8; 80]"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::hash_types::Txid" if !is_ref => Some("[u8; 32]"),
			"bitcoin::hash_types::BlockHash" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::hash_types::BlockHash" if !is_ref => Some("[u8; 32]"),
			"ln::channelmanager::PaymentHash" if is_ref => Some("*const [u8; 32]"),
			"ln::channelmanager::PaymentHash" if !is_ref => Some("[u8; 32]"),
			"ln::channelmanager::PaymentPreimage" if is_ref => Some("*const [u8; 32]"),
			"ln::channelmanager::PaymentPreimage" if !is_ref => Some("[u8; 32]"),
			"ln::channelmanager::PaymentSecret" if is_ref => Some("*const [u8; 32]"),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some("[u8; 32]"),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some("*const std::os::raw::c_char"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some("*const crate::ln::features::InitFeatures"),
			"ln::features::InitFeatures" => Some("crate::ln::features::InitFeatures"),
			"util::config::UserConfig" if !is_ref => Some("crate::util::config::UserConfig"),
			"util::errors::APIError" if !is_ref => Some("crate::util::errors::APIError"),
			"chain::transaction::OutPoint" => Some("crate::chain::transaction::OutPoint"),
			"ln::msgs::NetAddress" => Some("crate::ln::msgs::NetAddress"),

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
				eprintln!("    Type {} (ref: {}) unresolvable in C", full_path, is_ref);
				None
			},
		}
	}

	fn is_transparent_container(&self, full_path: &str) -> bool {
		full_path == "Option"
	}
	fn is_known_container(&self, full_path: &str) -> bool {
		full_path == "Result" || full_path == "Vec"
	}
	fn to_c_conversion_container_new_var<'b>(&self, full_path: &str, is_ref: bool, _args: syn::punctuated::Iter<syn::GenericArgument>, var_name: &syn::Ident)
			// Returns prefix + Vec<(prefix, var-name-to-inline-convert)> + suffix
			// expecting one element in the vec per generic type, each of which is inline-converted
			-> Option<(&'b str, Vec<(String, String)>, &'b str)> {
		match full_path {
			"Result" if !is_ref => {
				Some(("match ",
						vec![("{ Ok(o) => crate::c_types::CResultTempl::good(".to_string(), "o".to_string()),
						     ("), Err(e) => crate::c_types::CResultTempl::err(".to_string(), "e".to_string())],
						") }"))
			},
			"Vec" if !is_ref => {
				Some(("Vec::new(); for item in ", vec![(format!(".drain(..) {{ local_{}.push(", var_name), "item".to_string())], "); }"))
			},
			"Option" if is_ref =>
				Some(("if ", vec![(".is_none() { return std::ptr::null(); } else { ".to_string(), format!("(*{}.as_ref().unwrap())", var_name))], " }")),
			_ => None,
		}
	}

	fn from_c_conversion_container_new_var<'b>(&self, full_path: &str, is_ref: bool, mut args: syn::punctuated::Iter<syn::GenericArgument>, var_name: &syn::Ident)
			// Returns prefix + Vec<(prefix, var-name-to-inline-convert)> + suffix
			// expecting one element in the vec per generic type, each of which is inline-converted
			-> Option<(&'b str, Vec<(String, String)>, &'b str)> {
eprintln!("fccc: {:?}", full_path);
		match full_path {
			"Result" if !is_ref => {
				Some(("match ",
						vec![(".result_good { true => Ok(".to_string(), format!("(*unsafe {{ &*{}.contents.result }})", var_name)),
						     ("), false => Err(".to_string(), format!("(*unsafe {{ &*{}.contents.err }})", var_name))],
						")}"))
			},
			"Vec" if !is_ref => {
				Some(("Vec::new(); for item in ", vec![(format!(".into_rust().drain(..) {{ local_{}.push(", var_name), "item".to_string())], "); }"))
			},
			"Option" if is_ref => {
				if let syn::GenericArgument::Type(syn::Type::Path(pa)) = args.next().unwrap() {
					let arg = self.resolve_path(&pa.path);
					Some(("if ", vec![(".is_null() { None } else { Some(".to_string(), format!("{}", var_name))], ") }"))
				} else { unimplemented!(); }
			},
			"Option" => {
				if let syn::GenericArgument::Type(syn::Type::Path(pa)) = args.next().unwrap() {
					let arg = self.resolve_path(&pa.path);
					Some(("if ", vec![(".inner.is_null() { None } else { Some(".to_string(), format!("{}", var_name))], ") }"))
				} else { unimplemented!(); }
			},
			_ => None,
		}
	}

	fn from_c_conversion_new_var_from_path<'b>(&self, _full_path: &str, _is_ref: bool) -> Option<(&'b str, &'b str)> {
		None
	}
	fn from_c_conversion_prefix_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<&'b str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"Vec" if !is_ref => Some("local_"),
			"Result" if !is_ref => Some("local_"),
			"Option" if is_ref => Some("&local_"),
			"Option" => Some("local_"),

			"[u8; 32]" if is_ref => Some("unsafe { &*"),
			"[u8; 32]" if !is_ref => Some(""),
			"[u8; 3]" if !is_ref => Some(""),

			"bitcoin::secp256k1::key::PublicKey" if is_ref => Some("&"),
			"bitcoin::secp256k1::key::PublicKey" => Some(""),
			"bitcoin::secp256k1::Signature" if is_ref => Some("&"),
			"bitcoin::secp256k1::Signature" => Some(""),
			"bitcoin::secp256k1::key::SecretKey" if is_ref => Some("&::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *"),
			"bitcoin::secp256k1::key::SecretKey" => Some(""),
			"bitcoin::blockdata::script::Script" if is_ref => Some("&"),
			"bitcoin::blockdata::script::Script" => Some(""),
			"bitcoin::blockdata::transaction::Transaction" if is_ref => Some("&"),
			"bitcoin::blockdata::transaction::Transaction" => Some(""),
			"bitcoin::network::constants::Network" => Some(""),
			"bitcoin::blockdata::block::BlockHeader" => Some("::bitcoin::consensus::encode::deserialize(&"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref => Some("&::bitcoin::hash_types::Txid::from_slice(&unsafe { &*"),
			"bitcoin::hash_types::Txid" if !is_ref => Some("::bitcoin::hash_types::Txid::from_slice(&"),
			"bitcoin::hash_types::BlockHash" => Some("::bitcoin::hash_types::BlockHash::from_slice(&"),
			"ln::channelmanager::PaymentHash" if !is_ref => Some("::lightning::ln::channelmanager::PaymentHash("),
			"ln::channelmanager::PaymentHash" if is_ref => Some("&::lightning::ln::channelmanager::PaymentHash(unsafe { *"),
			"ln::channelmanager::PaymentPreimage" if !is_ref => Some("::lightning::ln::channelmanager::PaymentPreimage("),
			"ln::channelmanager::PaymentPreimage" if is_ref => Some("&::lightning::ln::channelmanager::PaymentPreimage(unsafe { *"),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some("::lightning::ln::channelmanager::PaymentSecret("),
			"ln::channelmanager::PaymentSecret" if is_ref=> Some("::lightning::ln::channelmanager::PaymentSecret(unsafe { *"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if !is_ref => Some("*unsafe { Box::from_raw("),
			"util::config::UserConfig" if !is_ref => Some("*unsafe { Box::from_raw("),
			"chain::transaction::OutPoint" if !is_ref => Some("*unsafe { Box::from_raw("),
			"ln::msgs::NetAddress" if !is_ref => Some("*unsafe { Box::from_raw("),

			// List of traits we map (possibly during processing of other files):
			"crate::util::logger::Logger" => Some(""),
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
	fn from_c_conversion_suffix_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<&'b str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"Vec" if !is_ref => Some(""),
			"Option" => Some(""),
			"Result" if !is_ref => Some(""),

			"[u8; 32]" if is_ref => Some("}"),
			"[u8; 32]" if !is_ref => Some(".data"),
			"[u8; 3]" if !is_ref => Some(".data"),

			"bitcoin::secp256k1::key::PublicKey" => Some(".into_rust()"),
			"bitcoin::secp256k1::Signature" => Some(".into_rust()"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some(".into_rust()"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref => Some("}[..]).unwrap()"),
			"bitcoin::blockdata::script::Script" => Some(".into_bitcoin()"),
			"bitcoin::blockdata::transaction::Transaction" => Some(".into_bitcoin()"),
			"bitcoin::network::constants::Network" => Some(".into_bitcoin()"),
			"bitcoin::blockdata::block::BlockHeader" => Some("[..]).unwrap()"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref => Some(" }[..]).unwrap()"),
			"bitcoin::hash_types::Txid" => Some("[..]).unwrap()"),
			"bitcoin::hash_types::BlockHash" => Some("[..]).unwrap()"),
			"ln::channelmanager::PaymentHash" if !is_ref => Some(")"),
			"ln::channelmanager::PaymentHash" if is_ref => Some(" })"),
			"ln::channelmanager::PaymentPreimage" if !is_ref => Some(")"),
			"ln::channelmanager::PaymentPreimage" if is_ref => Some(" })"),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some(")"),
			"ln::channelmanager::PaymentSecret" if is_ref=> Some(" })"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" => Some(".inner as *mut _) }"),
			"util::config::UserConfig" => Some(".inner as *mut lightning::util::config::UserConfig) }"),
			"chain::transaction::OutPoint" if !is_ref => Some(".inner as *mut _) }"),
			"ln::msgs::NetAddress" if !is_ref => Some(".inner as *mut _) }"),

			// List of traits we map (possibly during processing of other files):
			"crate::util::logger::Logger" => Some(""),
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

	fn to_c_conversion_new_var_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<(&'b str, &'b str)> {
		if self.is_primitive(full_path) {
			return None;
		}
		match full_path {
			"[u8]" if is_ref => Some(("crate::c_types::u8slice::from_slice(", ")")),

			"bitcoin::blockdata::transaction::Transaction" if is_ref => Some(("::bitcoin::consensus::encode::serialize(", ")")),
			"bitcoin::hash_types::Txid" => None,

			// Override the default since Records contain an fmt with a lifetime:
			// TODO: We should include the other record fields
			"util::logger::Record" => Some(("std::ffi::CString::new(format!(\"{}\", ", ".args)).unwrap()")),
			_ => None,
		}
	}
	fn to_c_conversion_inline_prefix_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<&'b str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"Result" if !is_ref => Some("local_"),
			"Vec" if !is_ref => Some("local_"),
			"Option" => Some("local_"),

			"[u8; 32]" if !is_ref => Some("crate::c_types::ThirtyTwoBytes { data: "),
			"[u8; 32]" if is_ref => Some("&"),
			"[u8]" if is_ref => Some("local_"),

			"bitcoin::secp256k1::key::PublicKey" => Some("crate::c_types::PublicKey::from_rust(&"),
			"bitcoin::secp256k1::Signature" => Some("crate::c_types::Signature::from_rust(&"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref  => unimplemented!(),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some("crate::c_types::SecretKey::from_rust("),
			"bitcoin::blockdata::script::Script" if is_ref => Some("crate::c_types::Script::from_bitcoin(&"),
			"bitcoin::blockdata::transaction::Transaction" if is_ref => Some("crate::c_types::Transaction::from_slice(&local_"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" => Some(""),
			"ln::channelmanager::PaymentHash" if is_ref => Some("&"),
			"ln::channelmanager::PaymentHash" => Some(""),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some("local_"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some("Box::into_raw(Box::new(crate::ln::features::InitFeatures { inner: &"),
			"ln::features::InitFeatures" if !is_ref => Some("crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new("),
			"ln::msgs::NetAddress" if !is_ref => Some("crate::ln::msgs::NetAddress { inner: Box::into_raw(Box::new("),

			"util::errors::APIError" if !is_ref => Some("crate::util::errors::APIError { inner: Box::into_raw(Box::new("),

			// List of traits we map (possibly during processing of other files):
			"crate::ln::msgs::ChannelMessageHandler" if is_ref => Some("&"),
			"crate::ln::msgs::RoutingMessageHandler" if is_ref => Some("&"),
			_ => {
				eprintln!("    Type {} (is_ref: {}) unconvertable to C", full_path, is_ref);
				None
			},
		}
	}
	fn to_c_conversion_inline_suffix_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<&'b str> {
		if self.is_primitive(full_path) {
			return Some("");
		}
		match full_path {
			"Result" if !is_ref => Some(""),
			"Vec" if !is_ref => Some(".into()"),
			"Option" => Some(""),

			"[u8; 32]" if !is_ref => Some(" }"),
			"[u8; 32]" if is_ref => Some(""),
			"[u8]" if is_ref => Some(""),

			"bitcoin::secp256k1::key::PublicKey" => Some(")"),
			"bitcoin::secp256k1::Signature" => Some(")"),
			"bitcoin::secp256k1::key::SecretKey" => Some(")"),
			"bitcoin::blockdata::script::Script" => Some(")"),
			"bitcoin::blockdata::transaction::Transaction" => Some(")"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if !is_ref => Some(".into_inner()"),
			"bitcoin::hash_types::Txid" => Some(".as_inner()"),
			"ln::channelmanager::PaymentHash" => Some(".0"),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some(".as_ptr()"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some(" }))"),
			"ln::features::InitFeatures" => Some(")) }"),
			"ln::msgs::NetAddress" if !is_ref => Some(")) }"),

			"util::errors::APIError" if !is_ref => Some(")) }"),

			// List of traits we map (possibly during processing of other files):
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable to C", full_path);
				None
			},
		}
	}

	// *************************************************
	// *** Type definition during main.rs processing ***
	// *************************************************

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
	pub fn struct_ignored(&mut self, ident: &syn::Ident) {
		eprintln!("Not importing {}", ident);
		self.declared.insert(ident.clone(), DeclType::StructIgnored);
	}
	pub fn trait_declared(&mut self, ident: &syn::Ident, t: &'a syn::ItemTrait) {
		eprintln!("Trait {} created", ident);
		self.declared.insert(ident.clone(), DeclType::Trait(t));
	}
	pub fn get_declared_type(&'a self, ident: &syn::Ident) -> Option<&'a DeclType> {
		self.declared.get(ident)
	}

	pub fn maybe_resolve_ident(&self, id: &syn::Ident) -> Option<String> {
		if let Some(imp) = self.imports.get(id) {
			Some(imp.clone())
		} else if self.declared.get(id).is_some() {
			Some(self.module_path.to_string() + "::" + &format!("{}", id))
		} else { None }
	}

	pub fn maybe_resolve_non_ignored_ident(&self, id: &syn::Ident) -> Option<String> {
		if let Some(imp) = self.imports.get(id) {
			Some(imp.clone())
		} else if let Some(decl_type) = self.declared.get(id) {
			match decl_type {
				DeclType::StructIgnored => None,
				_ => Some(self.module_path.to_string() + "::" + &format!("{}", id)),
			}
		} else { None }
	}

	pub fn maybe_resolve_path(&self, p: &syn::Path) -> Option<String> {
		if p.leading_colon.is_some() {
			//format!("{}", p.segments);
			return None;
		} else if let Some(id) = p.get_ident() {
			self.maybe_resolve_ident(id)
		} else {
			if p.segments.len() == 1 {
				let seg = p.segments.iter().next().unwrap();
				return self.maybe_resolve_ident(&seg.ident);
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

	// ***********************************
	// *** Original Rust Type Printing ***
	// ***********************************

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

	// ********************************
	// *** Type conversion printing ***
	// ********************************

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
	pub fn no_arg_to_rust<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		match t {
			syn::Type::Path(p) => {
				if p.qself.is_some() { unimplemented!(); }
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						write!(w, "{}", self.no_arg_path_to_rust(resolved.0)).unwrap();
						return;
					}
				}
				if let Some(full_path) = self.maybe_resolve_path(&p.path) {
					write!(w, "{}", self.no_arg_path_to_rust(&full_path)).unwrap();
				}
			},
			syn::Type::Reference(r) => {
				self.no_arg_to_rust(w, &*r.elem, generics);
			},
			_ => {},
		}
	}

	fn print_conversion_inline_intern<W: std::io::Write, LP: Fn(&str, bool) -> Option<&str>, DL: Fn(&mut W, &DeclType, &syn::Ident, bool)>
			(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool, prefix: bool,
			 path_lookup: LP, decl_lookup: DL) {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_conversion_inline_intern(w, &*r.elem, generics, true, ptr_for_ref, prefix, path_lookup, decl_lookup);
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}

				if let Some(gen_types) = generics {
					if let Some((path, ident)) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&path) && !self.is_transparent_container(&path));
						if let Some(c_type) = path_lookup(&path, is_ref) {
							write!(w, "{}", c_type).unwrap();
							return;
						} else if let Some(decl_type) = self.declared.get(ident) {
							decl_lookup(w, decl_type, ident, is_ref);
							return;
						} else { unimplemented!(); }
					}
				}

				let resolved_path = self.resolve_path(&p.path);
				if let Some(c_type) = path_lookup(&resolved_path, is_ref) {
					write!(w, "{}", c_type).unwrap();
				} else if let Some(stuct) = self.crate_types.structs.get(&resolved_path) {
					decl_lookup(w, &DeclType::StructImported(format!("crate::{}", resolved_path)), &stuct.ident, is_ref);
				} else if let Some(ident) = single_ident_generic_path_to_ident(&p.path) {
					if let Some(_) = self.imports.get(ident) {
						// prefix_lookup has to have succeeded:
						unimplemented!();
					} else if let Some(decl_type) = self.declared.get(ident) {
						decl_lookup(w, decl_type, ident, is_ref);
					} else { unimplemented!(); }
				}
			},
			syn::Type::Array(a) => {
				// We assume all arrays contain only [u8; X]s.
				// This may result in some outputs not compiling.
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						write!(w, "{}", path_lookup(&format!("[u8; {}]", i.base10_digits()), is_ref).unwrap()).unwrap();
					} else { unimplemented!(); }
				} else { unimplemented!(); }
			},
			syn::Type::Slice(s) => {
				// We assume all slices contain only u8s.
				// This may result in some outputs not compiling.
				if let syn::Type::Path(p) = &*s.elem {
					let resolved = self.resolve_path(&p.path);
					assert!(self.is_primitive(&resolved));
					write!(w, "{}", path_lookup("[u8]", is_ref).unwrap()).unwrap();
				} else { unimplemented!(); }
			},
			syn::Type::Tuple(t) => {
				if t.elems.len() != 0 { unimplemented!(); }
				// cbindgen has poor support for (), see, eg https://github.com/eqrion/cbindgen/issues/527
				// so work around it by just pretending its a 0u8
				if prefix {
					write!(w, "0u8 /*").unwrap();
				} else {
					write!(w, "*/").unwrap();
				}
			},
			_ => unimplemented!(),
		}
	}

	fn print_to_c_conversion_inline_prefix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, ptr_for_ref, true,
				|a, b| self.to_c_conversion_inline_prefix_from_path(a, b),
				|w, decl_type, ident, is_ref| {
					let decl_path = self.maybe_resolve_ident(ident).unwrap();
					match decl_type {
						DeclType::MirroredEnum => write!(w, "crate::{}::from_ln(", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported(_) if is_ref && ptr_for_ref =>
							write!(w, "Box::into_raw(Box::new(crate::{} {{ inner: &", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported(_) if is_ref =>
							write!(w, "&crate::{} {{ inner: ", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported(_) if !is_ref =>
							write!(w, "crate::{} {{ inner: Box::into_raw(Box::new(", decl_path).unwrap(),
						_ => unimplemented!(),
					}
				});
	}
	pub fn print_to_c_conversion_inline_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		self.print_to_c_conversion_inline_prefix_inner(w, t, generics, false, ptr_for_ref);
	}
	pub fn print_to_c_conversion_inline_suffix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, ptr_for_ref, false,
				|a, b| self.to_c_conversion_inline_suffix_from_path(a, b),
				|w, decl_type, _ident, is_ref| match decl_type {
					DeclType::MirroredEnum => write!(w, ")").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported(_) if is_ref && ptr_for_ref => write!(w, " }} ))").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported(_) if is_ref => write!(w, " }}").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported(_) if !is_ref => write!(w, ")) }}").unwrap(),
					_ => unimplemented!(),
				});
	}
	pub fn print_to_c_conversion_inline_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		self.print_to_c_conversion_inline_suffix_inner(w, t, generics, false, ptr_for_ref);
	}

	pub fn print_from_c_conversion_prefix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, false, true,
				|a, b| self.from_c_conversion_prefix_from_path(a, b),
				|w, decl_type, _ident, is_ref| match decl_type {
					DeclType::StructImported(_) if !is_ref => write!(w, "*unsafe {{ Box::from_raw(").unwrap(),
					DeclType::StructImported(_) => write!(w, "unsafe {{ &*").unwrap(),
					DeclType::MirroredEnum => {},
					DeclType::Trait(_) => {},
					_ => unimplemented!(),
				});
	}
	pub fn print_from_c_conversion_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_from_c_conversion_prefix_inner(w, t, generics, false);
	}
	pub fn print_from_c_conversion_suffix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, false, false,
				|a, b| self.from_c_conversion_suffix_from_path(a, b),
				|w, decl_type, _ident, is_ref| match decl_type {
					DeclType::StructImported(_) if !is_ref => write!(w, ".inner as *mut _) }}").unwrap(),
					DeclType::StructImported(_) => write!(w, ".inner }}").unwrap(),
					DeclType::MirroredEnum => {},
					DeclType::Trait(_) => {},
					_ => unimplemented!(),
				});
	}
	pub fn print_from_c_conversion_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_from_c_conversion_suffix_inner(w, t, generics, false);
	}

	fn print_conversion_new_var_intern<'b, W: std::io::Write,
		LP: Fn(&str, bool) -> Option<(&str, &str)>,
		LC: Fn(&str, bool, syn::punctuated::Iter<syn::GenericArgument>, &syn::Ident) ->  Option<(&'b str, Vec<(String, String)>, &'b str)>,
		VP: Fn(&mut W, &syn::Type, Option<&GenericTypes>, bool),
		VS: Fn(&mut W, &syn::Type, Option<&GenericTypes>, bool)>
			(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool,
			 path_lookup: &LP, container_lookup: &LC, var_prefix: &VP, var_suffix: &VS) -> bool {
		match t {
			syn::Type::Reference(r) => {
				if r.lifetime.is_some() { unimplemented!(); }
				self.print_conversion_new_var_intern(w, ident, &*r.elem, generics, true, path_lookup, container_lookup, var_prefix, var_suffix)
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0) && !self.is_transparent_container(&resolved.0));
						if let Some((prefix, suffix)) = path_lookup(&resolved.0, is_ref) {
							write!(w, "let local_{} = {}{}{};", ident, prefix, ident, suffix).unwrap();
							return true;
						} else { return false; }
					}
				}
				let resolved_path = self.resolve_path(&p.path);
				if self.is_known_container(&resolved_path) || self.is_transparent_container(&resolved_path) {
					if let syn::PathArguments::AngleBracketed(args) = &p.path.segments.iter().next().unwrap().arguments {
						let (prefix, conversions, suffix) = container_lookup(&resolved_path, is_ref, args.args.iter(), ident).unwrap();
						write!(w, "let mut local_{} = {}{}", ident, prefix, ident).unwrap();
						assert_eq!(conversions.len(), args.args.len());
						for ((pfx, var_name), arg) in conversions.iter().zip(args.args.iter()) {
							if let syn::GenericArgument::Type(ty) = arg {
								write!(w, "{}", pfx).unwrap();
								if self.print_conversion_new_var_intern(w, &syn::Ident::new("dummy", Span::call_site()), ty, generics,
										false, path_lookup, container_lookup, var_prefix, var_suffix) {
									// Inner types in containers must be expressable using only
									// prefix and suffix conversion, not new_var conversions.
									unimplemented!();
								}
								var_prefix(w, ty, generics, is_ref);
								write!(w, "{}", var_name).unwrap();
								var_suffix(w, ty, generics, is_ref);
							} else { unimplemented!(); }
						}
						write!(w, "{};", suffix).unwrap();
					} else { unimplemented!(); }
					true
				} else if self.is_primitive(&resolved_path) {
					false
				} else if let Some(ty_ident) = single_ident_generic_path_to_ident(&p.path) {
					if let Some((prefix, suffix)) = path_lookup(&resolved_path, is_ref) {
						write!(w, "let local_{} = {}{}{};", ident, prefix, ident, suffix).unwrap();
						true
					} else if self.declared.get(ty_ident).is_some() {
						false
					} else { false }
				} else { false }
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
					let slice_path = format!("[{}]", resolved);
					if let Some((prefix, suffix)) = path_lookup(&slice_path, is_ref) {
						write!(w, "let local_{} = {}{}{};", ident, prefix, ident, suffix).unwrap();
						true
					} else { false }
				} else { unimplemented!() }
			},
			syn::Type::Tuple(t) => {
				if t.elems.len() == 0 {
					false
				} else { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}

	pub fn print_to_c_conversion_new_var<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) -> bool {
		self.print_conversion_new_var_intern(w, ident, t, generics, false,
			&|a, b| self.to_c_conversion_new_var_from_path(a, b),
			&|a, b, c, d| self.to_c_conversion_container_new_var(a, b, c, d),
			&|a, b, c, d| self.print_to_c_conversion_inline_prefix_inner(a, b, c, d, ptr_for_ref),
			&|a, b, c, d| self.print_to_c_conversion_inline_suffix_inner(a, b, c, d, ptr_for_ref))
	}
	pub fn print_from_c_conversion_new_var<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		self.print_conversion_new_var_intern(w, ident, t, generics, false,
			&|a, b| self.from_c_conversion_new_var_from_path(a, b),
			&|a, b, c, d| self.from_c_conversion_container_new_var(a, b, c, d),
			&|a, b, c, d| self.print_from_c_conversion_prefix_inner(a, b, c, d),
			&|a, b, c, d| self.print_from_c_conversion_suffix_inner(a, b, c, d))
	}

	// **********************************
	// *** C Type Equivalent Printing ***
	// **********************************

	fn print_c_ident_intern<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
		let full_path = match self.maybe_resolve_path(&syn::Path::from(ident.clone())) {
			Some(path) => path, None => format!("{}::{}", self.module_path, ident) };
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

	fn print_c_path_intern<W: std::io::Write>(&self, w: &mut W, path: &syn::Path, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
//eprintln!("pcpi: {:?}", path);
		if let Some(ident) = single_ident_generic_path_to_ident(&path) {
			if self.print_c_ident_intern(w, ident, is_ref, is_mut, ptr_for_ref) { return true; }
		}
		let full_path = match self.maybe_resolve_path(&path) {
			Some(path) => path, None => return false };
		if let Some(t) = self.crate_types.traits.get(&full_path) {
			if is_ref && ptr_for_ref {
				write!(w, "*const crate::{}", full_path).unwrap();
			} else if is_ref {
				write!(w, "&crate::{}", full_path).unwrap();
			} else {
				write!(w, "crate::{}", full_path).unwrap();
			}
			true
		} else if let Some(s) = self.crate_types.structs.get(&full_path) {
			if is_ref && ptr_for_ref {
				write!(w, "*const crate::{}", full_path).unwrap();
			} else if is_ref {
				write!(w, "&crate::{}", full_path).unwrap();
			} else {
				write!(w, "crate::{}", full_path).unwrap();
			}
			true
		} else {
			false
		}
	}
	fn print_c_type_intern<W: std::io::Write>(&self, generics: Option<&GenericTypes>, w: &mut W, t: &syn::Type, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
//eprintln!("pcti: {:?}", t);
		match t {
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					return false;
				}
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						if self.is_known_container(&resolved.0) { return false; }
						if self.is_transparent_container(&resolved.0) { return false; }
						return self.print_c_ident_intern(w, &resolved.1, is_ref, is_mut, ptr_for_ref);
					}
				}
				if let Some(full_path) = self.maybe_resolve_path(&p.path) {
					if self.is_known_container(&full_path) || self.is_transparent_container(&full_path) {
						if let syn::PathArguments::AngleBracketed(args) = &p.path.segments.iter().next().unwrap().arguments {
							if !self.print_c_path_intern(w, &p.path, is_ref, is_mut, ptr_for_ref) { return false; }
							for arg in args.args.iter() {
								if let syn::GenericArgument::Type(t) = arg {
									if let syn::Type::Tuple(tuple) = t {
										if tuple.elems.len() == 0 {
											write!(w, "None").unwrap();
										} else { return false; }
									} else if let syn::Type::Path(p_arg) = t {
										if self.is_transparent_container(&full_path) {
											// We dont (yet) support primitives inside transparent
											// containers, so check for that first:
											let resolved_generic = self.resolve_path(&p_arg.path);
											if self.is_primitive(&resolved_generic) { return false; }
											if !self.print_c_path_intern(w, &p_arg.path, is_ref, is_mut, ptr_for_ref) { return false; }
										} else if let Some(id) = p_arg.path.get_ident() {
											write!(w, "{}", id).unwrap();
										} else { return false; }
									}
								} else { return false; }
							}
							return true;
						} else { return false; }
					}
				}
				if p.path.leading_colon.is_some() { return false; }
				self.print_c_path_intern(w, &p.path, is_ref, is_mut, ptr_for_ref)
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
					let mut typecheck = Vec::new();
					if !self.print_c_type_intern(generics, &mut typecheck, &a.elem, false, false, ptr_for_ref) { return false; }
					if typecheck[..] != ['u' as u8, '8' as u8] { return false; }
				}
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						if !is_ref {
							if let Some(ty) = self.c_type_from_path(&format!("[u8; {}]", i.base10_digits()), false) {
								write!(w, "{}", ty).unwrap();
								true
							} else { false }
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
			syn::Type::Tuple(t) => {
				if t.elems.len() == 0 {
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
