use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;

use proc_macro2::{TokenTree, Span};

mod types;
use types::*;

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

fn print_method_params<W: std::io::Write>(w: &mut W, sig: &syn::Signature, associated_types: &HashMap<&syn::Ident, &syn::Ident>, this_param: &str, types: &TypeResolver, generics: Option<&GenericTypes>, self_ptr: bool) {
	if sig.constness.is_some() || sig.asyncness.is_some() || sig.unsafety.is_some() ||
			sig.abi.is_some() || sig.variadic.is_some() || sig.generics.where_clause.is_some() {
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
				match &*arg.pat {
					syn::Pat::Ident(ident) => {
						if !ident.attrs.is_empty() || ident.by_ref.is_some() ||
								ident.mutability.is_some() || ident.subpat.is_some() {
							unimplemented!();
						}
						write!(w, "{}{}: ", if first_arg { "" } else { ", " }, ident.ident).unwrap();
						first_arg = false;
					}
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
				types.print_c_type(w, &*rtype, generics, true);
			}
		},
		_ => {},
	}
}

fn print_method_var_decl_body<W: std::io::Write>(w: &mut W, sig: &syn::Signature, extra_indent: &str, types: &TypeResolver, generics: Option<&GenericTypes>, to_c: bool) {
	for inp in sig.inputs.iter() {
		match inp {
			syn::FnArg::Receiver(_) => {},
			syn::FnArg::Typed(arg) => {
				if types.skip_arg(&*arg.ty, generics) { continue; }
				if !arg.attrs.is_empty() { unimplemented!(); }
				match &*arg.pat {
					syn::Pat::Ident(ident) => {
						if !ident.attrs.is_empty() || ident.by_ref.is_some() ||
								ident.mutability.is_some() || ident.subpat.is_some() {
							unimplemented!();
						}
						if to_c {
							if types.print_to_c_conversion_new_var(w, &ident.ident, &*arg.ty, generics, false) {
								write!(w, "\n\t{}", extra_indent).unwrap();
							}
						} else {
							if types.print_from_c_conversion_new_var(w, &ident.ident, &*arg.ty, generics) {
								write!(w, "\n\t{}", extra_indent).unwrap();
							}
						}
					}
					_ => unimplemented!(),
				}
			}
		}
	}
	match &sig.output {
		syn::ReturnType::Type(_, rtype) => {
			write!(w, "let mut ret = ").unwrap();
		},
		_ => {},
	}
}

