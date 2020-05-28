use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;

use proc_macro2::{TokenTree, TokenStream, Span};

mod types;
use types::*;

/// Because we don't expand macros, any code that we need to generated based on their contents has
/// to be completely manual. In this case its all just serialization, so its not too hard.
fn convert_macro<W: std::io::Write>(w: &mut W, macro_path: &syn::Path, stream: &TokenStream, types: &TypeResolver) {
	assert_eq!(macro_path.segments.len(), 1);
	match &format!("{}", macro_path.segments.iter().next().unwrap().ident) as &str {
		"impl_writeable" | "impl_writeable_len_match" => {
			let struct_for = if let TokenTree::Ident(i) = stream.clone().into_iter().next().unwrap() { i } else { unimplemented!(); };
			if let Some(s) = types.maybe_resolve_ident(&struct_for) {
				if !types.crate_types.opaques.get(&s).is_some() { return; }
				writeln!(w, "#[no_mangle]").unwrap();
				writeln!(w, "pub extern \"C\" fn {}_write(obj: *const {}) -> crate::c_types::derived::CVec_u8Z {{", struct_for, struct_for).unwrap();
				writeln!(w, "\tcrate::c_types::serialize_obj(unsafe {{ &(*(*obj).inner) }})").unwrap();
				writeln!(w, "}}").unwrap();
				writeln!(w, "#[no_mangle]").unwrap();
				writeln!(w, "pub extern \"C\" fn {}_read(ser: crate::c_types::u8slice) -> {} {{", struct_for, struct_for).unwrap();
				writeln!(w, "\tif let Ok(res) = crate::c_types::deserialize_obj(ser) {{").unwrap();
				writeln!(w, "\t\t{} {{ inner: Box::into_raw(Box::new(res)), _underlying_ref: false }}", struct_for).unwrap();
				writeln!(w, "\t}} else {{").unwrap();
				writeln!(w, "\t\t{} {{ inner: std::ptr::null(), _underlying_ref: false }}", struct_for).unwrap();
				writeln!(w, "\t}}\n}}").unwrap();
			}
		},
		_ => {},
	}
}

/// Manually mapped trait impls
fn maybe_convert_trait_impl<W: std::io::Write>(w: &mut W, trait_path: &syn::Path, for_obj: &syn::Ident, types: &TypeResolver) {
	if let Some(t) = types.maybe_resolve_path(&trait_path) {
		let s = types.maybe_resolve_ident(for_obj).unwrap();
		if !types.crate_types.opaques.get(&s).is_some() { return; }
		match &t as &str {
			"util::ser::Writeable" => {
				writeln!(w, "#[no_mangle]").unwrap();
				writeln!(w, "pub extern \"C\" fn {}_write(obj: *const {}) -> crate::c_types::derived::CVec_u8Z {{", for_obj, for_obj).unwrap();
				writeln!(w, "\tcrate::c_types::serialize_obj(unsafe {{ &(*(*obj).inner) }})").unwrap();
				writeln!(w, "}}").unwrap();
			},
			"util::ser::Readable" => {
				writeln!(w, "#[no_mangle]").unwrap();
				writeln!(w, "pub extern \"C\" fn {}_read(ser: crate::c_types::u8slice) -> {} {{", for_obj, for_obj).unwrap();
				writeln!(w, "\tif let Ok(res) = crate::c_types::deserialize_obj(ser) {{").unwrap();
				writeln!(w, "\t\t{} {{ inner: Box::into_raw(Box::new(res)), _underlying_ref: false }}", for_obj).unwrap();
				writeln!(w, "\t}} else {{").unwrap();
				writeln!(w, "\t\t{} {{ inner: std::ptr::null(), _underlying_ref: false }}", for_obj).unwrap();
				writeln!(w, "\t}}\n}}").unwrap();
			},
			_ => {},
		}
	}
}

/// Prints the docs from a given attribute list unless its tagged no export
fn println_docs<W: std::io::Write>(w: &mut W, attrs: &[syn::Attribute], prefix: &str) {
	for attr in attrs.iter() {
		let tokens_clone = attr.tokens.clone();
		let mut token_iter = tokens_clone.into_iter();
		if let Some(token) = token_iter.next() {
			match token {
				TokenTree::Punct(c) if c.as_char() == '=' => {
					// Really not sure where syn gets '=' from here -
					// it somehow represents '///' or '//!'
				},
				TokenTree::Group(_) => continue, // eg #[derive()]
				_ => unimplemented!(),
			}
		} else { continue; }
		match attr.style {
			syn::AttrStyle::Inner(_) => {
				match token_iter.next().unwrap() {
					TokenTree::Literal(lit) => {
						// TODO: This adds ""s around lit, which we don't want:
						writeln!(w, "{}//! {}", prefix, lit).unwrap();
					},
					_ => unimplemented!(),
				}
			},
			syn::AttrStyle::Outer => {
				match token_iter.next().unwrap() {
					TokenTree::Literal(lit) => {
						// TODO: This adds ""s around lit, which we don't want:
						writeln!(w, "{}/// {}", prefix, lit).unwrap();
					},
					_ => unimplemented!(),
				}
			},
		}
	}
}

fn print_method_params<W: std::io::Write>(w: &mut W, sig: &syn::Signature, associated_types: &HashMap<&syn::Ident, &syn::Ident>, this_param: &str, types: &mut TypeResolver, generics: Option<&GenericTypes>, self_ptr: bool, fn_decl: bool) {
	if sig.constness.is_some() || sig.asyncness.is_some() || sig.unsafety.is_some() ||
			sig.abi.is_some() || sig.variadic.is_some() {
		unimplemented!();
	}
	if sig.generics.lt_token.is_some() {
		for generic in sig.generics.params.iter() {
			match generic {
				syn::GenericParam::Type(_) => {
					// We ignore these, if they're not on skipped args, we'll blow up
					// anyway
				},
				_ => unimplemented!(),
			}
		}
	}

	let mut first_arg = true;
	let mut num_unused = 0;
	for inp in sig.inputs.iter() {
		match inp {
			syn::FnArg::Receiver(recv) => {
				if !recv.attrs.is_empty() || recv.reference.is_none() { unimplemented!(); }
				if recv.reference.as_ref().unwrap().1.is_some() { unimplemented!(); }
				write!(w, "this_arg: {}{}",
					match (self_ptr, recv.mutability.is_some()) {
						(true, true) => "*mut ",
						(true, false) => "*const ",
						(false, true) => "&mut ",
						(false, false) => "&",
					}, this_param).unwrap();
				assert!(first_arg);
				first_arg = false;
			},
			syn::FnArg::Typed(arg) => {
				if types.skip_arg(&*arg.ty, generics) { continue; }
				if !arg.attrs.is_empty() { unimplemented!(); }
				let is_ref = if let syn::Type::Reference(_) = *arg.ty { true } else { false };
				match &*arg.pat {
					syn::Pat::Ident(ident) => {
						if !ident.attrs.is_empty() || ident.subpat.is_some() {
							unimplemented!();
						}
						write!(w, "{}{}{}: ", if first_arg { "" } else { ", " }, if is_ref || !fn_decl { "" } else { "mut " }, ident.ident).unwrap();
						first_arg = false;
					},
					syn::Pat::Wild(wild) => {
						if !wild.attrs.is_empty() { unimplemented!(); }
						write!(w, "{}unused_{}: ", if first_arg { "" } else { ", " }, num_unused).unwrap();
						num_unused += 1;
					},
					_ => unimplemented!(),
				}
				types.print_c_type(w, &*arg.ty, generics, false);
			}
		}
	}
	write!(w, ")").unwrap();
	match &sig.output {
		syn::ReturnType::Type(_, rtype) => {
			write!(w, " -> ").unwrap();
			if let Some(mut remaining_path) = first_seg_self(&*rtype) {
				if let Some(associated_seg) = get_single_remaining_path_seg(&mut remaining_path) {
					// We're returning an associated type in a trait impl. Its probably a safe bet
					// that its also a trait, so just return the trait type.
					let real_type = associated_types.get(associated_seg).unwrap();
					types.print_c_type(w, &syn::Type::Path(syn::TypePath { qself: None,
						path: syn::PathSegment {
							ident: (*real_type).clone(),
							arguments: syn::PathArguments::None
						}.into()
					}), generics, true);
				} else {
					write!(w, "{}", this_param).unwrap();
				}
			} else {
				if let syn::Type::Reference(r) = &**rtype {
					// We can't return a reference, cause we allocate things on the stack.
					types.print_c_type(w, &*r.elem, generics, true);
				} else {
					types.print_c_type(w, &*rtype, generics, true);
				}
			}
		},
		_ => {},
	}
}

