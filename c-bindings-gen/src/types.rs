use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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
	typed_generics: Vec<HashMap<&'a syn::Ident, (String, Option<&'a syn::Path>)>>,
}
impl<'a> GenericTypes<'a> {
	pub fn new() -> Self {
		Self { typed_generics: vec![HashMap::new()], }
	}

	pub fn push_ctx(&mut self) {
		self.typed_generics.push(HashMap::new());
	}
	pub fn pop_ctx(&mut self) {
		self.typed_generics.pop();
	}

	pub fn learn_generics<'b, 'c>(&mut self, generics: &'a syn::Generics, types: &'b TypeResolver<'a, 'c>) -> bool {
		for generic in generics.params.iter() {
			match generic {
				syn::GenericParam::Type(type_param) => {
					let mut non_lifetimes_processed = false;
					for bound in type_param.bounds.iter() {
						if let syn::TypeParamBound::Trait(trait_bound) = bound {
							if let Some(ident) = single_ident_generic_path_to_ident(&trait_bound.path) {
								match &format!("{}", ident) as &str { "Send" => continue, "Sync" => continue, _ => {} }
							}

							assert_simple_bound(&trait_bound);
							if let Some(mut path) = types.maybe_resolve_path(&trait_bound.path) {
								if types.skip_path(&path) { continue; }
								if non_lifetimes_processed { return false; }
								non_lifetimes_processed = true;
								let new_ident = if path != "std::ops::Deref" {
									path = "crate::".to_string() + &path;
									Some(&trait_bound.path)
								} else { None };
								self.typed_generics.last_mut().unwrap().insert(&type_param.ident, (path, new_ident));
							} else { return false; }
						}
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
						if let Some(gen) = self.typed_generics.last_mut().unwrap().get_mut(&p_iter.next().unwrap().ident) {
							if gen.0 != "std::ops::Deref" { return false; }
							if &format!("{}", p_iter.next().unwrap().ident) != "Target" { return false; }

							let mut non_lifetimes_processed = false;
							for bound in t.bounds.iter() {
								if let syn::TypeParamBound::Trait(trait_bound) = bound {
									if non_lifetimes_processed { return false; }
									non_lifetimes_processed = true;
									assert_simple_bound(&trait_bound);
									*gen = ("crate::".to_string() + &types.resolve_path(&trait_bound.path),
										Some(&trait_bound.path));
								}
							}
						} else { return false; }
					} else { return false; }
				}
			}
		}
		for (_, (_, ident)) in self.typed_generics.last().unwrap().iter() {
			if ident.is_none() { return false; }
		}
		true
	}

	pub fn maybe_resolve_ident<'b>(&'b self, ident: &syn::Ident) -> Option<&'b String> {
		for gen in self.typed_generics.iter().rev() {
			if let Some(res) = gen.get(ident).map(|(a, _)| a) {
				return Some(res);
			}
		}
		None
	}
	pub fn maybe_resolve_path<'b>(&'b self, path: &syn::Path) -> Option<(&'b String, &'a syn::Path)> {
		if let Some(ident) = path.get_ident() {
			for gen in self.typed_generics.iter().rev() {
				if let Some(res) = gen.get(ident).map(|(a, b)| (a, b.unwrap())) {
					return Some(res);
				}
			}
		}
		None
	}
}

#[derive(Clone, PartialEq)]
// The type of declaration and the object itself
pub enum DeclType<'a> {
	MirroredEnum,
	Trait(&'a syn::ItemTrait),
	StructImported,
	StructIgnored,
	EnumIgnored,
}

pub struct CrateTypes<'a> {
	pub opaques: HashMap<String, &'a syn::Ident>, // Both structs and enums, but all opque
	pub mirrored_enums: HashMap<String, &'a syn::ItemEnum>,
	pub traits: HashMap<String, &'a syn::ItemTrait>,
	pub trait_impls: HashMap<String, Vec<&'a syn::Ident>>,

	/// Mangled type name -> has destructor
	pub templates_defined: HashMap<String, bool>,
	pub template_file: &'a mut File,
}

pub struct TypeResolver<'mod_lifetime, 'crate_lft: 'mod_lifetime> {
	pub orig_crate: &'mod_lifetime str,
	pub module_path: &'mod_lifetime str,
	imports: HashMap<syn::Ident, String>,
	// ident -> is-mirrored-enum
	declared: HashMap<syn::Ident, DeclType<'crate_lft>>,
	pub crate_types: &'mod_lifetime mut CrateTypes<'crate_lft>,
}

impl<'a, 'c: 'a> TypeResolver<'a, 'c> {
	pub fn new(orig_crate: &'a str, module_path: &'a str, crate_types: &'a mut CrateTypes<'c>) -> Self {
		let mut imports = HashMap::new();
		// Add primitives to the "imports" list:
		imports.insert(syn::Ident::new("bool", Span::call_site()), "bool".to_string());
		imports.insert(syn::Ident::new("u64", Span::call_site()), "u64".to_string());
		imports.insert(syn::Ident::new("u32", Span::call_site()), "u32".to_string());
		imports.insert(syn::Ident::new("u16", Span::call_site()), "u16".to_string());
		imports.insert(syn::Ident::new("u8", Span::call_site()), "u8".to_string());
		imports.insert(syn::Ident::new("usize", Span::call_site()), "usize".to_string());
		imports.insert(syn::Ident::new("str", Span::call_site()), "str".to_string());
		imports.insert(syn::Ident::new("String", Span::call_site()), "String".to_string());

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
		full_path == "bitcoin::secp256k1::Secp256k1" ||
		full_path == "bitcoin::secp256k1::Signing" ||
		full_path == "bitcoin::secp256k1::Verification"
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
	fn c_type_from_path<'b>(&self, full_path: &'b str, is_ref: bool, ptr_for_ref: bool) -> Option<&'b str> {
		if self.is_primitive(full_path) {
			return Some(full_path);
		}