fn print_method_call_params<W: std::io::Write>(w: &mut W, sig: &syn::Signature, extra_indent: &str, types: &TypeResolver, generics: Option<&GenericTypes>, this_type: &str, to_c: bool) {
	let mut first_arg = true;
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
				match &*arg.pat {
					syn::Pat::Ident(ident) => {
						if !ident.attrs.is_empty() || ident.by_ref.is_some() ||
								ident.mutability.is_some() || ident.subpat.is_some() {
							unimplemented!();
						}
						if !first_arg {
							write!(w, ", ").unwrap();
						}
						first_arg = false;
						if to_c {
							types.print_to_c_conversion_inline_prefix(w, &*arg.ty, generics, false);
							write!(w, "{}", ident.ident).unwrap();
							types.print_to_c_conversion_inline_suffix(w, &*arg.ty, generics, false);
						} else {
							types.print_from_c_conversion_prefix(w, &*arg.ty, generics);
							write!(w, "{}", ident.ident).unwrap();
							types.print_from_c_conversion_suffix(w, &*arg.ty, generics);
						}
					}
					_ => unimplemented!(),
				}
			}
		}
	}
	write!(w, ")").unwrap();
	match &sig.output {
		syn::ReturnType::Type(_, rtype) => {
			write!(w, ";\n\t{}", extra_indent).unwrap();
			if !to_c && first_seg_self(&*rtype).is_some() {
				write!(w, "{} {{ inner: Box::into_raw(Box::new(ret)) }}", this_type).unwrap();
			} else if to_c {
				let new_var = types.print_from_c_conversion_new_var(w, &syn::Ident::new("ret", Span::call_site()), rtype, generics);
				if new_var {
					write!(w, "\n\t{}", extra_indent).unwrap();
				}
				types.print_from_c_conversion_prefix(w, &*rtype, generics);
				write!(w, "ret").unwrap();
				types.print_from_c_conversion_suffix(w, &*rtype, generics);
			} else {
				let new_var = types.print_to_c_conversion_new_var(w, &syn::Ident::new("ret", Span::call_site()), rtype, generics, true);
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

fn maybe_print_generics<W: std::io::Write>(w: &mut W, generics: &syn::Generics, types: &TypeResolver) {
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
					write!(w, "{}'{}", if idx != 0 { ", " } else { "" }, lt.lifetime.ident).unwrap();
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

fn println_trait<'a, W: std::io::Write>(w: &mut W, t: &'a syn::ItemTrait, types: &mut TypeResolver<'a>) {
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
	let mut associated_types: HashMap<&syn::Ident, &syn::Ident> = HashMap::new();
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
				write!(w, "\tpub {}: extern \"C\" fn (", m.sig.ident).unwrap();
				print_method_params(w, &m.sig, &associated_types, "c_void", types, None, true);
				writeln!(w, ",").unwrap();
			},
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
		(s, _) => {
			if s != "util::events::MessageSendEventsProvider" { unimplemented!(); }
			// We straight-up cheat here. Sadly this really requires knowledg of the fns in a trait
			// in another file, which we don't have any ability to get in the current setup
			writeln!(w, "impl lightning::{} for {} {{", s, trait_name).unwrap();
			writeln!(w, "\tfn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {{").unwrap();
			writeln!(w, "\t\tunimplemented!()\n\t}}\n}}").unwrap();
		}
	) );
	writeln!(w, "\nuse {}::{}::{} as ln{};", types.orig_crate, types.module_path, t.ident, trait_name).unwrap();
	write!(w, "impl ln{}", t.ident).unwrap();
	maybe_print_generics(w, &t.generics, types);
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
				print_method_var_decl_body(w, &m.sig, "\t", types, None, true);
				write!(w, "(self.{})(", m.sig.ident).unwrap();
				print_method_call_params(w, &m.sig, "\t", types, None, "", true);

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
	types.trait_declared(&t.ident, t);
}

fn println_opaque<W: std::io::Write>(w: &mut W, ident: &syn::Ident, struct_name: &str, generics: &syn::Generics, attrs: &[syn::Attribute], types: &TypeResolver, extra_headers: &mut File) {
	// If we directly read the original type by its original name, cbindgen hits
	// https://github.com/eqrion/cbindgen/issues/286 Thus, instead, we import it as a temporary
	// name and then reference it by that name, which works around the issue.
	write!(w, "\nuse {}::{}::{} as ln{}Import;\ntype ln{} = ln{}Import", types.orig_crate, types.module_path, ident, ident, ident, ident).unwrap();
	maybe_print_generics(w, &generics, &types);
	writeln!(w, ";\n").unwrap();
	writeln!(extra_headers, "struct ln{}Opaque;\ntypedef struct ln{}Opaque LDKln{};", ident, ident, ident).unwrap();
	println_docs(w, &attrs, "");
	writeln!(w, "#[repr(C)]\npub struct {} {{\n\t/// Nearly everyhwere, inner must be non-null, however in places where", struct_name).unwrap();
	writeln!(w, "\t///the Rust equivalent takes an Option, it may be set to null to indicate None.").unwrap();
	writeln!(w, "\tpub inner: *const ln{},\n}}\n", ident).unwrap();
	writeln!(w, "#[no_mangle]\npub extern \"C\" fn {}_free(this_ptr: {}) {{", struct_name, struct_name).unwrap();
	writeln!(w, "\tlet _ = unsafe {{ Box::from_raw(this_ptr.inner as *mut ln{}) }};\n}}", struct_name).unwrap();
}

fn println_struct<W: std::io::Write>(w: &mut W, s: &syn::ItemStruct, types: &mut TypeResolver, extra_headers: &mut File) {
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

	//XXX: Stupid hack:
	if &struct_name as &str == "Record" {
		types.struct_imported(&s.ident, struct_name.clone());
		return;
	}

	println_opaque(w, &s.ident, struct_name, &s.generics, &s.attrs, types, extra_headers);

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
						let local_var = types.print_to_c_conversion_new_var(w, &syn::Ident::new("inner_val", Span::call_site()), &ref_type, Some(&gen_types), true);
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
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_set_{}(this_ptr: &mut {}, val: ", struct_name, ident, struct_name).unwrap();
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
			write!(w, "#[no_mangle]\npub extern \"C\" fn {}_new(", struct_name).unwrap();
			for (idx, field) in fields.named.iter().enumerate() {
				if idx != 0 { write!(w, ", ").unwrap(); }
				write!(w, "{}_arg: ", field.ident.as_ref().unwrap()).unwrap();
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
			writeln!(w, "\t}}))}}\n}}").unwrap();
		}
	}

	types.struct_imported(&s.ident, struct_name.clone());
}