fn print_method_var_decl_body<W: std::io::Write>(w: &mut W, sig: &syn::Signature, extra_indent: &str, types: &TypeResolver, generics: Option<&GenericTypes>, to_c: bool) {
	let mut num_unused = 0;
	for inp in sig.inputs.iter() {
		match inp {
			syn::FnArg::Receiver(_) => {},
			syn::FnArg::Typed(arg) => {
				if types.skip_arg(&*arg.ty, generics) { continue; }
				if !arg.attrs.is_empty() { unimplemented!(); }
				macro_rules! write_new_var {
					($ident: expr, $ty: expr) => {
						if to_c {
							if types.print_to_c_conversion_new_var(w, &$ident, &$ty, generics) {
								write!(w, "\n\t{}", extra_indent).unwrap();
							}
						} else {
							if types.print_from_c_conversion_new_var(w, &$ident, &$ty, generics) {
								write!(w, "\n\t{}", extra_indent).unwrap();
							}
						}
					}
				}
				match &*arg.pat {
					syn::Pat::Ident(ident) => {
						if !ident.attrs.is_empty() || ident.subpat.is_some() {
							unimplemented!();
						}
						write_new_var!(ident.ident, *arg.ty);
					},
					syn::Pat::Wild(w) => {
						if !w.attrs.is_empty() { unimplemented!(); }
						write_new_var!(syn::Ident::new(&format!("unused_{}", num_unused), Span::call_site()), *arg.ty);
						num_unused += 1;
					},
					_ => unimplemented!(),
				}
			}
		}
	}
	match &sig.output {
		syn::ReturnType::Type(_, _) => {
			write!(w, "let mut ret = ").unwrap();
		},
		_ => {},
	}
}

fn print_method_call_params<W: std::io::Write>(w: &mut W, sig: &syn::Signature, associated_types: &HashMap<&syn::Ident, &syn::Ident>, extra_indent: &str, types: &TypeResolver, generics: Option<&GenericTypes>, this_type: &str, to_c: bool) {
	let mut first_arg = true;
	let mut num_unused = 0;
	for inp in sig.inputs.iter() {
		match inp {
			syn::FnArg::Receiver(recv) => {
				if !recv.attrs.is_empty() || recv.reference.is_none() { unimplemented!(); }
				if recv.reference.as_ref().unwrap().1.is_some() { unimplemented!(); }
				if to_c {
					write!(w, "self.this_arg").unwrap();
					first_arg = false;
				}
			},
			syn::FnArg::Typed(arg) => {
				if types.skip_arg(&*arg.ty, generics) {
					if !to_c {
						if !first_arg {
							write!(w, ", ").unwrap();
						}
						first_arg = false;
						types.no_arg_to_rust(w, &*arg.ty, generics);
					}
					continue;
				}
				if !arg.attrs.is_empty() { unimplemented!(); }
				macro_rules! write_ident {
					($ident: expr) => {
						if !first_arg {
							write!(w, ", ").unwrap();
						}
						first_arg = false;
						if to_c {
							types.print_to_c_conversion_inline_prefix(w, &*arg.ty, generics, false);
							write!(w, "{}", $ident).unwrap();
							types.print_to_c_conversion_inline_suffix(w, &*arg.ty, generics, false);
						} else {
							types.print_from_c_conversion_prefix(w, &*arg.ty, generics);
							write!(w, "{}", $ident).unwrap();
							types.print_from_c_conversion_suffix(w, &*arg.ty, generics);
						}
					}
				}
				match &*arg.pat {
					syn::Pat::Ident(ident) => {
						if !ident.attrs.is_empty() || ident.subpat.is_some() {
							unimplemented!();
						}
						write_ident!(ident.ident);
					},
					syn::Pat::Wild(w) => {
						if !w.attrs.is_empty() { unimplemented!(); }
						write_ident!(format!("unused_{}", num_unused));
						num_unused += 1;
					},
					_ => unimplemented!(),
				}
			}
		}
	}
	write!(w, ")").unwrap();
	match &sig.output {
		syn::ReturnType::Type(_, rtype) => {
			write!(w, ";\n\t{}", extra_indent).unwrap();

			if to_c && first_seg_self(&*rtype).is_some() {
				// Assume rather blindly that we're returning an associated trait from a C fn call to a Rust trait object.
				write!(w, "ret").unwrap();
			} else if !to_c && first_seg_self(&*rtype).is_some() {
				if let Some(mut remaining_path) = first_seg_self(&*rtype) {
					if let Some(associated_seg) = get_single_remaining_path_seg(&mut remaining_path) {
						let real_type = associated_types.get(associated_seg).unwrap();
						if let Some(t) = types.crate_types.traits.get(&types.maybe_resolve_ident(&real_type).unwrap()) {
							// We're returning an associated trait from a Rust fn call to a C trait
							// object.
							writeln!(w, "{} {{\n\t\t{}this_arg: Box::into_raw(Box::new(ret)) as *mut c_void,", t.ident, extra_indent).unwrap();
							for i in t.items.iter() {
								match i {
									syn::TraitItem::Method(m) => {
										if let ExportStatus::Export = export_status(&m.attrs) {
											if let syn::ReturnType::Type(_, rtype) = &m.sig.output {
												if let syn::Type::Reference(r) = &**rtype {
													write!(w, "\t\t{}{}: ", extra_indent, m.sig.ident).unwrap();
													types.print_empty_rust_val(w, &*r.elem);
													writeln!(w, ",\n\t\t{}set_{}: Some({}_{}_set_{}),", extra_indent, m.sig.ident, this_type, real_type, m.sig.ident).unwrap();
													continue;
												}
											}
											writeln!(w, "\t\t{}{}: {}_{}_{},", extra_indent, m.sig.ident, this_type, real_type, m.sig.ident).unwrap();
										}
									},
									_ => {},
								}
							}
							write!(w, "\t{}}}", extra_indent).unwrap();
							return;
						}
					}
				}
				write!(w, "{} {{ inner: Box::into_raw(Box::new(ret)), _underlying_ref: false }}", this_type).unwrap();
			} else if to_c {
				let new_var = types.print_from_c_conversion_new_var(w, &syn::Ident::new("ret", Span::call_site()), rtype, generics);
				if new_var {
					write!(w, "\n\t{}", extra_indent).unwrap();
				}
				types.print_from_c_conversion_prefix(w, &*rtype, generics);
				write!(w, "ret").unwrap();
				types.print_from_c_conversion_suffix(w, &*rtype, generics);
			} else if let syn::Type::Reference(r) = &**rtype {
				if let syn::Type::Reference(_) = &**rtype {
					write!(w, "let mut ret = unsafe {{ (*ret).clone() }};\n\t{}", extra_indent).unwrap();
				}
				let new_var = types.print_to_c_conversion_new_var(w, &syn::Ident::new("ret", Span::call_site()), &r.elem, generics);
				if new_var {
					write!(w, "\n\t{}", extra_indent).unwrap();
				}
				types.print_to_c_conversion_inline_prefix(w, &r.elem, generics, true);
				write!(w, "ret").unwrap();
				types.print_to_c_conversion_inline_suffix(w, &r.elem, generics, true);
			} else {
				let new_var = types.print_to_c_conversion_new_var(w, &syn::Ident::new("ret", Span::call_site()), rtype, generics);
				if new_var {
					write!(w, "\n\t{}", extra_indent).unwrap();
				}
				types.print_to_c_conversion_inline_prefix(w, &*rtype, generics, true);
				write!(w, "ret").unwrap();
				types.print_to_c_conversion_inline_suffix(w, &*rtype, generics, true);
			}
		}
		_ => {},
	}
}

fn maybe_print_generics<W: std::io::Write>(w: &mut W, generics: &syn::Generics, types: &TypeResolver, concrete_lifetimes: bool) {
	let mut gen_types = GenericTypes::new();
	assert!(gen_types.learn_generics(generics, types));
	if !generics.params.is_empty() {
		write!(w, "<").unwrap();
		for (idx, generic) in generics.params.iter().enumerate() {
			match generic {
				syn::GenericParam::Type(type_param) => {
					let bound = type_param.bounds.iter().next().unwrap();
					if let syn::TypeParamBound::Trait(trait_bound) = bound {
						assert_simple_bound(&trait_bound);
						write!(w, "{}{}", if idx != 0 { ", " } else { "" }, gen_types.maybe_resolve_ident(&type_param.ident).unwrap()).unwrap();
					}
				},
				syn::GenericParam::Lifetime(lt) => {
					if concrete_lifetimes {
						write!(w, "'static").unwrap();
					} else {
						write!(w, "{}'{}", if idx != 0 { ", " } else { "" }, lt.lifetime.ident).unwrap();
					}
				},
				_ => unimplemented!(),
			}
		}
		write!(w, ">").unwrap();
	}
}