//eprintln!("ctfp: {}", full_path);
		match full_path {
			"Result" => Some("crate::c_types::derived::CResult"),
			"Vec" if !is_ref => Some("crate::c_types::derived::CVec"),
			"Option" => Some(""),

			"[u8; 32]" if !is_ref => Some("crate::c_types::ThirtyTwoBytes"),
			"[u8; 16]" if !is_ref => Some("crate::c_types::SixteenBytes"),
			"[u8; 10]" if !is_ref => Some("crate::c_types::TenBytes"),
			"[u8; 4]" if !is_ref => Some("crate::c_types::FourBytes"),
			"[u8; 3]" if !is_ref => Some("crate::c_types::ThreeBytes"), // Used for RGB values

			"str" if is_ref => Some("crate::c_types::Str"),
			"String" if !is_ref => Some("crate::c_types::derived::CVec_u8Z"),

			"std::time::Duration" => Some("u64"),

			"bitcoin::secp256k1::key::PublicKey" => Some("crate::c_types::PublicKey"),
			"bitcoin::secp256k1::Signature" => Some("crate::c_types::Signature"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some("crate::c_types::SecretKey"),
			"bitcoin::blockdata::script::Script" if is_ref => Some("crate::c_types::u8slice"),
			"bitcoin::blockdata::script::Script" if !is_ref => Some("crate::c_types::derived::CVec_u8Z"),
			"bitcoin::blockdata::transaction::OutPoint" if is_ref => Some("crate::chain::transaction::OutPoint"),
			"bitcoin::blockdata::transaction::Transaction" if is_ref && !ptr_for_ref => Some("crate::c_types::Transaction"),
			"bitcoin::blockdata::transaction::Transaction" => Some("crate::c_types::derived::CVec_u8Z"),
			"bitcoin::OutPoint" => Some("crate::chain::transaction::OutPoint"),
			"bitcoin::network::constants::Network" => Some("crate::bitcoin::network::Network"),
			"bitcoin::blockdata::block::BlockHeader" if is_ref  => Some("*const [u8; 80]"),
			"bitcoin::blockdata::block::BlockHeader" if !is_ref => Some("[u8; 80]"),
			"bitcoin::blockdata::block::Block" if is_ref  => Some("crate::c_types::u8slice"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::hash_types::Txid" if !is_ref => Some("crate::c_types::ThirtyTwoBytes"),
			"bitcoin::hash_types::BlockHash" if is_ref  => Some("*const [u8; 32]"),
			"bitcoin::hash_types::BlockHash" if !is_ref => Some("[u8; 32]"),
			"ln::channelmanager::PaymentHash" if is_ref => Some("*const [u8; 32]"),
			"ln::channelmanager::PaymentHash" if !is_ref => Some("[u8; 32]"),
			"ln::channelmanager::PaymentPreimage" if is_ref => Some("*const [u8; 32]"),
			"ln::channelmanager::PaymentPreimage" if !is_ref => Some("[u8; 32]"),
			"ln::channelmanager::PaymentSecret" if is_ref => Some("*const crate::c_types::ThirtyTwoBytes"),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some("crate::c_types::ThirtyTwoBytes"),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some("*const std::os::raw::c_char"),

			// List of structs we map that aren't detected:
			"ln::features::InitFeatures" if is_ref => Some("*const crate::ln::features::InitFeatures"),
			"ln::features::InitFeatures" => Some("crate::ln::features::InitFeatures"),
			_ => {
				eprintln!("    Type {} (ref: {}) unresolvable in C", full_path, is_ref);
				None
			},
		}
	}

	fn c_type_has_inner_from_path(&self, full_path: &str) -> bool{
		self.crate_types.opaques.get(full_path).is_some()
	}

	fn generated_container_path() -> &'static str {
		"crate::c_types::derived"
	}
	fn container_templ_path() -> &'static str {
		"crate::c_types"
	}
	fn is_transparent_container(&self, full_path: &str, _is_ref: bool) -> bool {
		full_path == "Option"
	}
	fn is_known_container(&self, full_path: &str, is_ref: bool) -> bool {
		(full_path == "Result" && !is_ref) || (full_path == "Vec" && !is_ref) || full_path.ends_with("Tuple")
	}
	fn to_c_conversion_container_new_var<'b>(&self, full_path: &str, is_ref: bool, only_contained_has_inner: bool, var_name: &syn::Ident)
			// Returns prefix + Vec<(prefix, var-name-to-inline-convert)> + suffix
			// expecting one element in the vec per generic type, each of which is inline-converted
			-> Option<(&'b str, Vec<(String, String)>, &'b str)> {
		match full_path {
			"Result" if !is_ref => {
				Some(("match ",
						vec![("{ Ok(mut o) => crate::c_types::CResultTempl::good(".to_string(), "o".to_string()),
						     ("), Err(mut e) => crate::c_types::CResultTempl::err(".to_string(), "e".to_string())],
						") }"))
			},
			"Vec" if !is_ref => {
				Some(("Vec::new(); for item in ", vec![(format!(".drain(..) {{ local_{}.push(", var_name), "item".to_string())], "); }"))
			},
			"Option" if only_contained_has_inner && is_ref =>
				Some(("if ", vec![(".is_none() { std::ptr::null() } else { ".to_string(), format!("({}.as_ref().unwrap())", var_name))], " }")),
			"Option" if is_ref =>
				Some(("if ", vec![(".is_none() { std::ptr::null() } else { ".to_string(), format!("(*{}.as_ref().unwrap())", var_name))], " }")),
			"Option" =>
				Some(("if ", vec![(".is_none() { std::ptr::null_mut() } else { Box::into_raw(Box::new(".to_string(), format!("({}.unwrap())", var_name))], ")) }")),
			_ => None,
		}
	}

	/// only_contained_has_inner implies that there is only one contained element in the container
	/// and it has an inner field (ie is an "opaque" type we've defined).
	fn from_c_conversion_container_new_var<'b>(&self, full_path: &str, is_ref: bool, only_contained_has_inner: bool, var_name: &syn::Ident)
			// Returns prefix + Vec<(prefix, var-name-to-inline-convert)> + suffix
			// expecting one element in the vec per generic type, each of which is inline-converted
			-> Option<(&'b str, Vec<(String, String)>, &'b str)> {
//eprintln!("fccc: {:?}", full_path);
		match full_path {
			"Result" if !is_ref => {
				Some(("match ",
						vec![(".result_good { true => Ok(".to_string(), format!("(*unsafe {{ Box::from_raw({}.contents.result.take_ptr()) }})", var_name)),
						     ("), false => Err(".to_string(), format!("(*unsafe {{ Box::from_raw({}.contents.err.take_ptr()) }})", var_name))],
						")}"))
			},
			"Vec" if !is_ref => {
				Some(("Vec::new(); for mut item in ", vec![(format!(".into_rust().drain(..) {{ local_{}.push(", var_name), "item".to_string())], "); }"))
			},
			"Option" if only_contained_has_inner && is_ref => {
				Some(("if ", vec![(".inner.is_null() { None } else { Some((*".to_string(), format!("{}", var_name))], ").clone()) }"))
			},
			"Option" if only_contained_has_inner => {
				Some(("if ", vec![(".inner.is_null() { None } else { Some(".to_string(), format!("{}", var_name))], ") }"))
			},
			"Option" if is_ref => {
				Some(("if ", vec![(".is_null() { None } else { Some(*".to_string(), format!("{}", var_name))], ") }"))
			},
			"Option" => {
				Some(("if ", vec![(".is_null() { None } else { Some(".to_string(), format!("unsafe {{ *{} }}", var_name))], ") }"))
			},
			_ => None,
		}
	}

	fn from_c_conversion_new_var_from_path<'b>(&self, _full_path: &str, _is_ref: bool) -> Option<(&'b str, &'b str)> {
		None
	}
	fn from_c_conversion_prefix_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<String> {
		if self.is_primitive(full_path) {
			return Some("".to_owned());
		}
		match full_path {
			"Vec" if !is_ref => Some("local_"),
			"Result" if !is_ref => Some("local_"),
			"Option" if is_ref => Some("&local_"),
			"Option" => Some("local_"),

			"[u8; 32]" if is_ref => Some("unsafe { &*"),
			"[u8; 32]" if !is_ref => Some(""),
			"[u8; 16]" if !is_ref => Some(""),
			"[u8; 10]" if !is_ref => Some(""),
			"[u8; 4]" if !is_ref => Some(""),
			"[u8; 3]" if !is_ref => Some(""),

			"[u8]" if is_ref => Some(""),
			"[u32]" if is_ref => Some(""),

			"str" if is_ref => Some(""),
			"String" if !is_ref => Some("String::from_utf8("),

			"std::time::Duration" => Some("std::time::Duration::from_secs("),

			"bitcoin::secp256k1::key::PublicKey" if is_ref => Some("&"),
			"bitcoin::secp256k1::key::PublicKey" => Some(""),
			"bitcoin::secp256k1::Signature" if is_ref => Some("&"),
			"bitcoin::secp256k1::Signature" => Some(""),
			"bitcoin::secp256k1::key::SecretKey" if is_ref => Some("&::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some(""),
			"bitcoin::blockdata::script::Script" if is_ref => Some("&::bitcoin::blockdata::script::Script::from(Vec::from("),
			"bitcoin::blockdata::script::Script" if !is_ref => Some("::bitcoin::blockdata::script::Script::from("),
			"bitcoin::blockdata::transaction::Transaction" if is_ref => Some("&"),
			"bitcoin::blockdata::transaction::Transaction" => Some("::bitcoin::consensus::encode::deserialize(&"),
			"bitcoin::network::constants::Network" => Some(""),
			"bitcoin::blockdata::block::BlockHeader" => Some("&::bitcoin::consensus::encode::deserialize(unsafe { &*"),
			"bitcoin::blockdata::block::Block" if is_ref => Some("&::bitcoin::consensus::encode::deserialize("),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref => Some("&::bitcoin::hash_types::Txid::from_slice(&unsafe { &*"),
			"bitcoin::hash_types::Txid" if !is_ref => Some("::bitcoin::hash_types::Txid::from_slice(&"),
			"bitcoin::hash_types::BlockHash" => Some("::bitcoin::hash_types::BlockHash::from_slice(&"),
			"ln::channelmanager::PaymentHash" if !is_ref => Some("::lightning::ln::channelmanager::PaymentHash("),
			"ln::channelmanager::PaymentHash" if is_ref => Some("&::lightning::ln::channelmanager::PaymentHash(unsafe { *"),
			"ln::channelmanager::PaymentPreimage" if !is_ref => Some("::lightning::ln::channelmanager::PaymentPreimage("),
			"ln::channelmanager::PaymentPreimage" if is_ref => Some("&::lightning::ln::channelmanager::PaymentPreimage(unsafe { *"),
			"ln::channelmanager::PaymentSecret" if is_ref => Some("&::lightning::ln::channelmanager::PaymentSecret(unsafe { *"),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some("::lightning::ln::channelmanager::PaymentSecret("),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if !is_ref => Some("*unsafe { Box::from_raw("),

			// List of traits we map (possibly during processing of other files):
			"crate::util::logger::Logger" => Some(""),
			"crate::chain::chaininterface::BroadcasterInterface" => Some(""),
			"crate::chain::chaininterface::FeeEstimator" => Some(""),
			"crate::chain::chaininterface::ChainWatchInterface" if !is_ref => Some(""),
			"crate::chain::keysinterface::KeysInterface" => Some(""),
			"crate::ln::channelmonitor::ManyChannelMonitor" => Some(""),
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			"crate::util::events::EventsProvider" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable from C", full_path);
				None
			},
		}.map(|s| s.to_owned())
	}
	fn from_c_conversion_suffix_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<String> {
		if self.is_primitive(full_path) {
			return Some("".to_owned());
		}
		match full_path {
			"Vec" if !is_ref => Some(""),
			"Option" => Some(""),
			"Result" if !is_ref => Some(""),

			"[u8; 32]" if is_ref => Some("}"),
			"[u8; 32]" if !is_ref => Some(".data"),
			"[u8; 16]" if !is_ref => Some(".data"),
			"[u8; 10]" if !is_ref => Some(".data"),
			"[u8; 4]" if !is_ref => Some(".data"),
			"[u8; 3]" if !is_ref => Some(".data"),

			"[u8]" if is_ref => Some(".to_slice()"),
			"[u32]" if is_ref => Some(".to_slice()"),

			"str" if is_ref => Some(".into()"),
			"String" if !is_ref => Some(".into_rust()).unwrap()"),

			"std::time::Duration" => Some(")"),

			"bitcoin::secp256k1::key::PublicKey" => Some(".into_rust()"),
			"bitcoin::secp256k1::Signature" => Some(".into_rust()"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some(".into_rust()"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref => Some("}[..]).unwrap()"),
			"bitcoin::blockdata::script::Script" if is_ref => Some(".to_slice()))"),
			"bitcoin::blockdata::script::Script" if !is_ref => Some(".into_rust())"),
			"bitcoin::blockdata::transaction::Transaction" if is_ref => Some(".into_bitcoin()"),
			"bitcoin::blockdata::transaction::Transaction" => Some(".into_rust()[..]).unwrap()"),
			"bitcoin::network::constants::Network" => Some(".into_bitcoin()"),
			"bitcoin::blockdata::block::BlockHeader" => Some(" }).unwrap()"),
			"bitcoin::blockdata::block::Block" => Some(".to_slice()).unwrap()"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref => Some(" }[..]).unwrap()"),
			"bitcoin::hash_types::Txid" => Some(".data[..]).unwrap()"),
			"bitcoin::hash_types::BlockHash" => Some("[..]).unwrap()"),
			"ln::channelmanager::PaymentHash" if !is_ref => Some(")"),
			"ln::channelmanager::PaymentHash" if is_ref => Some(" })"),
			"ln::channelmanager::PaymentPreimage" if !is_ref => Some(")"),
			"ln::channelmanager::PaymentPreimage" if is_ref => Some(" })"),
			"ln::channelmanager::PaymentSecret" if is_ref=> Some(" }.data)"),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some(".data)"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some(".inner as *mut _) }"),
			"ln::features::InitFeatures" if !is_ref => Some(".inner.take_ptr() as *mut _) }"),

			// List of traits we map (possibly during processing of other files):
			"crate::util::logger::Logger" => Some(""),
			"crate::chain::chaininterface::BroadcasterInterface" => Some(""),
			"crate::chain::chaininterface::FeeEstimator" => Some(""),
			"crate::chain::chaininterface::ChainWatchInterface" if !is_ref => Some(""),
			"crate::chain::keysinterface::KeysInterface" => Some(""),
			"crate::ln::channelmonitor::ManyChannelMonitor" => Some(""),
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			"crate::util::events::EventsProvider" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable from C", full_path);
				None
			},
		}.map(|s| s.to_owned())
	}

	fn to_c_conversion_new_var_from_path<'b>(&self, full_path: &str, is_ref: bool) -> Option<(&'b str, &'b str)> {
		if self.is_primitive(full_path) {
			return None;
		}
		match full_path {
			"[u8]" if is_ref => Some(("crate::c_types::u8slice::from_slice(", ")")),
			"[u32]" if is_ref => Some(("crate::c_types::u32slice::from_slice(", ")")),

			"bitcoin::blockdata::transaction::Transaction" if is_ref => Some(("::bitcoin::consensus::encode::serialize(", ")")),
			"bitcoin::blockdata::transaction::Transaction" if !is_ref => Some(("::bitcoin::consensus::encode::serialize(&", ")")),
			"bitcoin::blockdata::block::BlockHeader" if is_ref => Some(("{ let mut s = [0u8; 80]; s[..].copy_from_slice(&::bitcoin::consensus::encode::serialize(", ")); s }")),
			"bitcoin::blockdata::block::Block" if is_ref => Some(("::bitcoin::consensus::encode::serialize(", ")")),
			"bitcoin::hash_types::Txid" => None,

			// Override the default since Records contain an fmt with a lifetime:
			// TODO: We should include the other record fields
			"util::logger::Record" => Some(("std::ffi::CString::new(format!(\"{}\", ", ".args)).unwrap()")),
			_ => None,
		}.map(|s| s.to_owned())
	}
	fn to_c_conversion_inline_prefix_from_path(&self, full_path: &str, is_ref: bool, ptr_for_ref: bool) -> Option<String> {
		if self.is_primitive(full_path) {
			return Some("".to_owned());
		}
		match full_path {
			"Result" if !is_ref => Some("local_"),
			"Vec" if !is_ref => Some("local_"),
			"Option" => Some("local_"),

			"[u8; 32]" if !is_ref => Some("crate::c_types::ThirtyTwoBytes { data: "),
			"[u8; 32]" if is_ref => Some("&"),
			"[u8; 16]" if !is_ref => Some("crate::c_types::SixteenBytes { data: "),
			"[u8; 10]" if !is_ref => Some("crate::c_types::TenBytes { data: "),
			"[u8; 4]" if !is_ref => Some("crate::c_types::FourBytes { data: "),
			"[u8; 3]" if is_ref => Some("&"),

			"[u8]" if is_ref => Some("local_"),
			"[u32]" if is_ref => Some("local_"),

			"str" if is_ref => Some(""),
			"String" if !is_ref => Some(""),

			"std::time::Duration" => Some(""),

			"bitcoin::secp256k1::key::PublicKey" => Some("crate::c_types::PublicKey::from_rust(&"),
			"bitcoin::secp256k1::Signature" => Some("crate::c_types::Signature::from_rust(&"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref  => Some(""),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some("crate::c_types::SecretKey::from_rust("),
			"bitcoin::blockdata::script::Script" if is_ref => Some("crate::c_types::u8slice::from_slice(&"),
			"bitcoin::blockdata::script::Script" if !is_ref => Some(""),
			"bitcoin::blockdata::transaction::Transaction" if is_ref && !ptr_for_ref => Some("crate::c_types::Transaction::from_slice(&local_"),
			"bitcoin::blockdata::transaction::Transaction" => Some("local_"),
			"bitcoin::blockdata::block::BlockHeader" if is_ref => Some("&local_"),
			"bitcoin::blockdata::block::Block" if is_ref => Some("crate::c_types::u8slice::from_slice(&local_"),

			"bitcoin::hash_types::Txid" if !is_ref => Some("crate::c_types::ThirtyTwoBytes { data: "),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" if is_ref => Some(""),
			"bitcoin::hash_types::BlockHash" => Some(""),
			"ln::channelmanager::PaymentHash" if is_ref => Some("&"),
			"ln::channelmanager::PaymentHash" => Some(""),
			"ln::channelmanager::PaymentPreimage" => Some(""),
			"ln::channelmanager::PaymentSecret" if !is_ref => Some("crate::c_types::ThirtyTwoBytes { data: "),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some("local_"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some("Box::into_raw(Box::new(crate::ln::features::InitFeatures { inner: &"),
			"ln::features::InitFeatures" if !is_ref => Some("crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new("),

			// List of traits we map (possibly during processing of other files):
			"crate::ln::msgs::ChannelMessageHandler" if is_ref => Some("&"),
			"crate::ln::msgs::RoutingMessageHandler" if is_ref => Some("&"),
			_ => {
				eprintln!("    Type {} (is_ref: {}) unconvertable to C", full_path, is_ref);
				None
			},
		}.map(|s| s.to_owned())
	}
	fn to_c_conversion_inline_suffix_from_path(&self, full_path: &str, is_ref: bool, ptr_for_ref: bool) -> Option<String> {
		if self.is_primitive(full_path) {
			return Some("".to_owned());
		}
		match full_path {
			"Result" if !is_ref => Some(""),
			"Vec" if !is_ref => Some(".into()"),
			"Option" => Some(""),

			"[u8; 32]" if !is_ref => Some(" }"),
			"[u8; 32]" if is_ref => Some(""),
			"[u8; 16]" if !is_ref => Some(" }"),
			"[u8; 10]" if !is_ref => Some(" }"),
			"[u8; 4]" if !is_ref => Some(" }"),
			"[u8; 3]" if is_ref => Some(""),

			"[u8]" if is_ref => Some(""),
			"[u32]" if is_ref => Some(""),

			"str" if is_ref => Some(".into()"),
			"String" if !is_ref => Some(".into_bytes().into()"),

			"std::time::Duration" => Some(".as_secs()"),

			"bitcoin::secp256k1::key::PublicKey" => Some(")"),
			"bitcoin::secp256k1::Signature" => Some(")"),
			"bitcoin::secp256k1::key::SecretKey" if !is_ref => Some(")"),
			"bitcoin::secp256k1::key::SecretKey" if is_ref => Some(".as_ref()"),
			"bitcoin::blockdata::script::Script" if is_ref => Some("[..])"),
			"bitcoin::blockdata::script::Script" if !is_ref => Some(".into_bytes().into()"),
			"bitcoin::blockdata::transaction::Transaction" if is_ref && !ptr_for_ref => Some(")"),
			"bitcoin::blockdata::transaction::Transaction" => Some(".into()"),
			"bitcoin::blockdata::block::BlockHeader" if is_ref => Some(""),
			"bitcoin::blockdata::block::Block" if is_ref => Some(")"),

			"bitcoin::hash_types::Txid" if !is_ref => Some(".into_inner() }"),

			// Newtypes that we just expose in their original form.
			"bitcoin::hash_types::Txid" => Some(".as_inner()"),
			"bitcoin::hash_types::BlockHash" if !is_ref => Some(".into_inner()"),
			"bitcoin::hash_types::BlockHash" => Some(".as_inner()"),
			"ln::channelmanager::PaymentHash" => Some(".0"),
			"ln::channelmanager::PaymentPreimage" => Some(".0"),
			"ln::channelmanager::PaymentSecret" => Some(".0 }"),

			// Override the default since Records contain an fmt with a lifetime:
			"util::logger::Record" => Some(".as_ptr()"),

			// List of structs we map (possibly during processing of other files):
			"ln::features::InitFeatures" if is_ref => Some(", _underlying_ref: true }))"),
			"ln::features::InitFeatures" => Some(")), _underlying_ref: false }"),

			// List of traits we map (possibly during processing of other files):
			"crate::ln::msgs::ChannelMessageHandler" => Some(""),
			"crate::ln::msgs::RoutingMessageHandler" => Some(""),
			_ => {
				eprintln!("    Type {} unconvertable to C", full_path);
				None
			},
		}.map(|s| s.to_owned())
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
	pub fn enum_ignored(&mut self, ident: &'c syn::Ident) {
		self.declared.insert(ident.clone(), DeclType::EnumIgnored);
	}
	pub fn struct_imported(&mut self, ident: &'c syn::Ident, named: String) {
		eprintln!("Imported {} as {}", ident, named);
		self.declared.insert(ident.clone(), DeclType::StructImported);
	}
	pub fn struct_ignored(&mut self, ident: &syn::Ident) {
		eprintln!("Not importing {}", ident);
		self.declared.insert(ident.clone(), DeclType::StructIgnored);
	}
	pub fn trait_declared(&mut self, ident: &syn::Ident, t: &'c syn::ItemTrait) {
		eprintln!("Trait {} created", ident);
		self.declared.insert(ident.clone(), DeclType::Trait(t));
	}
	pub fn get_declared_type(&'a self, ident: &syn::Ident) -> Option<&'a DeclType<'c>> {
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

	/// Prints a constructor for something which is "uninitialized" (but obviously not actually
	/// unint'd memory).
	pub fn print_empty_rust_val<W: std::io::Write>(&self, w: &mut W, t: &syn::Type) {
		match t {
			syn::Type::Path(p) => {
				let resolved = self.resolve_path(&p.path);
				if self.crate_types.opaques.get(&resolved).is_some() {
					write!(w, "crate::{} {{ inner: std::ptr::null(), _underlying_ref: false }}", resolved).unwrap();
				} else { unimplemented!(); }
			},
			syn::Type::Array(a) => {
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						let arrty = format!("[u8; {}]", i.base10_digits());
						write!(w, "{}", self.to_c_conversion_inline_prefix_from_path(&arrty, false, false).unwrap()).unwrap();
						write!(w, "[0; {}]", i.base10_digits()).unwrap();
						write!(w, "{}", self.to_c_conversion_inline_suffix_from_path(&arrty, false, false).unwrap()).unwrap();
					} else { unimplemented!(); }
				} else { unimplemented!(); }
			}
			_ => unimplemented!(),
		}
	}

	/// Prints a suffix to determine if a variable is empty (ie was set by print_empty_rust_val).
	pub fn print_empty_rust_val_check<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, var_access: &str) {
		match t {
			syn::Type::Path(p) => {
				let resolved = self.resolve_path(&p.path);
				if self.crate_types.opaques.get(&resolved).is_some() {
					write!(w, "{}.inner.is_null()", var_access).unwrap();
				} else { unimplemented!(); }
			},
			syn::Type::Array(a) => {
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						let arrty = format!("[u8; {}]", i.base10_digits());
						// We don't (yet) support a prefix conversion here.
						write!(w, "{}{}{} == [0; {}]",
							self.from_c_conversion_prefix_from_path(&arrty, false).unwrap(),
							var_access,
							self.from_c_conversion_suffix_from_path(&arrty, false).unwrap(),
							i.base10_digits()).unwrap();
					} else { unimplemented!(); }
				} else { unimplemented!(); }
			}
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

	fn print_conversion_inline_intern<W: std::io::Write, LP: Fn(&str, bool, bool) -> Option<String>, DL: Fn(&mut W, &DeclType, &str, bool)>
			(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool,
			 tupleconv: &str, sliceconv: &str, prefix: bool,
			 path_lookup: LP, decl_lookup: DL) {
		match t {
			syn::Type::Reference(r) => {
				if let Some(lft) = &r.lifetime {
					if format!("{}", lft.ident) != "static" { unimplemented!(); }
				}
				self.print_conversion_inline_intern(w, &*r.elem, generics, true, ptr_for_ref, tupleconv, sliceconv, prefix, path_lookup, decl_lookup);
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}

				if let Some(gen_types) = generics {
					if let Some((path, synpath)) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&path, is_ref) && !self.is_transparent_container(&path, is_ref));
						if let Some(c_type) = path_lookup(&path, is_ref, ptr_for_ref) {
							write!(w, "{}", c_type).unwrap();
							return;
						} else if let Some(decl_type) = self.declared.get(single_ident_generic_path_to_ident(synpath).unwrap()) {
							decl_lookup(w, decl_type, &self.maybe_resolve_path(synpath).unwrap(), is_ref);
							return;
						} else { unimplemented!(); }
					}
				}

				let resolved_path = self.resolve_path(&p.path);
				if let Some(c_type) = path_lookup(&resolved_path, is_ref, ptr_for_ref) {
					write!(w, "{}", c_type).unwrap();
				} else if self.crate_types.opaques.get(&resolved_path).is_some() {
					decl_lookup(w, &DeclType::StructImported, &resolved_path, is_ref);
				} else if self.crate_types.mirrored_enums.get(&resolved_path).is_some() {
					decl_lookup(w, &DeclType::MirroredEnum, &resolved_path, is_ref);
				} else if let Some(ident) = single_ident_generic_path_to_ident(&p.path) {
					if let Some(_) = self.imports.get(ident) {
						// prefix_lookup has to have succeeded:
						panic!("Failed to print inline conversion for {}", ident);
					} else if let Some(decl_type) = self.declared.get(ident) {
						decl_lookup(w, decl_type, &self.maybe_resolve_ident(ident).unwrap(), is_ref);
					} else { unimplemented!(); }
				}
			},
			syn::Type::Array(a) => {
				// We assume all arrays contain only [u8; X]s.
				// This may result in some outputs not compiling.
				if let syn::Expr::Lit(l) = &a.len {
					if let syn::Lit::Int(i) = &l.lit {
						write!(w, "{}", path_lookup(&format!("[u8; {}]", i.base10_digits()), is_ref, ptr_for_ref).unwrap()).unwrap();
					} else { unimplemented!(); }
				} else { unimplemented!(); }
			},
			syn::Type::Slice(s) => {
				// We assume all slices contain only u8s.
				// This may result in some outputs not compiling.
				if let syn::Type::Path(p) = &*s.elem {
					let resolved = self.resolve_path(&p.path);
					assert!(self.is_primitive(&resolved));
					write!(w, "{}", path_lookup("[u8]", is_ref, ptr_for_ref).unwrap()).unwrap();
				} else if let syn::Type::Reference(_) = &*s.elem {
					write!(w, "{}", sliceconv).unwrap();
				} else { unimplemented!(); }
			},
			syn::Type::Tuple(t) => {
				if t.elems.is_empty() {
					// cbindgen has poor support for (), see, eg https://github.com/eqrion/cbindgen/issues/527
					// so work around it by just pretending its a 0u8
					write!(w, "{}", tupleconv).unwrap();
				} else {
					if prefix { write!(w, "local_").unwrap(); }
				}
			},
			_ => unimplemented!(),
		}
	}

	fn print_to_c_conversion_inline_prefix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool, from_ptr: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, ptr_for_ref, "0u8 /*", "", true,
				|a, b, c| self.to_c_conversion_inline_prefix_from_path(a, b, c),
				|w, decl_type, decl_path, is_ref| {
					match decl_type {
						DeclType::MirroredEnum if is_ref => write!(w, "&crate::{}::from_ln(&", decl_path).unwrap(),
						DeclType::MirroredEnum => write!(w, "crate::{}::ln_into(", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported if is_ref && ptr_for_ref =>
							write!(w, "Box::into_raw(Box::new(crate::{} {{ inner: &", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported if is_ref =>
							write!(w, "&crate::{} {{ inner: ", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported if !is_ref && from_ptr =>
							write!(w, "crate::{} {{ inner: ", decl_path).unwrap(),
						DeclType::EnumIgnored|DeclType::StructImported if !is_ref =>
							write!(w, "crate::{} {{ inner: Box::into_raw(Box::new(", decl_path).unwrap(),
						_ => unimplemented!(),
					}
				});
	}
	pub fn print_to_c_conversion_inline_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		self.print_to_c_conversion_inline_prefix_inner(w, t, generics, false, ptr_for_ref, false);
	}
	fn print_to_c_conversion_inline_suffix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool, from_ptr: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, ptr_for_ref, "*/", ".into()", false,
				|a, b, c| self.to_c_conversion_inline_suffix_from_path(a, b, c),
				|w, decl_type, _full_path, is_ref| match decl_type {
					DeclType::MirroredEnum => write!(w, ")").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported if is_ref && ptr_for_ref => write!(w, ", _underlying_ref: true }} ))").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported if is_ref => write!(w, ", _underlying_ref: true }}").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported if !is_ref && from_ptr => write!(w, ", _underlying_ref: false }}").unwrap(),
					DeclType::EnumIgnored|DeclType::StructImported if !is_ref => write!(w, ")), _underlying_ref: false }}").unwrap(),
					_ => unimplemented!(),
				});
	}
	pub fn print_to_c_conversion_inline_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		self.print_to_c_conversion_inline_suffix_inner(w, t, generics, false, ptr_for_ref, false);
	}

	fn print_from_c_conversion_prefix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, false, "() /*", "&local_", true,
				|a, b, _c| self.from_c_conversion_prefix_from_path(a, b),
				|w, decl_type, _full_path, is_ref| match decl_type {
					DeclType::StructImported if is_ref && ptr_for_ref => write!(w, "unsafe {{ &*(*").unwrap(),
					DeclType::StructImported if is_ref => write!(w, "unsafe {{ &*").unwrap(),
					DeclType::StructImported if !is_ref => write!(w, "*unsafe {{ Box::from_raw(").unwrap(),
					DeclType::MirroredEnum if is_ref => write!(w, "&").unwrap(),
					DeclType::MirroredEnum => {},
					DeclType::Trait(_) => {},
					_ => unimplemented!(),
				});
	}
	pub fn print_from_c_conversion_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_from_c_conversion_prefix_inner(w, t, generics, false, false);
	}
	fn print_from_c_conversion_suffix_inner<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, is_ref: bool, ptr_for_ref: bool) {
		self.print_conversion_inline_intern(w, t, generics, is_ref, false, "*/", "[..]", false,
				|a, b, _c| self.from_c_conversion_suffix_from_path(a, b),
				|w, decl_type, _full_path, is_ref| match decl_type {
					DeclType::StructImported if is_ref && ptr_for_ref => write!(w, ").inner }}").unwrap(),
					DeclType::StructImported if is_ref => write!(w, ".inner }}").unwrap(),
					DeclType::StructImported if !is_ref => write!(w, ".inner.take_ptr() as *mut _) }}").unwrap(),
					DeclType::MirroredEnum if is_ref => write!(w, ".to_ln()").unwrap(),
					DeclType::MirroredEnum => write!(w, ".into_ln()").unwrap(),
					DeclType::Trait(_) => {},
					_ => unimplemented!(),
				});
	}
	pub fn print_from_c_conversion_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_from_c_conversion_suffix_inner(w, t, generics, false, false);
	}
	// Note that compared to the above conversion functions, the following are generally
	// significantly undertested:
	pub fn print_from_c_conversion_to_ref_prefix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_conversion_inline_intern(w, t, generics, false, false, "() /*", "&", true,
				|a, b, _c| {
					if let Some(conv) = self.from_c_conversion_prefix_from_path(a, b) {
						Some(format!("&{}", conv))
					} else { None }
				},
				|w, decl_type, _full_path, is_ref| match decl_type {
					DeclType::StructImported if !is_ref => write!(w, "unsafe {{ &*").unwrap(),
					_ => unimplemented!(),
				});
	}
	pub fn print_from_c_conversion_to_ref_suffix<W: std::io::Write>(&self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>) {
		self.print_conversion_inline_intern(w, t, generics, false, false, "*/", ".into_vec()[..]", false,
				|a, b, _c| self.from_c_conversion_suffix_from_path(a, b),
				|w, decl_type, _full_path, is_ref| match decl_type {
					DeclType::StructImported if !is_ref => write!(w, ".inner }}").unwrap(),
					_ => unimplemented!(),
				});
	}

	fn print_conversion_new_var_intern<'b, W: std::io::Write,
		LP: Fn(&str, bool) -> Option<(&str, &str)>,
		LC: Fn(&str, bool, bool, &syn::Ident) ->  Option<(&'b str, Vec<(String, String)>, &'b str)>,
		VP: Fn(&mut W, &syn::Type, Option<&GenericTypes>, bool, bool),
		VS: Fn(&mut W, &syn::Type, Option<&GenericTypes>, bool, bool)>
			(&self, w: &mut W, ident: &syn::Ident, var: &str, t: &syn::Type, generics: Option<&GenericTypes>, mut is_ref: bool, to_c: bool,
			 path_lookup: &LP, container_lookup: &LC, var_prefix: &VP, var_suffix: &VS) -> bool {
		match t {
			syn::Type::Reference(r) => {
				if let Some(lft) = &r.lifetime {
					if format!("{}", lft.ident) != "static" { unimplemented!(); }
				}
				self.print_conversion_new_var_intern(w, ident, var, &*r.elem, generics, true, to_c, path_lookup, container_lookup, var_prefix, var_suffix)
			},
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					unimplemented!();
				}
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						assert!(!self.is_known_container(&resolved.0, is_ref) && !self.is_transparent_container(&resolved.0, is_ref));
						if let Some((prefix, suffix)) = path_lookup(&resolved.0, is_ref) {
							write!(w, "let local_{} = {}{}{};", ident, prefix, var, suffix).unwrap();
							return true;
						} else { return false; }
					}
				}
				let resolved_path = self.resolve_path(&p.path);
				if self.is_known_container(&resolved_path, is_ref) || self.is_transparent_container(&resolved_path, is_ref) {
					if let syn::PathArguments::AngleBracketed(args) = &p.path.segments.iter().next().unwrap().arguments {
						let mut needs_ref_map = false;
						let mut only_contained_has_inner = false;
						if args.args.len() == 1 && self.is_transparent_container(&resolved_path, is_ref) {
							if let syn::GenericArgument::Type(syn::Type::Reference(t)) = args.args.iter().next().unwrap() {
								if let syn::Type::Path(p) = &*t.elem {
									only_contained_has_inner = self.c_type_has_inner_from_path(&self.resolve_path(&p.path));
								} else { return false; }
								is_ref = true;
								needs_ref_map = true;
							} else if let syn::GenericArgument::Type(syn::Type::Path(p)) = args.args.iter().next().unwrap() {
								only_contained_has_inner = self.c_type_has_inner_from_path(&self.resolve_path(&p.path));
							}
						}

						let (prefix, conversions, suffix) = container_lookup(&resolved_path, is_ref, only_contained_has_inner, ident).unwrap();
						assert_eq!(conversions.len(), args.args.len());
						write!(w, "let mut local_{}{} = ", ident, if !to_c && needs_ref_map {"_base"} else { "" }).unwrap();
						if only_contained_has_inner && to_c {
							if let syn::GenericArgument::Type(ty) = args.args.iter().next().unwrap() {
								var_prefix(w, ty, generics, is_ref, true);
							}
						}
						write!(w, "{}{}", prefix, var).unwrap();

						for ((pfx, var_name), (idx, arg)) in conversions.iter().zip(args.args.iter().enumerate()) {
							if let syn::GenericArgument::Type(ty) = arg {
								let mut var = std::io::Cursor::new(Vec::new());
								write!(&mut var, "{}", var_name).unwrap();
								let var_access = String::from_utf8(var.into_inner()).unwrap();

								write!(w, "{} {{ ", pfx).unwrap();
								let new_var_name = format!("{}_{}", ident, idx);
								let new_var = self.print_conversion_new_var_intern(w, &syn::Ident::new(&new_var_name, Span::call_site()),
										&var_name, ty, generics, false, to_c, path_lookup, container_lookup, var_prefix, var_suffix);
								if new_var { write!(w, " ").unwrap(); }
								if needs_ref_map && to_c && !only_contained_has_inner {
									write!(w, "Box::into_raw(Box::new(").unwrap();
								}
								if !only_contained_has_inner || !to_c {
									var_prefix(w, ty, generics, is_ref, false);
								}
								write!(w, "{}", if new_var { new_var_name } else { var_access }).unwrap();
								if !only_contained_has_inner || !to_c {
									var_suffix(w, ty, generics, is_ref, false);
								}
								if needs_ref_map && to_c && !only_contained_has_inner {
									write!(w, "))").unwrap();
								}
								write!(w, " }}").unwrap();
							} else { unimplemented!(); }
						}
						write!(w, "{}", suffix).unwrap();
						if only_contained_has_inner && to_c {
							if let syn::GenericArgument::Type(ty) = args.args.iter().next().unwrap() {
								var_suffix(w, ty, generics, is_ref, true);
							}
						}
						write!(w, ";").unwrap();
						if !to_c && needs_ref_map {
							write!(w, " let mut local_{} = local_{}_base.as_ref();", ident, ident).unwrap();
						}
					} else { unimplemented!(); }
					true
				} else if self.is_primitive(&resolved_path) {
					false
				} else if let Some(ty_ident) = single_ident_generic_path_to_ident(&p.path) {
					if let Some((prefix, suffix)) = path_lookup(&resolved_path, is_ref) {
						write!(w, "let local_{} = {}{}{};", ident, prefix, var, suffix).unwrap();
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
						write!(w, "let local_{} = {}{}{};", ident, prefix, var, suffix).unwrap();
						true
					} else { false }
				} else if let syn::Type::Reference(r) = &*s.elem {
					if !to_c {
						if let syn::Type::Path(p) = &*r.elem {
							if !self.c_type_has_inner_from_path(&self.resolve_path(&p.path)) {
								write!(w, "let local_{}_vec = {}.into_vec(); let mut local_{} = local_{}_vec.iter().collect::<Vec<_>>();",
									ident, ident, ident, ident).unwrap();
							} else {
								write!(w, "let local_{} = {}.into_vec();", ident, ident).unwrap();
							}
						} else { unimplemented!(); };
						true
					} else { false }
				} else { unimplemented!() }
			},
			syn::Type::Tuple(t) => {
				if !t.elems.is_empty() {
					// We don't (yet) support tuple elements which cannot be converted inline
					write!(w, "let (").unwrap();
					for idx in 0..t.elems.len() {
						if idx != 0 { write!(w, ", ").unwrap(); }
						write!(w, "mut orig_{}_{}", ident, idx).unwrap();
					}
					write!(w, ") = {}{}; ", var, if !to_c { ".to_rust()" } else { "" }).unwrap();
					for (idx, elem) in t.elems.iter().enumerate() {
						if let syn::Type::Path(_) = elem {
							let v_name = format!("orig_{}_{}", ident, idx);
							let tuple_elem_ident = syn::Ident::new(&v_name, Span::call_site());
							if self.print_conversion_new_var_intern(w, &tuple_elem_ident, &v_name, elem, generics, is_ref,
									to_c, path_lookup, container_lookup, var_prefix, var_suffix) {
								write!(w, " ").unwrap();
							}
						}
					}
					write!(w, "let local_{} = (", ident).unwrap();
					for (idx, elem) in t.elems.iter().enumerate() {
						if idx != 0 { write!(w, ", ").unwrap(); }
						var_prefix(w, elem, generics, is_ref, false);
						write!(w, "orig_{}_{}", ident, idx).unwrap();
						var_suffix(w, elem, generics, is_ref, false);
					}
					write!(w, "){};", if to_c { ".into()" } else { "" }).unwrap();
					true
				} else { false }
			},
			_ => unimplemented!(),
		}
	}

	pub fn print_to_c_conversion_new_var_inner<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, var_access: &str, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		self.print_conversion_new_var_intern(w, ident, var_access, t, generics, false, true,
			&|a, b| self.to_c_conversion_new_var_from_path(a, b),
			&|a, b, c, d| self.to_c_conversion_container_new_var(a, b, c, d),
			// We force ptr_for_ref here since we can't generate a ref on one line and use it later
			&|a, b, c, d, e| self.print_to_c_conversion_inline_prefix_inner(a, b, c, d, false, e),
			&|a, b, c, d, e| self.print_to_c_conversion_inline_suffix_inner(a, b, c, d, false, e))
	}
	pub fn print_to_c_conversion_new_var<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		self.print_to_c_conversion_new_var_inner(w, ident, &format!("{}", ident), t, generics)
	}
	pub fn print_from_c_conversion_new_var<W: std::io::Write>(&self, w: &mut W, ident: &syn::Ident, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		self.print_conversion_new_var_intern(w, ident, &format!("{}", ident), t, generics, false, false,
			&|a, b| self.from_c_conversion_new_var_from_path(a, b),
			&|a, b, c, d| self.from_c_conversion_container_new_var(a, b, c, d),
			// We force ptr_for_ref here since we can't generate a ref on one line and use it later
			&|a, b, c, d, _e| self.print_from_c_conversion_prefix_inner(a, b, c, d, true),
			&|a, b, c, d, _e| self.print_from_c_conversion_suffix_inner(a, b, c, d, true))
	}

	// ******************************************************
	// *** C Container Type Equivalent and alias Printing ***
	// ******************************************************

	fn print_template_constructor<W: std::io::Write>(&mut self, w: &mut W, container_type: &str, mangled_container: &str, args: &Vec<&syn::Type>, is_ref: bool) {
		if container_type == "Result" {
			assert_eq!(args.len(), 2);
			macro_rules! print_fn {
				($call: expr) => { {
					writeln!(w, "#[no_mangle]\npub extern \"C\" fn {}_{}() -> {} {{", mangled_container, $call, mangled_container).unwrap();
					writeln!(w, "\t{}::CResultTempl::{}(0)\n}}\n", Self::container_templ_path(), $call).unwrap();
				} }
			}
			macro_rules! print_alias {
				($call: expr, $item: expr) => { {
					write!(w, "#[no_mangle]\npub static {}_{}: extern \"C\" fn (", mangled_container, $call).unwrap();
					if let syn::Type::Path(syn::TypePath { path, .. }) = $item {
						let resolved = self.resolve_path(path);
						if self.is_known_container(&resolved, is_ref) || self.is_transparent_container(&resolved, is_ref) {
							self.print_c_mangled_container_path_intern(w, Self::path_to_generic_args(path),
								&format!("{}", single_ident_generic_path_to_ident(path).unwrap()), is_ref, false, false, false);
						} else {
							self.print_template_generics(w, &mut [$item].iter().map(|t| *t), is_ref, true);
						}
					} else if let syn::Type::Tuple(syn::TypeTuple { elems, .. }) = $item {
						self.print_c_mangled_container_path_intern(w, elems.iter().collect(),
							&format!("{}Tuple", elems.len()), is_ref, false, false, false);
					} else { unimplemented!(); }
					write!(w, ") -> {} =\n\t{}::CResultTempl::<", mangled_container, Self::container_templ_path()).unwrap();
					self.print_template_generics(w, &mut args.iter().map(|t| *t), is_ref, true);
					writeln!(w, ">::{};\n", $call).unwrap();
				} }
			}
			match args[0] {
				syn::Type::Tuple(t) if t.elems.is_empty() => print_fn!("good"),
				_ => print_alias!("good", args[0]),
			}
			match args[1] {
				syn::Type::Tuple(t) if t.elems.is_empty() => print_fn!("err"),
				_ => print_alias!("err", args[1]),
			}
		} else if container_type == "Slice" {
			let inner_type = args.iter().map(|t| *t).next().unwrap();
			let (has_inner, resolved_path) = if let syn::Type::Path(p) = inner_type {
				let res = self.resolve_path(&p.path);
				(self.c_type_has_inner_from_path(&res), res)
			} else { unimplemented!(); };

			write!(w, "impl From<&[&").unwrap();
			self.print_template_generics(w, &mut args.iter().map(|t| *t), false, false);
			writeln!(w, "]> for {} {{", mangled_container).unwrap();
			write!(w, "\tfn from(slice: &[&").unwrap();
			self.print_template_generics(w, &mut args.iter().map(|t| *t), false, false);
			writeln!(w, "]) -> Self {{").unwrap();
			writeln!(w, "\t\tlet mut v = Vec::with_capacity(slice.len());").unwrap();
			writeln!(w, "\t\tfor e in slice.iter() {{").unwrap();
			if has_inner {
				writeln!(w, "\t\t\tv.push(crate::{} {{ inner: *e, _underlying_ref: true }});", resolved_path).unwrap();
			} else {
				write!(w, "\t\t\t").unwrap();
				let new_var = self.print_to_c_conversion_new_var_inner(w, &syn::Ident::new("e", Span::call_site()), "**e", inner_type, None);
				if new_var {
					write!(w, "\n\t\t\t").unwrap();
				}
				write!(w, "v.push(").unwrap();
				self.print_to_c_conversion_inline_prefix(w, inner_type, None, false);
				write!(w, "{}e", if new_var { "" } else { "**" }).unwrap();
				self.print_to_c_conversion_inline_suffix(w, inner_type, None, false);
				writeln!(w, ");").unwrap();
			}
			writeln!(w, "\t\t}}").unwrap();
			writeln!(w, "\t\tSelf {{ datalen: v.len(), data: unsafe {{ (*Box::into_raw(v.into_boxed_slice())).as_mut_ptr() }} }}").unwrap();
			writeln!(w, "\t}}").unwrap();
			writeln!(w, "}}").unwrap();

			writeln!(w, "impl {} {{", mangled_container).unwrap();
			write!(w, "\tpub(crate) fn into_vec(mut self) -> Vec<").unwrap();
			if has_inner {
				write!(w, "&'static ").unwrap();
			}
			self.print_template_generics(w, &mut args.iter().map(|t| *t), is_ref, false);
			writeln!(w, "> {{").unwrap();
			writeln!(w, "\t\tlet mut ret = Vec::new();").unwrap();
			writeln!(w, "\t\tlet mut orig: Vec<_> = unsafe {{ Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.datalen)) }}.into();").unwrap();
			writeln!(w, "\t\tfor e in orig.drain(..) {{").unwrap();
			if has_inner {
				writeln!(w, "\t\t\tret.push(unsafe {{ &*e.inner }});").unwrap();
			} else {
				write!(w, "\t\t\t").unwrap();
				if self.print_from_c_conversion_new_var(w, &syn::Ident::new("e", Span::call_site()), inner_type, None) {
					write!(w, "\n\t\t\t").unwrap();
				}
				write!(w, "ret.push(").unwrap();
				self.print_from_c_conversion_prefix(w, inner_type, None);
				write!(w, "e").unwrap();
				self.print_from_c_conversion_suffix(w, inner_type, None);
				writeln!(w, ");").unwrap();
			}
			writeln!(w, "\t\t}}").unwrap();
			writeln!(w, "\t\t// Make sure we don't try to de-allocate the things we just drain(..)ed").unwrap();
			writeln!(w, "\t\tself.data = std::ptr::null_mut(); self.datalen = 0;").unwrap();
			writeln!(w, "\t\tret\n\t}}").unwrap();
			writeln!(w, "}}").unwrap();
		} else if container_type.ends_with("Tuple") {
			write!(w, "#[no_mangle]\npub extern \"C\" fn {}_new(", mangled_container).unwrap();
			for (idx, gen) in args.iter().enumerate() {
				write!(w, "{}{}: ", if idx != 0 { ", " } else { "" }, ('a' as u8 + idx as u8) as char).unwrap();
				self.print_c_type_intern(None, w, gen, false, false, false);
			}
			writeln!(w, ") -> {} {{", mangled_container).unwrap();
			writeln!(w, "\t{} {{", mangled_container).unwrap();
			for idx in 0..args.len() {
				writeln!(w, "\t\t{}: Box::into_raw(Box::new({})),", ('a' as u8 + idx as u8) as char, ('a' as u8 + idx as u8) as char).unwrap();
			}
			writeln!(w, "\t}}\n}}\n").unwrap();
		} else {
			writeln!(w, "").unwrap();
		}
	}

	fn print_template_generics<'b, W: std::io::Write>(&self, w: &mut W, args: &mut dyn Iterator<Item=&'b syn::Type>, is_ref: bool, in_crate: bool) {
		for (idx, t) in args.enumerate() {
			if idx != 0 {
				write!(w, ", ").unwrap();
			}
			if let syn::Type::Tuple(tup) = t {
				if tup.elems.is_empty() {
					write!(w, "u8").unwrap();
				} else {
					write!(w, "{}::C{}TupleTempl<", Self::container_templ_path(), tup.elems.len()).unwrap();
					self.print_template_generics(w, &mut tup.elems.iter(), is_ref, in_crate);
					write!(w, ">").unwrap();
				}
			} else if let syn::Type::Path(p_arg) = t {
				let resolved_generic = self.resolve_path(&p_arg.path);
				if self.is_primitive(&resolved_generic) {
					write!(w, "{}", resolved_generic).unwrap();
				} else if let Some(c_type) = self.c_type_from_path(&resolved_generic, is_ref, false) {
					if self.is_known_container(&resolved_generic, is_ref) {
						write!(w, "{}::C{}Templ<", Self::container_templ_path(), single_ident_generic_path_to_ident(&p_arg.path).unwrap()).unwrap();
						assert_eq!(p_arg.path.segments.len(), 1);
						if let syn::PathArguments::AngleBracketed(args) = &p_arg.path.segments.iter().next().unwrap().arguments {
							self.print_template_generics(w, &mut args.args.iter().map(|gen|
								if let syn::GenericArgument::Type(t) = gen { t } else { unimplemented!() }), is_ref, in_crate);
						} else { unimplemented!(); }
						write!(w, ">").unwrap();
					} else if resolved_generic == "Option" {
						if let syn::PathArguments::AngleBracketed(args) = &p_arg.path.segments.iter().next().unwrap().arguments {
							self.print_template_generics(w, &mut args.args.iter().map(|gen|
								if let syn::GenericArgument::Type(t) = gen { t } else { unimplemented!() }), is_ref, in_crate);
						} else { unimplemented!(); }
					} else if in_crate {
						write!(w, "{}", c_type).unwrap();
					} else {
						self.print_rust_type(w, &t);
					}
				} else {
					write!(w, "{}::{}", if in_crate { "crate" } else { self.orig_crate }, resolved_generic).unwrap();
				}
			} else if let syn::Type::Array(a_arg) = t {
				if let syn::Type::Path(p_arg) = &*a_arg.elem {
					let resolved = self.resolve_path(&p_arg.path);
					assert!(self.is_primitive(&resolved));
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(len), .. }) = &a_arg.len {
						write!(w, "{}",
							self.c_type_from_path(&format!("[{}; {}]", resolved, len.base10_digits()), is_ref, false).unwrap()).unwrap();
					}
				}
			}
		}
	}
	fn check_create_container(&mut self, mangled_container: String, container_type: &str, args: Vec<&syn::Type>, is_ref: bool) {
		if !self.crate_types.templates_defined.get(&mangled_container).is_some() {
			self.crate_types.templates_defined.insert(mangled_container.clone(), container_type != "Slice");
			let mut created_container: Vec<u8> = Vec::new();

			write!(&mut created_container, "#[no_mangle]\npub type {} = ", mangled_container).unwrap();
			write!(&mut created_container, "{}::C{}Templ<", Self::container_templ_path(), container_type).unwrap();
			self.print_template_generics(&mut created_container, &mut args.iter().map(|t| *t), false, true);
			writeln!(&mut created_container, ">;").unwrap();

			if container_type != "Slice" {
				write!(&mut created_container, "#[no_mangle]\npub static {}_free: extern \"C\" fn({}) = ", mangled_container, mangled_container).unwrap();
				write!(&mut created_container, "{}::C{}Templ_free::<", Self::container_templ_path(), container_type).unwrap();
				self.print_template_generics(&mut created_container, &mut args.iter().map(|t| *t), is_ref, true);
				writeln!(&mut created_container, ">;").unwrap();
			}

			self.print_template_constructor(&mut created_container, container_type, &mangled_container, &args, is_ref);

			self.crate_types.template_file.write(&created_container).unwrap();
		}
	}
	fn path_to_generic_args(path: &syn::Path) -> Vec<&syn::Type> {
		if let syn::PathArguments::AngleBracketed(args) = &path.segments.iter().next().unwrap().arguments {
			args.args.iter().map(|gen| if let syn::GenericArgument::Type(t) = gen { t } else { unimplemented!() }).collect()
		} else { unimplemented!(); }
	}
	fn print_c_mangled_container_path_intern<W: std::io::Write>
			(&mut self, w: &mut W, args: Vec<&syn::Type>, ident: &str, is_ref: bool, is_mut: bool, ptr_for_ref: bool, in_type: bool) -> bool {
		let mut mangled_type: Vec<u8> = Vec::new();
		if !self.is_transparent_container(ident, is_ref) {
			write!(w, "C{}_", ident).unwrap();
			write!(mangled_type, "C{}_", ident).unwrap();
		} else { assert_eq!(args.len(), 1); }
		for arg in args.iter() {
			macro_rules! print_path {
				($p_arg: expr, $extra_write: expr) => {
					let subtype = self.resolve_path(&$p_arg.path);
					if self.is_transparent_container(ident, is_ref) {
						// We dont (yet) support primitives or containers inside transparent
						// containers, so check for that first:
						if self.is_primitive(&subtype) { return false; }
						if self.is_known_container(&subtype, is_ref) { return false; }
						if !in_type {
							if self.c_type_has_inner_from_path(&subtype) {
								if !self.print_c_path_intern(w, &$p_arg.path, is_ref, is_mut, ptr_for_ref) { return false; }
							} else {
								if !self.print_c_path_intern(w, &$p_arg.path, true, is_mut, true) { return false; }
							}
						} else {
							if $p_arg.path.segments.len() == 1 {
								write!(w, "{}", $p_arg.path.segments.iter().next().unwrap().ident).unwrap();
							} else {
								return false;
							}
						}
					} else if self.is_known_container(&subtype, is_ref) || self.is_transparent_container(&subtype, is_ref) {
						if !self.print_c_mangled_container_path_intern(w, Self::path_to_generic_args(&$p_arg.path),
								&subtype, is_ref, is_mut, ptr_for_ref, true) {
							return false;
						}
						self.print_c_mangled_container_path_intern(&mut mangled_type, Self::path_to_generic_args(&$p_arg.path),
							&subtype, is_ref, is_mut, ptr_for_ref, true);
						if let Some(w2) = $extra_write as Option<&mut Vec<u8>> {
							self.print_c_mangled_container_path_intern(w2, Self::path_to_generic_args(&$p_arg.path),
								&subtype, is_ref, is_mut, ptr_for_ref, true);
						}
					} else if let Some(id) = $p_arg.path.get_ident() {
						write!(w, "{}", id).unwrap();
						write!(mangled_type, "{}", id).unwrap();
						if let Some(w2) = $extra_write as Option<&mut Vec<u8>> {
							write!(w2, "{}", id).unwrap();
						}
					} else { return false; }
				}

			}
			if let syn::Type::Tuple(tuple) = arg {
				if tuple.elems.len() == 0 {
					write!(w, "None").unwrap();
					write!(mangled_type, "None").unwrap();
				} else {
					let mut mangled_tuple_type: Vec<u8> = Vec::new();

					write!(w, "C{}Tuple_", tuple.elems.len()).unwrap();
					write!(mangled_type, "C{}Tuple_", tuple.elems.len()).unwrap();
					write!(mangled_tuple_type, "C{}Tuple_", tuple.elems.len()).unwrap();
					for elem in tuple.elems.iter() {
						if let syn::Type::Path(p) = elem {
							print_path!(p, Some(&mut mangled_tuple_type));
						} else { return false; }
					}
					write!(w, "Z").unwrap();
					write!(mangled_type, "Z").unwrap();
					write!(mangled_tuple_type, "Z").unwrap();
					self.check_create_container(String::from_utf8(mangled_tuple_type).unwrap(),
						&format!("{}Tuple", tuple.elems.len()), tuple.elems.iter().collect(), is_ref);
				}
			} else if let syn::Type::Path(p_arg) = arg {
				print_path!(p_arg, None);
			} else if let syn::Type::Reference(refty) = arg {
				if let syn::Type::Path(p_arg) = &*refty.elem {
					if args.len() != 1 { return false; }
					write!(w, "*const ").unwrap();
					print_path!(p_arg, None);
				} else { return false; }
			} else if let syn::Type::Array(a) = arg {
				if let syn::Type::Path(p_arg) = &*a.elem {
					let resolved = self.resolve_path(&p_arg.path);
					if !self.is_primitive(&resolved) { return false; }
					if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(len), .. }) = &a.len {
						if self.c_type_from_path(&format!("[{}; {}]", resolved, len.base10_digits()), is_ref, ptr_for_ref).is_none() { return false; }
						write!(w, "_{}{}", resolved, len.base10_digits()).unwrap();
						write!(mangled_type, "_{}{}", resolved, len.base10_digits()).unwrap();
					} else { return false; }
				} else { return false; }
			} else { return false; }
		}
		if self.is_transparent_container(ident, is_ref) { return true; }
		write!(w, "Z").unwrap();
		write!(mangled_type, "Z").unwrap();

		// Make sure the type is actually defined:
		self.check_create_container(String::from_utf8(mangled_type).unwrap(), ident, args, is_ref);
		true
	}
	fn print_c_mangled_container_path<W: std::io::Write>(&mut self, w: &mut W, args: Vec<&syn::Type>, ident: &str, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
		if !self.is_transparent_container(ident, is_ref) {
			write!(w, "{}::", Self::generated_container_path()).unwrap();
		}
		self.print_c_mangled_container_path_intern(w, args, ident, is_ref, is_mut, ptr_for_ref, false)
	}

	// **********************************
	// *** C Type Equivalent Printing ***
	// **********************************

	fn print_c_path_intern<W: std::io::Write>(&self, w: &mut W, path: &syn::Path, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
//eprintln!("pcpi ({} {} {}): {:?}", is_ref, is_mut, ptr_for_ref, path);
		let full_path = match self.maybe_resolve_path(&path) {
			Some(path) => path, None => return false };
		if let Some(c_type) = self.c_type_from_path(&full_path, is_ref, ptr_for_ref) {
			write!(w, "{}", c_type).unwrap();
			true
		} else if self.crate_types.traits.get(&full_path).is_some() {
			if is_ref && ptr_for_ref {
				write!(w, "*{} crate::{}", if is_mut { "mut" } else { "const" }, full_path).unwrap();
			} else if is_ref {
				write!(w, "&{}crate::{}", if is_mut { "mut " } else { "" }, full_path).unwrap();
			} else {
				write!(w, "crate::{}", full_path).unwrap();
			}
			true
		} else if self.crate_types.opaques.get(&full_path).is_some() || self.crate_types.mirrored_enums.get(&full_path).is_some() {
			if is_ref && ptr_for_ref {
				write!(w, "*{} crate::{}", if is_mut { "mut" } else { "const" }, full_path).unwrap();
			} else if is_ref {
				write!(w, "&{}crate::{}", if is_mut { "mut " } else { "" }, full_path).unwrap();
			} else {
				write!(w, "crate::{}", full_path).unwrap();
			}
			true
		} else {
			false
		}
	}
	fn print_c_type_intern<W: std::io::Write>(&mut self, generics: Option<&GenericTypes>, w: &mut W, t: &syn::Type, is_ref: bool, is_mut: bool, ptr_for_ref: bool) -> bool {
//eprintln!("pcti ({} {} {}): {:?}", is_ref, is_mut, ptr_for_ref, t);
		match t {
			syn::Type::Path(p) => {
				if p.qself.is_some() || p.path.leading_colon.is_some() {
					return false;
				}
				if let Some(gen_types) = generics {
					if let Some(resolved) = gen_types.maybe_resolve_path(&p.path) {
						if self.is_known_container(&resolved.0, is_ref) { return false; }
						if self.is_transparent_container(&resolved.0, is_ref) { return false; }
						return self.print_c_path_intern(w, &resolved.1, is_ref, is_mut, ptr_for_ref);
					}
				}
				if let Some(full_path) = self.maybe_resolve_path(&p.path) {
					if self.is_known_container(&full_path, is_ref) || self.is_transparent_container(&full_path, is_ref) {
						return self.print_c_mangled_container_path(w, Self::path_to_generic_args(&p.path), &full_path, is_ref, is_mut, ptr_for_ref);
					}
				}
				if p.path.leading_colon.is_some() { return false; }
				self.print_c_path_intern(w, &p.path, is_ref, is_mut, ptr_for_ref)
			},
			syn::Type::Reference(r) => {
				if let Some(lft) = &r.lifetime {
					if format!("{}", lft.ident) != "static" { return false; }
				}
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
							if let Some(ty) = self.c_type_from_path(&format!("[u8; {}]", i.base10_digits()), false, ptr_for_ref) {
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
					if self.is_primitive(&resolved) {
						write!(w, "{}::{}slice", Self::container_templ_path(), resolved).unwrap();
						true
					} else { false }
				} else if let syn::Type::Reference(r) = &*s.elem {
					if let syn::Type::Path(p) = &*r.elem {
						let resolved = self.resolve_path(&p.path);
						let mangled_container = if let Some(ident) = self.crate_types.opaques.get(&resolved) {
							format!("C{}Slice", ident)
						} else if let Some(en) = self.crate_types.mirrored_enums.get(&resolved) {
							format!("C{}Slice", en.ident)
						} else if let Some(id) = p.path.get_ident() {
							format!("C{}Slice", id)
						} else { return false; };
						write!(w, "{}::{}", Self::generated_container_path(), mangled_container).unwrap();
						self.check_create_container(mangled_container, "Slice", vec![&*r.elem], true);
						true
					} else { false }
				} else { false }
			},
			syn::Type::Tuple(t) => {
				if t.elems.len() == 0 {
					true
				} else {
					self.print_c_mangled_container_path(w, t.elems.iter().collect(),
						&format!("{}Tuple", t.elems.len()), is_ref, is_mut, ptr_for_ref)
				}
			},
			_ => false,
		}
	}
	pub fn print_c_type<W: std::io::Write>(&mut self, w: &mut W, t: &syn::Type, generics: Option<&GenericTypes>, ptr_for_ref: bool) {
		assert!(self.print_c_type_intern(generics, w, t, false, false, ptr_for_ref));
	}
	pub fn understood_c_path(&mut self, p: &syn::Path) -> bool {
		if p.leading_colon.is_some() { return false; }
		self.print_c_path_intern(&mut std::io::sink(), p, false, false, false)
	}
	pub fn understood_c_type(&mut self, t: &syn::Type, generics: Option<&GenericTypes>) -> bool {
		self.print_c_type_intern(generics, &mut std::io::sink(), t, false, false, false)
	}
}