fn println_impl<W: std::io::Write>(w: &mut W, i: &syn::ItemImpl, types: &TypeResolver) {
	if let &syn::Type::Path(ref p) = &*i.self_ty {
		if p.qself.is_some() { unimplemented!(); }
		if let Some(ident) = single_ident_generic_path_to_ident(&p.path) {
			if let Some(resolved_path) = types.maybe_resolve_non_ignored_ident(&ident) {
				let mut gen_types = GenericTypes::new();
				if !gen_types.learn_generics(&i.generics, types) {
					eprintln!("Not implementing anything for impl {} due to not understood generics", ident);
				}

				if i.defaultness.is_some() || i.unsafety.is_some() { unimplemented!(); }
				if let Some(trait_path) = i.trait_.as_ref() {
					if trait_path.0.is_some() { unimplemented!(); }
					if types.understood_c_path(&trait_path.1) {
eprintln!("WIP: IMPL {:?} FOR {}", trait_path.1, ident);
						let full_trait_path = types.resolve_path(&trait_path.1);
						let trait_obj = types.crate_types.traits.get(&full_trait_path).unwrap();
						let export = export_status(&trait_obj.attrs);
						match export {
							ExportStatus::Export => {},
							ExportStatus::NoExport|ExportStatus::TestOnly => return,
						}
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_as_{}(this_arg: *const {}) -> crate::{} {{\n", ident, trait_obj.ident, ident, full_trait_path).unwrap();
						write!(w, "\tcrate::{} {{\n\t\tthis_arg: unsafe {{ (*this_arg).inner as *mut c_void }},\n", full_trait_path).unwrap();

						for item in i.items.iter() {
							match item {
								syn::ImplItem::Method(m) => {
									let trait_method = trait_obj.items.iter().filter_map(|item| {
										if let syn::TraitItem::Method(t_m) = item { Some(t_m) } else { None }
									}).find(|trait_meth| trait_meth.sig.ident == m.sig.ident).unwrap();
									match export_status(&trait_method.attrs) {
										ExportStatus::Export => {},
										ExportStatus::NoExport => {
											write!(w, "\t\t//XXX: Need to export {}\n", m.sig.ident).unwrap();
											continue;
										},
										ExportStatus::TestOnly => continue,
									}
									write!(w, "\t\t{}: {}_{}_{},\n", m.sig.ident, ident, trait_obj.ident, m.sig.ident).unwrap();
								},
								_ => {},
							}
						}
						walk_supertraits!(trait_obj, types, (
							(s, t) => {
								if s.starts_with("util::") {
									let supertrait_obj = types.crate_types.traits.get(s).unwrap();
									write!(w, "\t\t{}: crate::{} {{\t\t\tthis_arg: unsafe {{ (*this_arg).inner as *mut c_void }},\n", t, s).unwrap();
									// TODO: Expose supertrait methods
									write!(w, "\t\t}},\n").unwrap();
								}
							}
						) );
						write!(w, "\t}}\n}}\nuse {}::{} as {}TraitImport;\n", types.orig_crate, full_trait_path, trait_obj.ident).unwrap();

						for item in i.items.iter() {
							match item {
								syn::ImplItem::Method(m) => {
									let trait_method = trait_obj.items.iter().filter_map(|item| {
										if let syn::TraitItem::Method(t_m) = item { Some(t_m) } else { None }
									}).find(|trait_meth| trait_meth.sig.ident == m.sig.ident).unwrap();
									match export_status(&trait_method.attrs) {
										ExportStatus::Export => {},
										ExportStatus::NoExport|ExportStatus::TestOnly => continue,
									}
									write!(w, "extern \"C\" fn {}_{}_{}(", ident, trait_obj.ident, m.sig.ident).unwrap();
									print_method_params(w, &m.sig, &HashMap::new(), "c_void", types, Some(&gen_types), true);
									write!(w, " {{\n\t").unwrap();
									print_method_var_decl_body(w, &m.sig, "", types, Some(&gen_types), false);
									let mut takes_self = false;
									for inp in m.sig.inputs.iter() {
										if let syn::FnArg::Receiver(_) = inp {
											takes_self = true;
										}
									}
									if takes_self {
										write!(w, "unsafe {{ &*(*(this_arg as *const {})).inner }}.{}(", ident, m.sig.ident).unwrap();
									} else {
										write!(w, "lightning::{}::{}(", resolved_path, m.sig.ident).unwrap();
									}
									print_method_call_params(w, &m.sig, "", types, Some(&gen_types), "", false);
									write!(w, "\n}}\n").unwrap();
								},
								syn::ImplItem::Type(_) => {},
								_ => unimplemented!(),
							}
						}
						write!(w, "\n").unwrap();
					} else if let Some(trait_ident) = trait_path.1.get_ident() {
						//XXX: implement for other things like ToString
						match &format!("{}", trait_ident) as &str {
							"From" => {},
							"Default" => {
								write!(w, "#[no_mangle]\npub extern \"C\" fn {}_default() -> {} {{\n", ident, ident).unwrap();
								write!(w, "\t{} {{ inner: Box::into_raw(Box::new(Default::default())) }}\n", ident).unwrap();
								write!(w, "}}\n").unwrap();
							},
							"PartialEq" => {},
							_ => {},
						}
					}
				} else {
					let declared_type = types.get_declared_type(&ident).unwrap();
eprintln!("GOOOOOO {}", ident);
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
									write!(w, "#[no_mangle]\npub extern \"C\" fn {}_{}(", ident, m.sig.ident).unwrap();
									let ret_type = match declared_type {
										DeclType::MirroredEnum => format!("{}", ident),
										DeclType::StructImported(newname) => format!("{}", newname),
										_ => unimplemented!(),
									};
									print_method_params(w, &m.sig, &HashMap::new(), &ret_type, types, Some(&gen_types), false);
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
									print_method_call_params(w, &m.sig, "", types, Some(&gen_types), &ret_type, false);
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

fn println_enum<W: std::io::Write>(w: &mut W, e: &syn::ItemEnum, types: &mut TypeResolver, extra_headers: &mut File) {
	match export_status(&e.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
	}

	for var in e.variants.iter() {
		if let syn::Fields::Unit = var.fields {} else {
			eprintln!("Skipping enum {} as it contains non-unit fields", e.ident);
			println_opaque(w, &e.ident, &format!("{}", e.ident), &e.generics, &e.attrs, types, extra_headers);
			types.enum_ignored(&e.ident);
			return;
		}
	}
	println_docs(w, &e.attrs, "");

	if e.generics.lt_token.is_some() {
		unimplemented!();
	}
	types.mirrored_enum_declared(&e.ident);
	writeln!(w, "#[repr(C)]\npub enum {} {{", e.ident).unwrap();
	for var in e.variants.iter() {
		assert_eq!(export_status(&var.attrs), ExportStatus::Export); // We can't partially-export a mirrored enum
		println_docs(w, &var.attrs, "\t");
		if let syn::Fields::Unit = var.fields {} else { unimplemented!(); }
		if var.discriminant.is_some() { unimplemented!(); }
		writeln!(w, "\t{},", var.ident).unwrap();
	}
	writeln!(w, "}}\nuse {}::{}::{} as ln{};\nimpl {} {{", types.orig_crate, types.module_path, e.ident, e.ident, e.ident).unwrap();
	writeln!(w, "\t#[allow(unused)]\n\tpub(crate) fn to_ln(&self) -> ln{} {{\n\t\tmatch self {{", e.ident).unwrap();
	for var in e.variants.iter() {
		writeln!(w, "\t\t\t{}::{} => ln{}::{},", e.ident, var.ident, e.ident, var.ident).unwrap();
	}
	writeln!(w, "\t\t}}\n\t}}").unwrap();
	writeln!(w, "\t#[allow(unused)]\n\tpub(crate) fn from_ln(lnt: ln{}) -> Self {{\n\t\tmatch lnt {{", e.ident).unwrap();
	for var in e.variants.iter() {
		writeln!(w, "\t\t\tln{}::{} => {}::{},", e.ident, var.ident, e.ident, var.ident).unwrap();
	}
	writeln!(w, "\t\t}}\n\t}}\n}}").unwrap();
}

struct FullLibraryAST {
	files: HashMap<String, syn::File>,
}

fn convert_file(libast: &FullLibraryAST, crate_types: &CrateTypes, path: &str, out_path: &str, orig_crate: &str, module: &str, header_file: &mut File) {
	eprintln!("Converting {}...", path);

	let syntax = if let Some(ast) = libast.files.get(module) { ast } else { return };

	assert!(syntax.shebang.is_none()); // Not sure what this is, hope we dont have one

	let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
		.open(&out_path).expect("Unable to open new src file");

	assert_eq!(export_status(&syntax.attrs), ExportStatus::Export);
	println_docs(&mut out, &syntax.attrs, "");

	let mut type_resolver = TypeResolver::new(orig_crate, module, crate_types);

	if path.ends_with("/lib.rs") {
		writeln!(out, "#![allow(non_camel_case_types)]").unwrap();
		writeln!(out, "#![allow(non_snake_case)]").unwrap();
		writeln!(out, "#![allow(unused_imports)]").unwrap();
		writeln!(out, "#![allow(unused_variables)]").unwrap();
		writeln!(out, "#![allow(unused_mut)]").unwrap();
		writeln!(out, "#![allow(unused_parens)]").unwrap();
		writeln!(out, "#![allow(unused_unsafe)]").unwrap();
		writeln!(out, "mod c_types;").unwrap();
		writeln!(out, "mod bitcoin;").unwrap();
	} else {
		writeln!(out, "\nuse std::ffi::c_void;\nuse bitcoin::hashes::Hash;\n").unwrap();
	}

	for item in syntax.items.iter() {
		match item {
			syn::Item::Use(u) => type_resolver.process_use(&mut out, &u),
			syn::Item::Static(_) => {},
			syn::Item::Enum(e) => {
				if let syn::Visibility::Public(_) = e.vis {
					println_enum(&mut out, &e, &mut type_resolver, header_file);
				}
			},
			syn::Item::Impl(i) => {
				println_impl(&mut out, &i, &type_resolver);
			},
			syn::Item::Struct(s) => {
				if let syn::Visibility::Public(_) = s.vis {
					println_struct(&mut out, &s, &mut type_resolver, header_file);
				}
			},
			syn::Item::Trait(t) => {
				if let syn::Visibility::Public(_) = t.vis {
					println_trait(&mut out, &t, &mut type_resolver);
				}
			},
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
							orig_crate, &new_mod, header_file);
					} else {
						let f_path = format!("{}/{}/mod.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
						println_docs(&mut out, &m.attrs, "");
						writeln!(out, "pub mod {};", m.ident).unwrap();
						convert_file(libast, crate_types, &f_path,
							&format!("{}/{}/mod.rs", (out_path.as_ref() as &Path).parent().unwrap().display(), m.ident),
							orig_crate, &new_mod, header_file);
					}
				}
			},
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
						println_opaque(&mut out, &t.ident, &format!("{}", t.ident), &t.generics, &t.attrs, &type_resolver, header_file);
					}
				}
			},
			syn::Item::Fn(_c) => {
			},
			syn::Item::Macro(_) => {},
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
					crate_types.structs.insert(struct_path, &s);
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
	if args.len() != 5 {
		eprintln!("Usage: source/dir target/dir source_crate_name module::path extra/includes.h");
		process::exit(1);
	}

	let mut header_file = std::fs::OpenOptions::new().write(true).truncate(true)
		.open(&args[4]).expect("Unable to open new header file");

	let mut libast = FullLibraryAST { files: HashMap::new() };
	load_ast(&(args[1].clone() + "/lib.rs"), "".to_string(), &mut libast);

	let mut libtypes = CrateTypes { traits: HashMap::new(), trait_impls: HashMap::new(), structs: HashMap::new() };
	walk_ast(&(args[1].clone() + "/lib.rs"), "".to_string(), &libast, &mut libtypes);

	convert_file(&libast, &libtypes, &(args[1].clone() + "/lib.rs"), &(args[2].clone() + "lib.rs"), &args[3], "", &mut header_file);

	header_file.flush().unwrap();
}