macro_rules! walk_supertraits { ($t: expr, $types: expr, ($( $pat: pat => $e: expr),*) ) => { {
	if $t.colon_token.is_some() {
		for st in $t.supertraits.iter() {
			match st {
				syn::TypeParamBound::Trait(supertrait) => {
					if supertrait.paren_token.is_some() || supertrait.lifetimes.is_some() {
						unimplemented!();
					}
					if let Some(ident) = supertrait.path.get_ident() {
						match (&format!("{}", ident) as &str, &ident) {
							$( $pat => $e, )*
						}
					} else {
						let path = $types.resolve_path(&supertrait.path);
						match (&path as &str, &supertrait.path.segments.iter().last().unwrap().ident) {
							$( $pat => $e, )*
						}
					}
				},
				syn::TypeParamBound::Lifetime(_) => unimplemented!(),
			}
		}
	}
} } }

fn learn_associated_types<'a>(t: &'a syn::ItemTrait) -> HashMap<&'a syn::Ident, &'a syn::Ident> {
	let mut associated_types = HashMap::new();
	for item in t.items.iter() {
		match item {
			&syn::TraitItem::Type(ref t) => {
				if t.default.is_some() || t.generics.lt_token.is_some() { unimplemented!(); }
				let mut bounds_iter = t.bounds.iter();
				match bounds_iter.next().unwrap() {
					syn::TypeParamBound::Trait(tr) => {
						assert_simple_bound(&tr);
						associated_types.insert(&t.ident, assert_single_path_seg(&tr.path));
					},
					_ => unimplemented!(),
				}
				if bounds_iter.next().is_some() { unimplemented!(); }
			},
			_ => {},
		}
	}
	associated_types
}

fn println_trait<'a, 'b, W: std::io::Write>(w: &mut W, t: &'a syn::ItemTrait, types: &mut TypeResolver<'b, 'a>, extra_headers: &mut File, cpp_headers: &mut File) {
	let trait_name = format!("{}", t.ident);
	match export_status(&t.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
	}
	println_docs(w, &t.attrs, "");

	walk_supertraits!(t, types, (
		("Clone", _) => writeln!(w, "#[derive(Clone)]").unwrap(),
		("std::cmp::Eq", _) => {},
		("std::hash::Hash", _) => {},
		("Send", _) => {}, ("Sync", _) => {},
		(s, _) => {
			if !s.starts_with("util::") { unimplemented!(); }
		}
	) );
	writeln!(w, "#[repr(C)]\npub struct {} {{", trait_name).unwrap();
	writeln!(w, "\tpub this_arg: *mut c_void,").unwrap();
	let associated_types = learn_associated_types(t);
	for item in t.items.iter() {
		match item {
			&syn::TraitItem::Method(ref m) => {
				match export_status(&m.attrs) {
					ExportStatus::NoExport => {
						writeln!(w, "\t//XXX: Need to export {}", m.sig.ident).unwrap();
						continue;
					},
					ExportStatus::Export => {},
					ExportStatus::TestOnly => continue,
				}
				if m.default.is_some() { unimplemented!(); }

				println_docs(w, &m.attrs, "\t");

				if let syn::ReturnType::Type(_, rtype) = &m.sig.output {
					if let syn::Type::Reference(r) = &**rtype {
						write!(w, "\tpub {}: ", m.sig.ident).unwrap();
						types.print_c_type(w, &*r.elem, None, false);
						writeln!(w, ",").unwrap();
						// TODO: free field!
						writeln!(w, "\t/// Fill in the {} field as a reference to it will be given to Rust after this returns", m.sig.ident).unwrap();
						writeln!(w, "\t/// Note that this takes a pointer to this object, not the this_ptr like other methods do").unwrap();
						writeln!(w, "\tpub set_{}: Option<extern \"C\" fn(&{})>,", m.sig.ident, trait_name).unwrap();
						// Note that cbindgen will now generate
						// typedef struct Thing {..., set_thing: (const Thing*), ...} Thing;
						// which does not compile since Thing is not defined before it is used.
						writeln!(extra_headers, "struct LDK{};", trait_name).unwrap();
						writeln!(extra_headers, "typedef struct LDK{} LDK{};", trait_name, trait_name).unwrap();
						continue;
					}
					// Sadly, this currently doesn't do what we want, but it should be easy to get
					// cbindgen to support it. See https://github.com/eqrion/cbindgen/issues/531
					writeln!(w, "\t#[must_use]").unwrap();
				}

				write!(w, "\tpub {}: extern \"C\" fn (", m.sig.ident).unwrap();
				print_method_params(w, &m.sig, &associated_types, "c_void", types, None, true, false);
				writeln!(w, ",").unwrap();
			},
			&syn::TraitItem::Type(_) => {},
			_ => unimplemented!(),
		}
	}
	walk_supertraits!(t, types, (
		("Clone", _) => {},
		("std::cmp::Eq", _) => writeln!(w, "\tpub eq: extern \"C\" fn (this_arg: *const c_void, other_arg: *const c_void) -> bool,").unwrap(),
		("std::hash::Hash", _) => writeln!(w, "\tpub hash: extern \"C\" fn (this_arg: *const c_void) -> u64,").unwrap(),
		("Send", _) => {}, ("Sync", _) => {},
		(s, i) => {
			if !s.starts_with("util::") { unimplemented!(); }
			writeln!(w, "\tpub {}: crate::{},", i, s).unwrap();
		}
	) );
	writeln!(w, "}}").unwrap();
	walk_supertraits!(t, types, (
		("Send", _) => writeln!(w, "unsafe impl Send for {} {{}}", trait_name).unwrap(),
		("Sync", _) => writeln!(w, "unsafe impl Sync for {} {{}}", trait_name).unwrap(),
		("std::cmp::Eq", _) => {
			writeln!(w, "impl std::cmp::Eq for {} {{}}", trait_name).unwrap();
			writeln!(w, "impl std::cmp::PartialEq for {} {{", trait_name).unwrap();
			writeln!(w, "\tfn eq(&self, o: &Self) -> bool {{ (self.eq)(self.this_arg, o.this_arg) }}\n}}").unwrap();
		},
		("std::hash::Hash", _) => {
			writeln!(w, "impl std::hash::Hash for {} {{", trait_name).unwrap();
			writeln!(w, "\tfn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {{ hasher.write_u64((self.hash)(self.this_arg)) }}\n}}").unwrap();
		},
		("Clone", _) => {},
		(s, i) => {
			if s != "util::events::MessageSendEventsProvider" { unimplemented!(); }
			// We straight-up cheat here - instead of bothering to get the trait object we just
			// print what we need since this is only used in one place.
			writeln!(w, "impl lightning::{} for {} {{", s, trait_name).unwrap();
			writeln!(w, "\tfn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {{").unwrap();
			writeln!(w, "\t\t<crate::{} as lightning::{}>::get_and_clear_pending_msg_events(&self.{})", s, s, i).unwrap();
			writeln!(w, "\t}}\n}}").unwrap();
		}
	) );
	writeln!(w, "\nuse {}::{}::{} as ln{};", types.orig_crate, types.module_path, t.ident, trait_name).unwrap();
	write!(w, "impl ln{}", t.ident).unwrap();
	maybe_print_generics(w, &t.generics, types, false);
	writeln!(w, " for {} {{", trait_name).unwrap();
	for item in t.items.iter() {
		match item {
			syn::TraitItem::Method(m) => {
				if let ExportStatus::TestOnly = export_status(&m.attrs) { continue; }
				if m.default.is_some() { unimplemented!(); }
				if m.sig.constness.is_some() || m.sig.asyncness.is_some() || m.sig.unsafety.is_some() ||
						m.sig.abi.is_some() || m.sig.variadic.is_some() {
					unimplemented!();
				}
				write!(w, "\tfn {}", m.sig.ident).unwrap();
				types.print_rust_generic_param(w, m.sig.generics.params.iter());
				write!(w, "(").unwrap();
				for inp in m.sig.inputs.iter() {
					match inp {
						syn::FnArg::Receiver(recv) => {
							if !recv.attrs.is_empty() || recv.reference.is_none() { unimplemented!(); }
							write!(w, "&").unwrap();
							if let Some(lft) = &recv.reference.as_ref().unwrap().1 {
								write!(w, "'{} ", lft.ident).unwrap();
							}
							if recv.mutability.is_some() {
								write!(w, "mut self").unwrap();
							} else {
								write!(w, "self").unwrap();
							}
						},
						syn::FnArg::Typed(arg) => {
							if !arg.attrs.is_empty() { unimplemented!(); }
							match &*arg.pat {
								syn::Pat::Ident(ident) => {
									if !ident.attrs.is_empty() || ident.by_ref.is_some() ||
											ident.mutability.is_some() || ident.subpat.is_some() {
										unimplemented!();
									}
									write!(w, ", {}{}: ", if types.skip_arg(&*arg.ty, None) { "_" } else { "" }, ident.ident).unwrap();
								}
								_ => unimplemented!(),
							}
							types.print_rust_type(w, &*arg.ty);
						}
					}
				}
				write!(w, ")").unwrap();
				match &m.sig.output {
					syn::ReturnType::Type(_, rtype) => {
						write!(w, " -> ").unwrap();
						types.print_rust_type(w, &*rtype)
					},
					_ => {},
				}
				write!(w, " {{\n\t\t").unwrap();
				match export_status(&m.attrs) {
					ExportStatus::NoExport => {
						writeln!(w, "unimplemented!();\n\t}}").unwrap();
						continue;
					},
					_ => {},
				}
				if let syn::ReturnType::Type(_, rtype) = &m.sig.output {
					if let syn::Type::Reference(r) = &**rtype {
						assert_eq!(m.sig.inputs.len(), 1); // Must only take self!
						writeln!(w, "if let Some(f) = self.set_{} {{", m.sig.ident).unwrap();
						writeln!(w, "\t\t\t(f)(self);").unwrap();
						write!(w, "\t\t}}\n\t\t").unwrap();
						types.print_from_c_conversion_to_ref_prefix(w, &*r.elem, None);
						write!(w, "self.{}", m.sig.ident).unwrap();
						types.print_from_c_conversion_to_ref_suffix(w, &*r.elem, None);
						writeln!(w, "\n\t}}").unwrap();
						continue;
					}
				}
				print_method_var_decl_body(w, &m.sig, "\t", types, None, true);
				write!(w, "(self.{})(", m.sig.ident).unwrap();
				print_method_call_params(w, &m.sig, &associated_types, "\t", types, None, "", true);

				writeln!(w, "\n\t}}").unwrap();
			},
			&syn::TraitItem::Type(ref t) => {
				if t.default.is_some() || t.generics.lt_token.is_some() { unimplemented!(); }
				let mut bounds_iter = t.bounds.iter();
				match bounds_iter.next().unwrap() {
					syn::TypeParamBound::Trait(tr) => {
						writeln!(w, "\ttype {} = crate::{};", t.ident, types.resolve_path(&tr.path)).unwrap();
					},
					_ => unimplemented!(),
				}
				if bounds_iter.next().is_some() { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}
	writeln!(w, "}}\n").unwrap();
	writeln!(w, "// We're essentially a pointer already, or at least a set of pointers, so allow us to be used").unwrap();
	writeln!(w, "// directly as a Deref trait in higher-level structs:").unwrap();
	writeln!(w, "impl std::ops::Deref for {} {{\n\ttype Target = Self;", trait_name).unwrap();
	writeln!(w, "\tfn deref(&self) -> &Self {{\n\t\tself\n\t}}\n}}").unwrap();

	// TODO: We should really define an abstract base class instead of a typedef here, but that
	// requires a ton of tweaking, sadly.
	writeln!(cpp_headers, "typedef LDK{} {};", trait_name, trait_name).unwrap();
	types.trait_declared(&t.ident, t);
}

fn println_opaque<W: std::io::Write>(w: &mut W, ident: &syn::Ident, struct_name: &str, generics: &syn::Generics, attrs: &[syn::Attribute], types: &TypeResolver, extra_headers: &mut File, cpp_headers: &mut File) {
	// If we directly read the original type by its original name, cbindgen hits
	// https://github.com/eqrion/cbindgen/issues/286 Thus, instead, we import it as a temporary
	// name and then reference it by that name, which works around the issue.
	write!(w, "\nuse {}::{}::{} as ln{}Import;\ntype ln{} = ln{}Import", types.orig_crate, types.module_path, ident, ident, ident, ident).unwrap();
	maybe_print_generics(w, &generics, &types, true);
	writeln!(w, ";\n").unwrap();
	writeln!(extra_headers, "struct ln{}Opaque;\ntypedef struct ln{}Opaque LDKln{};", ident, ident, ident).unwrap();
	println_docs(w, &attrs, "");
	writeln!(w, "#[must_use]\n#[repr(C)]\npub struct {} {{\n\t/// Nearly everyhwere, inner must be non-null, however in places where", struct_name).unwrap();
	writeln!(w, "\t///the Rust equivalent takes an Option, it may be set to null to indicate None.").unwrap();
	writeln!(w, "\tpub inner: *const ln{},\n\tpub _underlying_ref: bool,\n}}\n", ident).unwrap();
	writeln!(w, "impl Drop for {} {{\n\tfn drop(&mut self) {{", struct_name).unwrap();
	writeln!(w, "\t\tif !self._underlying_ref && !self.inner.is_null() {{").unwrap();
	writeln!(w, "\t\t\tlet _ = unsafe {{ Box::from_raw(self.inner as *mut ln{}) }};\n\t\t}}\n\t}}\n}}", struct_name).unwrap();
	writeln!(w, "#[no_mangle]\npub extern \"C\" fn {}_free(this_ptr: {}) {{ }}", struct_name, struct_name).unwrap();

	'attr_loop: for attr in attrs.iter() {
		let tokens_clone = attr.tokens.clone();
		let mut token_iter = tokens_clone.into_iter();
		if let Some(token) = token_iter.next() {
			match token {
				TokenTree::Group(g) => {
					if format!("{}", single_ident_generic_path_to_ident(&attr.path).unwrap()) == "derive" {
						for id in g.stream().into_iter() {
							if let TokenTree::Ident(i) = id {
								if i == "Clone" {
									writeln!(w, "impl Clone for {} {{", struct_name).unwrap();
									writeln!(w, "\tfn clone(&self) -> Self {{").unwrap();
									writeln!(w, "\t\tSelf {{").unwrap();
									writeln!(w, "\t\t\tinner: Box::into_raw(Box::new(unsafe {{ &*self.inner }}.clone())),").unwrap();
									writeln!(w, "\t\t\t_underlying_ref: false,").unwrap();
									writeln!(w, "\t\t}}\n\t}}\n}}").unwrap();
									break 'attr_loop;
								}
							}
						}
					}
				},
				_ => {},
			}
		}
	}

	writeln!(cpp_headers, "class {} {{\nprivate:", ident).unwrap();
	writeln!(cpp_headers, "\tLDK{} self;", ident).unwrap();
	writeln!(cpp_headers, "public:").unwrap();
	writeln!(cpp_headers, "\t{}(const {}&) = delete;", ident, ident).unwrap();
	writeln!(cpp_headers, "\t~{}() {{ {}_free(self); }}", ident, ident).unwrap();
	writeln!(cpp_headers, "\t{}({}&& o) : self(o.self) {{ o.self.inner = NULL; }}", ident, ident).unwrap();
	writeln!(cpp_headers, "\t{}(LDK{}&& m_self) : self(m_self) {{ m_self.inner = NULL; }}", ident, ident).unwrap();
	writeln!(cpp_headers, "\toperator LDK{}() {{ LDK{} res = self; self.inner = NULL; return res; }}", ident, ident).unwrap();
	writeln!(cpp_headers, "\tLDK{}* operator &() {{ return &self; }}", ident).unwrap();
	writeln!(cpp_headers, "\tLDK{}* operator ->() {{ return &self; }}", ident).unwrap();
	writeln!(cpp_headers, "}};").unwrap();
}

fn println_struct<'a, 'b, W: std::io::Write>(w: &mut W, s: &'a syn::ItemStruct, types: &mut TypeResolver<'b, 'a>, extra_headers: &mut File, cpp_headers: &mut File) {
	let struct_name = &format!("{}", s.ident);
	let export = export_status(&s.attrs);
	match export {
		ExportStatus::Export => {},
		ExportStatus::TestOnly => return,
		ExportStatus::NoExport => {
			types.struct_ignored(&s.ident);
			return;
		}
	}

	println_opaque(w, &s.ident, struct_name, &s.generics, &s.attrs, types, extra_headers, cpp_headers);

	eprintln!("exporting fields for {}", struct_name);
	if let syn::Fields::Named(fields) = &s.fields {
		let mut gen_types = GenericTypes::new();
		assert!(gen_types.learn_generics(&s.generics, types));

		let mut all_fields_settable = true;
		for field in fields.named.iter() {
			if let syn::Visibility::Public(_) = field.vis {
				let export = export_status(&field.attrs);
				match export {
					ExportStatus::Export => {},
					ExportStatus::NoExport|ExportStatus::TestOnly => {
						all_fields_settable = false;
						continue
					},
				}

				if let Some(ident) = &field.ident {
					let ref_type = syn::Type::Reference(syn::TypeReference {
						and_token: syn::Token!(&)(Span::call_site()), lifetime: None, mutability: None,
						elem: Box::new(field.ty.clone()) });
					if types.understood_c_type(&ref_type, Some(&gen_types)) {
						println_docs(w, &field.attrs, "");
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_get_{}(this_ptr: &{}) -> ", struct_name, ident, struct_name).unwrap();
						types.print_c_type(w, &ref_type, Some(&gen_types), true);
						write!(w, " {{\n\tlet inner_val = &unsafe {{ &*this_ptr.inner }}.{};\n\t", ident).unwrap();
						let local_var = types.print_to_c_conversion_new_var(w, &syn::Ident::new("inner_val", Span::call_site()), &ref_type, Some(&gen_types));
						if local_var { write!(w, "\n\t").unwrap(); }
						types.print_to_c_conversion_inline_prefix(w, &ref_type, Some(&gen_types), true);
						if local_var {
							write!(w, "inner_val").unwrap();
						} else {
							write!(w, "(*inner_val)").unwrap();
						}
						types.print_to_c_conversion_inline_suffix(w, &ref_type, Some(&gen_types), true);
						writeln!(w, "\n}}").unwrap();
					}

					if types.understood_c_type(&field.ty, Some(&gen_types)) {
						println_docs(w, &field.attrs, "");
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_set_{}(this_ptr: &mut {}, mut val: ", struct_name, ident, struct_name).unwrap();
						types.print_c_type(w, &field.ty, Some(&gen_types), false);
						write!(w, ") {{\n\t").unwrap();
						let local_var = types.print_from_c_conversion_new_var(w, &syn::Ident::new("val", Span::call_site()), &field.ty, Some(&gen_types));
						if local_var { write!(w, "\n\t").unwrap(); }
						write!(w, "unsafe {{ &mut *(this_ptr.inner as *mut ln{}) }}.{} = ", s.ident, ident).unwrap();
						types.print_from_c_conversion_prefix(w, &field.ty, Some(&gen_types));
						write!(w, "val").unwrap();
						types.print_from_c_conversion_suffix(w, &field.ty, Some(&gen_types));
						writeln!(w, ";\n}}").unwrap();
					} else { all_fields_settable = false; }
				} else { all_fields_settable = false; }
			} else { all_fields_settable = false; }
		}

		if all_fields_settable {
			// Build a constructor!
			write!(w, "#[must_use]\n#[no_mangle]\npub extern \"C\" fn {}_new(", struct_name).unwrap();
			for (idx, field) in fields.named.iter().enumerate() {
				if idx != 0 { write!(w, ", ").unwrap(); }
				write!(w, "mut {}_arg: ", field.ident.as_ref().unwrap()).unwrap();
				types.print_c_type(w, &field.ty, Some(&gen_types), false);
			}
			write!(w, ") -> {} {{\n\t", struct_name).unwrap();
			for field in fields.named.iter() {
				let field_name = format!("{}_arg", field.ident.as_ref().unwrap());
				if types.print_from_c_conversion_new_var(w, &syn::Ident::new(&field_name, Span::call_site()), &field.ty, Some(&gen_types)) {
					write!(w, "\n\t").unwrap();
				}
			}
			writeln!(w, "{} {{ inner: Box::into_raw(Box::new(ln{} {{", struct_name, s.ident).unwrap();
			for field in fields.named.iter() {
				write!(w, "\t\t{}: ", field.ident.as_ref().unwrap()).unwrap();
				types.print_from_c_conversion_prefix(w, &field.ty, Some(&gen_types));
				write!(w, "{}_arg", field.ident.as_ref().unwrap()).unwrap();
				types.print_from_c_conversion_suffix(w, &field.ty, Some(&gen_types));
				writeln!(w, ",").unwrap();
			}
			writeln!(w, "\t}})), _underlying_ref: false }}\n}}").unwrap();
		}
	}

	types.struct_imported(&s.ident, struct_name.clone());
}

fn println_impl<W: std::io::Write>(w: &mut W, i: &syn::ItemImpl, types: &mut TypeResolver) {
	if let &syn::Type::Path(ref p) = &*i.self_ty {
		if p.qself.is_some() { unimplemented!(); }
		if let Some(ident) = single_ident_generic_path_to_ident(&p.path) {
			if let Some(resolved_path) = types.maybe_resolve_non_ignored_ident(&ident) {
				let mut gen_types = GenericTypes::new();
				if !gen_types.learn_generics(&i.generics, types) {
					eprintln!("Not implementing anything for impl {} due to not understood generics", ident);
					return;
				}

				if i.defaultness.is_some() || i.unsafety.is_some() { unimplemented!(); }
				if let Some(trait_path) = i.trait_.as_ref() {
					if trait_path.0.is_some() { unimplemented!(); }
					if types.understood_c_path(&trait_path.1) {
eprintln!("WIP: IMPL {:?} FOR {}", trait_path.1, ident);
						let full_trait_path = types.resolve_path(&trait_path.1);
						let trait_obj = *types.crate_types.traits.get(&full_trait_path).unwrap();
						// We learn the associated types maping from the original trait object.
						// That's great, except that they are unresolved idents, so if we learn
						// mappings from a trai defined in a different file, we may mis-resolve or
						// fail to resolve the mapped types.
						let trait_associated_types = learn_associated_types(trait_obj);
						let mut impl_associated_types = HashMap::new();
						for item in i.items.iter() {
							match item {
								syn::ImplItem::Type(t) => {
									if let syn::Type::Path(p) = &t.ty {
										if let Some(id) = single_ident_generic_path_to_ident(&p.path) {
											impl_associated_types.insert(&t.ident, id);
										}
									}
								},
								_ => {},
							}
						}

						let export = export_status(&trait_obj.attrs);
						match export {
							ExportStatus::Export => {},
							ExportStatus::NoExport|ExportStatus::TestOnly => return,
						}
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_as_{}(this_arg: *const {}) -> crate::{} {{\n", ident, trait_obj.ident, ident, full_trait_path).unwrap();
						write!(w, "\tcrate::{} {{\n\t\tthis_arg: unsafe {{ (*this_arg).inner as *mut c_void }},\n", full_trait_path).unwrap();

						macro_rules! print_meth {
							($m: expr, $trait: expr, $indent: expr) => {
								let trait_method = $trait.items.iter().filter_map(|item| {
									if let syn::TraitItem::Method(t_m) = item { Some(t_m) } else { None }
								}).find(|trait_meth| trait_meth.sig.ident == $m.sig.ident).unwrap();
								match export_status(&trait_method.attrs) {
									ExportStatus::Export => {},
									ExportStatus::NoExport => {
										write!(w, "{}\t\t//XXX: Need to export {}\n", $indent, $m.sig.ident).unwrap();
										continue;
									},
									ExportStatus::TestOnly => continue,
								}

								let mut printed = false;
								if let syn::ReturnType::Type(_, rtype) = &$m.sig.output {
									if let syn::Type::Reference(r) = &**rtype {
										write!(w, "\n\t\t{}{}: ", $indent, $m.sig.ident).unwrap();
										types.print_empty_rust_val(w, &*r.elem);
										writeln!(w, ",\n{}\t\tset_{}: Some({}_{}_set_{}),", $indent, $m.sig.ident, ident, trait_obj.ident, $m.sig.ident).unwrap();
										printed = true;
									}
								}
								if !printed {
									write!(w, "{}\t\t{}: {}_{}_{},\n", $indent, $m.sig.ident, ident, trait_obj.ident, $m.sig.ident).unwrap();
								}
							}
						}
						for item in trait_obj.items.iter() {
							match item {
								syn::TraitItem::Method(m) => {
									print_meth!(m, trait_obj, "");
								},
								_ => {},
							}
						}
						walk_supertraits!(trait_obj, types, (
							(s, t) => {
								if s.starts_with("util::") {
									let supertrait_obj = types.crate_types.traits.get(s).unwrap();
									write!(w, "\t\t{}: crate::{} {{\n\t\t\tthis_arg: unsafe {{ (*this_arg).inner as *mut c_void }},\n", t, s).unwrap();
									for item in supertrait_obj.items.iter() {
										match item {
											syn::TraitItem::Method(m) => {
												print_meth!(m, supertrait_obj, "\t");
											},
											_ => {},
										}
									}
									write!(w, "\t\t}},\n").unwrap();
								}
							}
						) );
						write!(w, "\t}}\n}}\nuse {}::{} as {}TraitImport;\n", types.orig_crate, full_trait_path, trait_obj.ident).unwrap();

						macro_rules! impl_meth {
							($m: expr, $trait: expr, $indent: expr) => {
								let trait_method = $trait.items.iter().filter_map(|item| {
									if let syn::TraitItem::Method(t_m) = item { Some(t_m) } else { None }
								}).find(|trait_meth| trait_meth.sig.ident == $m.sig.ident).unwrap();
								match export_status(&trait_method.attrs) {
									ExportStatus::Export => {},
									ExportStatus::NoExport|ExportStatus::TestOnly => continue,
								}

								if let syn::ReturnType::Type(_, _) = &$m.sig.output {
									writeln!(w, "#[must_use]").unwrap();
								}
								write!(w, "extern \"C\" fn {}_{}_{}(", ident, trait_obj.ident, $m.sig.ident).unwrap();
								gen_types.push_ctx();
								assert!(gen_types.learn_generics(&$m.sig.generics, types));
								print_method_params(w, &$m.sig, &trait_associated_types, "c_void", types, Some(&gen_types), true, true);
								write!(w, " {{\n\t").unwrap();
								print_method_var_decl_body(w, &$m.sig, "", types, Some(&gen_types), false);
								let mut takes_self = false;
								for inp in $m.sig.inputs.iter() {
									if let syn::FnArg::Receiver(_) = inp {
										takes_self = true;
									}
								}
								if takes_self {
									write!(w, "unsafe {{ &mut *(this_arg as *mut ln{}) }}.{}(", ident, $m.sig.ident).unwrap();
								} else {
									write!(w, "lightning::{}::{}(", resolved_path, $m.sig.ident).unwrap();
								}

								let mut real_type = "".to_string();
								match &$m.sig.output {
									syn::ReturnType::Type(_, rtype) => {
										if let Some(mut remaining_path) = first_seg_self(&*rtype) {
											if let Some(associated_seg) = get_single_remaining_path_seg(&mut remaining_path) {
												real_type = format!("{}", impl_associated_types.get(associated_seg).unwrap());
											}
										}
									},
									_ => {},
								}
								print_method_call_params(w, &$m.sig, &trait_associated_types, "", types, Some(&gen_types), &real_type, false);
								gen_types.pop_ctx();
								write!(w, "\n}}\n").unwrap();
								if let syn::ReturnType::Type(_, rtype) = &$m.sig.output {
									if let syn::Type::Reference(r) = &**rtype {
										assert_eq!($m.sig.inputs.len(), 1); // Must only take self
										writeln!(w, "extern \"C\" fn {}_{}_set_{}(trait_self_arg: &{}) {{", ident, trait_obj.ident, $m.sig.ident, trait_obj.ident).unwrap();
										writeln!(w, "\t// This is a bit race-y in the general case, but for our specific use-cases today, we're safe").unwrap();
										writeln!(w, "\t// Specifically, we must ensure that the first time we're called it can never be in parallel").unwrap();
										write!(w, "\tif ").unwrap();
										types.print_empty_rust_val_check(w, &*r.elem, &format!("trait_self_arg.{}", $m.sig.ident));
										writeln!(w, " {{").unwrap();
										writeln!(w, "\t\tunsafe {{ &mut *(trait_self_arg as *const {}  as *mut {}) }}.{} = {}_{}_{}(trait_self_arg.this_arg);", trait_obj.ident, trait_obj.ident, $m.sig.ident, ident, trait_obj.ident, $m.sig.ident).unwrap();
										writeln!(w, "\t}}").unwrap();
										writeln!(w, "}}").unwrap();
									}
								}
							}
						}

						for item in i.items.iter() {
							match item {
								syn::ImplItem::Method(m) => {
									impl_meth!(m, trait_obj, "");
								},
								syn::ImplItem::Type(_) => {},
								_ => unimplemented!(),
							}
						}
						walk_supertraits!(trait_obj, types, (
							(s, t) => {
								if s.starts_with("util::") {
									writeln!(w, "use lightning::{} as ln{}Trait;", s, t).unwrap();
									let supertrait_obj = *types.crate_types.traits.get(s).unwrap();
									for item in supertrait_obj.items.iter() {
										match item {
											syn::TraitItem::Method(m) => {
												impl_meth!(m, supertrait_obj, "\t");
											},
											_ => {},
										}
									}
								}
							}
						) );
						write!(w, "\n").unwrap();
					} else if let Some(trait_ident) = trait_path.1.get_ident() {
						//XXX: implement for other things like ToString
						match &format!("{}", trait_ident) as &str {
							"From" => {},
							"Default" => {
								write!(w, "#[must_use]\n#[no_mangle]\npub extern \"C\" fn {}_default() -> {} {{\n", ident, ident).unwrap();
								write!(w, "\t{} {{ inner: Box::into_raw(Box::new(Default::default())), _underlying_ref: false }}\n", ident).unwrap();
								write!(w, "}}\n").unwrap();
							},
							"PartialEq" => {},
							// If we have no generics, try a manual implementation:
							_ if p.path.get_ident().is_some() => maybe_convert_trait_impl(w, &trait_path.1, &ident, types),
							_ => {},
						}
					} else if p.path.get_ident().is_some() {
						// If we have no generics, try a manual implementation:
						maybe_convert_trait_impl(w, &trait_path.1, &ident, types);
					}
				} else {
					let declared_type = (*types.get_declared_type(&ident).unwrap()).clone();
					for item in i.items.iter() {
						match item {
							syn::ImplItem::Method(m) => {
								if let syn::Visibility::Public(_) = m.vis {
									match export_status(&m.attrs) {
										ExportStatus::Export => {},
										ExportStatus::NoExport|ExportStatus::TestOnly => continue,
									}
									if m.defaultness.is_some() { unimplemented!(); }
									println_docs(w, &m.attrs, "");
									if let syn::ReturnType::Type(_, _) = &m.sig.output {
										writeln!(w, "#[must_use]").unwrap();
									}
									write!(w, "#[no_mangle]\npub extern \"C\" fn {}_{}(", ident, m.sig.ident).unwrap();
									let ret_type = match &declared_type {
										DeclType::MirroredEnum => format!("{}", ident),
										DeclType::StructImported => format!("{}", ident),
										_ => unimplemented!(),
									};
									gen_types.push_ctx();
									assert!(gen_types.learn_generics(&m.sig.generics, types));
									print_method_params(w, &m.sig, &HashMap::new(), &ret_type, types, Some(&gen_types), false, true);
									write!(w, " {{\n\t").unwrap();
									print_method_var_decl_body(w, &m.sig, "", types, Some(&gen_types), false);
									let mut takes_self = false;
									let mut takes_mut_self = false;
									for inp in m.sig.inputs.iter() {
										if let syn::FnArg::Receiver(r) = inp {
											takes_self = true;
											if r.mutability.is_some() { takes_mut_self = true; }
										}
									}
									if takes_mut_self {
										write!(w, "unsafe {{ &mut (*(this_arg.inner as *mut ln{})) }}.{}(", ident, m.sig.ident).unwrap();
									} else if takes_self {
										write!(w, "unsafe {{ &*this_arg.inner }}.{}(", m.sig.ident).unwrap();
									} else {
										write!(w, "lightning::{}::{}(", resolved_path, m.sig.ident).unwrap();
									}
									print_method_call_params(w, &m.sig, &HashMap::new(), "", types, Some(&gen_types), &ret_type, false);
									gen_types.pop_ctx();
									writeln!(w, "\n}}\n").unwrap();
								}
							},
							_ => {},
						}
					}
				}
			} else {
				eprintln!("Not implementing anything for {} due to no-resolve (probably the type isn't pub or its marked not exported)", ident);
			}
		}
	}
}

fn is_enum_opaque(e: &syn::ItemEnum) -> bool {
	for var in e.variants.iter() {
		if let syn::Fields::Unit = var.fields {
		} else if let syn::Fields::Named(fields) = &var.fields {
			for field in fields.named.iter() {
				match export_status(&field.attrs) {
					ExportStatus::Export|ExportStatus::TestOnly => {},
					ExportStatus::NoExport => return true,
				}
			}
		} else {
			return true;
		}
	}
	false
}

fn println_enum<'a, 'b, W: std::io::Write>(w: &mut W, e: &'a syn::ItemEnum, types: &mut TypeResolver<'b, 'a>, extra_headers: &mut File, cpp_headers: &mut File) {
	match export_status(&e.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
	}

	if is_enum_opaque(e) {
		eprintln!("Skipping enum {} as it contains non-unit fields", e.ident);
		println_opaque(w, &e.ident, &format!("{}", e.ident), &e.generics, &e.attrs, types, extra_headers, cpp_headers);
		types.enum_ignored(&e.ident);
		return;
	}
	println_docs(w, &e.attrs, "");

	if e.generics.lt_token.is_some() {
		unimplemented!();
	}
	types.mirrored_enum_declared(&e.ident);

	let mut needs_free = false;

	writeln!(w, "#[must_use]\n#[derive(Clone)]\n#[repr(C)]\npub enum {} {{", e.ident).unwrap();
	for var in e.variants.iter() {
		assert_eq!(export_status(&var.attrs), ExportStatus::Export); // We can't partially-export a mirrored enum
		println_docs(w, &var.attrs, "\t");
		write!(w, "\t{}", var.ident).unwrap();
		if let syn::Fields::Named(fields) = &var.fields {
			needs_free = true;
			writeln!(w, " {{").unwrap();
			for field in fields.named.iter() {
				if export_status(&field.attrs) == ExportStatus::TestOnly { continue; }
				write!(w, "\t\t{}: ", field.ident.as_ref().unwrap()).unwrap();
				types.print_c_type(w, &field.ty, None, false);
				writeln!(w, ",").unwrap();
			}
			write!(w, "\t}}").unwrap();
		}
		if var.discriminant.is_some() { unimplemented!(); }
		writeln!(w, ",").unwrap();
	}
	writeln!(w, "}}\nuse {}::{}::{} as ln{};\nimpl {} {{", types.orig_crate, types.module_path, e.ident, e.ident, e.ident).unwrap();

	macro_rules! print_conv {
		($fn_sig: expr, $to_c: expr, $ref: expr) => {
			writeln!(w, "\t#[allow(unused)]\n\tpub(crate) fn {} {{\n\t\tmatch {} {{", $fn_sig, if $to_c { "lnt" } else { "self" }).unwrap();
			for var in e.variants.iter() {
				write!(w, "\t\t\t{}{}::{} ", if $to_c { "ln" } else { "" }, e.ident, var.ident).unwrap();
				if let syn::Fields::Named(fields) = &var.fields {
					write!(w, "{{").unwrap();
					for field in fields.named.iter() {
						if export_status(&field.attrs) == ExportStatus::TestOnly { continue; }
						write!(w, "{}{}, ", if $ref { "ref " } else { "mut " }, field.ident.as_ref().unwrap()).unwrap();
					}
					write!(w, "}} ").unwrap();
				}
				write!(w, "=>").unwrap();
				if let syn::Fields::Named(fields) = &var.fields {
					write!(w, " {{\n\t\t\t\t").unwrap();
					for field in fields.named.iter() {
						if export_status(&field.attrs) == ExportStatus::TestOnly { continue; }
						let mut sink = ::std::io::sink();
						let mut out: &mut dyn std::io::Write = if $ref { &mut sink } else { w };
						let new_var = if $to_c {
							types.print_to_c_conversion_new_var(&mut out, field.ident.as_ref().unwrap(), &field.ty, None)
						} else {
							types.print_from_c_conversion_new_var(&mut out, field.ident.as_ref().unwrap(), &field.ty, None)
						};
						if $ref || new_var {
							if $ref {
								write!(w, "let mut {}_nonref = (*{}).clone();", field.ident.as_ref().unwrap(), field.ident.as_ref().unwrap()).unwrap();
								if new_var {
									let nonref_ident = syn::Ident::new(&format!("{}_nonref", field.ident.as_ref().unwrap()), Span::call_site());
									if $to_c {
										types.print_to_c_conversion_new_var(w, &nonref_ident, &field.ty, None);
									} else {
										types.print_from_c_conversion_new_var(w, &nonref_ident, &field.ty, None);
									}
								}
							}
							write!(w, "\n\t\t\t\t").unwrap();
						}
					}
				} else { write!(w, " ").unwrap(); }
				write!(w, "{}{}::{}", if $to_c { "" } else { "ln" }, e.ident, var.ident).unwrap();
				if let syn::Fields::Named(fields) = &var.fields {
					write!(w, " {{").unwrap();
					for field in fields.named.iter() {
						if export_status(&field.attrs) == ExportStatus::TestOnly { continue; }
						write!(w, "\n\t\t\t\t\t{}: ", field.ident.as_ref().unwrap()).unwrap();
						if $to_c {
							types.print_to_c_conversion_inline_prefix(w, &field.ty, None, false);
						} else {
							types.print_from_c_conversion_prefix(w, &field.ty, None);
						}
						write!(w, "{}{}",
							field.ident.as_ref().unwrap(),
							if $ref { "_nonref" } else { "" }).unwrap();
						if $to_c {
							types.print_to_c_conversion_inline_suffix(w, &field.ty, None, false);
						} else {
							types.print_from_c_conversion_suffix(w, &field.ty, None);
						}
						write!(w, ",").unwrap();
					}
					writeln!(w, "\n\t\t\t\t}}").unwrap();
					write!(w, "\t\t\t}}").unwrap();
				}
				writeln!(w, ",").unwrap();
			}
			writeln!(w, "\t\t}}\n\t}}").unwrap();
		}
	}

	print_conv!(format!("to_ln(&self) -> ln{}", e.ident), false, true);
	print_conv!(format!("into_ln(self) -> ln{}", e.ident), false, false);
	print_conv!(format!("from_ln(lnt: &ln{}) -> Self", e.ident), true, true);
	print_conv!(format!("ln_into(lnt: ln{}) -> Self", e.ident), true, false);
	writeln!(w, "}}").unwrap();

	if needs_free {
		writeln!(w, "#[no_mangle]\npub extern \"C\" fn {}_free(this_ptr: {}) {{ }}", e.ident, e.ident).unwrap();
		writeln!(cpp_headers, "class {} {{\nprivate:", e.ident).unwrap();
		writeln!(cpp_headers, "\tLDK{} self;", e.ident).unwrap();
		writeln!(cpp_headers, "public:").unwrap();
		writeln!(cpp_headers, "\t{}(const {}&) = delete;", e.ident, e.ident).unwrap();
		writeln!(cpp_headers, "\t~{}() {{ if (self.tag != LDK{}_Sentinel) {{ {}_free(self); }} }}", e.ident, e.ident, e.ident).unwrap();
		writeln!(cpp_headers, "\t{}({}&& o) : self(o.self) {{ o.self.tag = LDK{}_Sentinel; }}", e.ident, e.ident, e.ident).unwrap();
		writeln!(cpp_headers, "\t{}(LDK{}&& m_self) : self(m_self) {{ m_self.tag = LDK{}_Sentinel; }}", e.ident, e.ident, e.ident).unwrap();
		writeln!(cpp_headers, "\toperator LDK{}() {{ LDK{} res = self; self.tag = LDK{}_Sentinel; return res; }}", e.ident, e.ident, e.ident).unwrap();
		writeln!(cpp_headers, "\tLDK{}* operator &() {{ return &self; }}", e.ident).unwrap();
		writeln!(cpp_headers, "\tLDK{}* operator ->() {{ return &self; }}", e.ident).unwrap();
		writeln!(cpp_headers, "}};").unwrap();
	} else {
		writeln!(cpp_headers, "typedef LDK{} {};", e.ident, e.ident).unwrap();
	}
}

struct FullLibraryAST {
	files: HashMap<String, syn::File>,
}

fn convert_file<'a, 'b>(libast: &'a FullLibraryAST, crate_types: &mut CrateTypes<'a>, path: &str, out_path: &str, orig_crate: &str, module: &str, header_file: &mut File, cpp_header_file: &mut File) {
	eprintln!("Converting {}...", path);

	let syntax = if let Some(ast) = libast.files.get(module) { ast } else { return };

	assert!(syntax.shebang.is_none()); // Not sure what this is, hope we dont have one

	let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
		.open(&out_path).expect("Unable to open new src file");

	assert_eq!(export_status(&syntax.attrs), ExportStatus::Export);
	println_docs(&mut out, &syntax.attrs, "");

	if path.ends_with("/lib.rs") {
		writeln!(out, "#![allow(unknown_lints)]").unwrap();
		writeln!(out, "#![allow(non_camel_case_types)]").unwrap();
		writeln!(out, "#![allow(non_snake_case)]").unwrap();
		writeln!(out, "#![allow(unused_imports)]").unwrap();
		writeln!(out, "#![allow(unused_variables)]").unwrap();
		writeln!(out, "#![allow(unused_mut)]").unwrap();
		writeln!(out, "#![allow(unused_parens)]").unwrap();
		writeln!(out, "#![allow(unused_unsafe)]").unwrap();
		writeln!(out, "#![allow(unused_braces)]").unwrap();
		writeln!(out, "mod c_types;").unwrap();
		writeln!(out, "mod bitcoin;").unwrap();
	} else {
		writeln!(out, "\nuse std::ffi::c_void;\nuse bitcoin::hashes::Hash;\nuse crate::c_types::*;\n").unwrap();
	}

	for item in syntax.items.iter() {
		match item {
			syn::Item::Mod(m) => {
				if let syn::Visibility::Public(_) = m.vis {
					match export_status(&m.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}

					if m.content.is_some() { unimplemented!(); } // Probably mod tests {}
					let f_path = format!("{}/{}.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
					let new_mod = if module.is_empty() { format!("{}", m.ident) } else { format!("{}::{}", module, m.ident) };
					if let Ok(_) = File::open(&f_path) {
						println_docs(&mut out, &m.attrs, "");
						writeln!(out, "pub mod {};", m.ident).unwrap();
						convert_file(libast, crate_types, &f_path,
							&format!("{}/{}.rs", (out_path.as_ref() as &Path).parent().unwrap().display(), m.ident),
							orig_crate, &new_mod, header_file, cpp_header_file);
					} else {
						let f_path = format!("{}/{}/mod.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
						println_docs(&mut out, &m.attrs, "");
						writeln!(out, "pub mod {};", m.ident).unwrap();
						convert_file(libast, crate_types, &f_path,
							&format!("{}/{}/mod.rs", (out_path.as_ref() as &Path).parent().unwrap().display(), m.ident),
							orig_crate, &new_mod, header_file, cpp_header_file);
					}
				}
			},
			_ => {},
		}
	}

	let mut type_resolver = TypeResolver::new(orig_crate, module, crate_types);

	for item in syntax.items.iter() {
		match item {
			syn::Item::Use(u) => type_resolver.process_use(&mut out, &u),
			syn::Item::Static(_) => {},
			syn::Item::Enum(e) => {
				if let syn::Visibility::Public(_) = e.vis {
					println_enum(&mut out, &e, &mut type_resolver, header_file, cpp_header_file);
				}
			},
			syn::Item::Impl(i) => {
				println_impl(&mut out, &i, &mut type_resolver);
			},
			syn::Item::Struct(s) => {
				if let syn::Visibility::Public(_) = s.vis {
					println_struct(&mut out, &s, &mut type_resolver, header_file, cpp_header_file);
				}
			},
			syn::Item::Trait(t) => {
				if let syn::Visibility::Public(_) = t.vis {
					println_trait(&mut out, &t, &mut type_resolver, header_file, cpp_header_file);
				}
			},
			syn::Item::Mod(_) => {},
			syn::Item::Const(_c) => {
				//XXX
			},
			syn::Item::Type(t) => {
				if let syn::Visibility::Public(_) = t.vis {
					match export_status(&t.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}
					if t.generics.lt_token.is_none() {
						println_opaque(&mut out, &t.ident, &format!("{}", t.ident), &t.generics, &t.attrs, &type_resolver, header_file, cpp_header_file);
					}
				}
			},
			syn::Item::Fn(_c) => {
			},
			syn::Item::Macro(m) => {
				if m.ident.is_none() { // If its not a macro definition
					convert_macro(&mut out, &m.mac.path, &m.mac.tokens, &type_resolver);
				}
			},
			syn::Item::Verbatim(_) => {},
			syn::Item::ExternCrate(_) => {},
			_ => unimplemented!(),
		}
	}

	out.flush().unwrap();
}

fn load_ast(path: &str, module: String, ast_storage: &mut FullLibraryAST) {
	eprintln!("Loading {}...", path);

	let mut file = File::open(path).expect("Unable to open file");
	let mut src = String::new();
	file.read_to_string(&mut src).expect("Unable to read file");
	let syntax = syn::parse_file(&src).expect("Unable to parse file");

	assert_eq!(export_status(&syntax.attrs), ExportStatus::Export);

	for item in syntax.items.iter() {
		match item {
			syn::Item::Mod(m) => {
				if let syn::Visibility::Public(_) = m.vis {
					match export_status(&m.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}

					let f_path = format!("{}/{}.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
					let new_mod = if module.is_empty() { format!("{}", m.ident) } else { format!("{}::{}", module, m.ident) };
					if let Ok(_) = File::open(&f_path) {
						load_ast(&f_path, new_mod, ast_storage);
					} else {
						let f_path = format!("{}/{}/mod.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
						load_ast(&f_path, new_mod, ast_storage);
					}
				}
			},
			_ => {},
		}
	}
	ast_storage.files.insert(module, syntax);
}

fn walk_ast<'a>(path: &str, module: String, ast_storage: &'a FullLibraryAST, crate_types: &mut CrateTypes<'a>) {
	eprintln!("Walking {}...", path);
	let syntax = if let Some(ast) = ast_storage.files.get(&module) { ast } else { return };
	assert_eq!(export_status(&syntax.attrs), ExportStatus::Export);

	for item in syntax.items.iter() {
		match item {
			syn::Item::Mod(m) => {
				if let syn::Visibility::Public(_) = m.vis {
					match export_status(&m.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}

					let f_path = format!("{}/{}.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
					let new_mod = if module.is_empty() { format!("{}", m.ident) } else { format!("{}::{}", module, m.ident) };
					if let Ok(_) = File::open(&f_path) {
						walk_ast(&f_path, new_mod, ast_storage, crate_types);
					} else {
						let f_path = format!("{}/{}/mod.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
						walk_ast(&f_path, new_mod, ast_storage, crate_types);
					}
				}
			},
			syn::Item::Struct(s) => {
				if let syn::Visibility::Public(_) = s.vis {
					match export_status(&s.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}
					let struct_path = format!("{}::{}", module, s.ident);
					crate_types.opaques.insert(struct_path, &s.ident);
				}
			},
			syn::Item::Trait(t) => {
				if let syn::Visibility::Public(_) = t.vis {
					match export_status(&t.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}
					let trait_path = format!("{}::{}", module, t.ident);
					crate_types.traits.insert(trait_path, &t);
				}
			},
			syn::Item::Enum(e) if is_enum_opaque(e) => {
				if let syn::Visibility::Public(_) = e.vis {
					match export_status(&e.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}
					let enum_path = format!("{}::{}", module, e.ident);
					crate_types.opaques.insert(enum_path, &e.ident);
				}
			},
			syn::Item::Enum(e) => {
				if let syn::Visibility::Public(_) = e.vis {
					match export_status(&e.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					}
					let enum_path = format!("{}::{}", module, e.ident);
					crate_types.mirrored_enums.insert(enum_path, &e);
				}
			},
			syn::Item::Impl(i) => {
				match export_status(&i.attrs) {
					ExportStatus::Export => {},
					ExportStatus::NoExport|ExportStatus::TestOnly => continue,
				}
				if i.defaultness.is_none() && i.unsafety.is_none() && i.trait_.is_some() {
					if let syn::Type::Path(path) = &*i.self_ty {
						if let Some(ident) = single_ident_generic_path_to_ident(&path.path) {
							if let Some(trait_ident) = single_ident_generic_path_to_ident(&i.trait_.as_ref().unwrap().1) {
								crate_types.trait_impls.entry(format!("{}::{}", module, ident)).or_insert(Vec::new()).push(trait_ident);
							}
						}
					}
				}
			},
			_ => {},
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 7 {
		eprintln!("Usage: source/dir target/dir source_crate_name module::path derived_templates.rs extra/includes.h extra/cpp/includes.hpp");
		process::exit(1);
	}

	let mut derived_templates = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
		.open(&args[4]).expect("Unable to open new header file");
	let mut header_file = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
		.open(&args[5]).expect("Unable to open new header file");
	let mut cpp_header_file = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
		.open(&args[6]).expect("Unable to open new header file");

	writeln!(header_file, "#if defined(__GNUC__)\n#define MUST_USE_STRUCT __attribute__((warn_unused))").unwrap();
	writeln!(header_file, "#else\n#define MUST_USE_STRUCT\n#endif").unwrap();
	writeln!(header_file, "#if defined(__GNUC__)\n#define MUST_USE_RES __attribute__((warn_unused_result))").unwrap();
	writeln!(header_file, "#else\n#define MUST_USE_RES\n#endif").unwrap();
	writeln!(cpp_header_file, "#include <string.h>\nnamespace LDK {{").unwrap();

	let mut libast = FullLibraryAST { files: HashMap::new() };
	load_ast(&(args[1].clone() + "/lib.rs"), "".to_string(), &mut libast);

	let mut libtypes = CrateTypes { traits: HashMap::new(), trait_impls: HashMap::new(), opaques: HashMap::new(),
		mirrored_enums: HashMap::new(), templates_defined: HashMap::new(), template_file: &mut derived_templates };
	walk_ast(&(args[1].clone() + "/lib.rs"), "".to_string(), &libast, &mut libtypes);

	convert_file(&libast, &mut libtypes, &(args[1].clone() + "/lib.rs"), &(args[2].clone() + "lib.rs"), &args[3], "", &mut header_file, &mut cpp_header_file);

	for (ty, has_destructor) in libtypes.templates_defined.iter() {
		writeln!(cpp_header_file, "struct {} {{", ty).unwrap();
		writeln!(cpp_header_file, "\tLDK{} self;", ty).unwrap();
		writeln!(cpp_header_file, "\t{}(const {}&) = delete;", ty, ty).unwrap();
		if *has_destructor {
			writeln!(cpp_header_file, "\t~{}() {{ {}_free(self); }}", ty, ty).unwrap();
		}
		writeln!(cpp_header_file, "\t{}({}&& o) : self(o.self) {{ memset(&o, 0, sizeof({})); }}", ty, ty, ty).unwrap();
		writeln!(cpp_header_file, "\t{}(LDK{}&& m_self) : self(m_self) {{ memset(&m_self, 0, sizeof(LDK{})); }}", ty, ty, ty).unwrap();
		writeln!(cpp_header_file, "\toperator LDK{}() {{ LDK{} res = self; memset(&self, 0, sizeof(LDK{})); return res; }}", ty, ty, ty).unwrap();
		writeln!(cpp_header_file, "\tLDK{}* operator &() {{ return &self; }}", ty).unwrap();
		writeln!(cpp_header_file, "\tLDK{}* operator ->() {{ return &self; }}", ty).unwrap();
		writeln!(cpp_header_file, "}};").unwrap();
	}
	writeln!(cpp_header_file, "}}").unwrap();

	header_file.flush().unwrap();
	cpp_header_file.flush().unwrap();
	derived_templates.flush().unwrap();
}
