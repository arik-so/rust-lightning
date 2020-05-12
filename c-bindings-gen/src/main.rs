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
						write!(w, "{}//! {}\n", prefix, lit).unwrap();
					},
					_ => unimplemented!(),
				}
			},
			syn::AttrStyle::Outer => {
				match token_iter.next().unwrap() {
					TokenTree::Literal(lit) => {
						// TODO: This adds ""s around lit, which we don't want:
						write!(w, "{}/// {}\n", prefix, lit).unwrap();
					},
					_ => unimplemented!(),
				}
			},
		}
	}
}

fn print_method_params<W: std::io::Write>(w: &mut W, sig: &syn::Signature, associated_types: &HashMap<&syn::Ident, &syn::Ident>, this_param: &str, types: &TypeResolver, generics: Option<&GenericTypes>) {
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
				write!(w, "this_arg: *{} {}", if recv.mutability.is_some() { "mut" } else { "const" }, this_param).unwrap();
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
				types.print_c_type(w, &*arg.ty, generics);
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
					}), generics);
				} else {
					write!(w, "{}", this_param).unwrap();
				}
			} else {
				types.print_c_type(w, &*rtype, generics);
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
							if types.print_to_c_conversion_new_var(w, &ident.ident, &*arg.ty, generics) {
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
			if to_c {
				types.print_from_c_conversion_prefix(w, &*rtype, generics);
			} else {
				if first_seg_self(&*rtype).is_some() {
					write!(w, "let ret = ").unwrap();
				} else {
					types.print_to_c_conversion_inline_prefix(w, &*rtype, generics);
				}
			}
		},
		_ => {},
	}
}

fn print_method_call_params<W: std::io::Write>(w: &mut W, sig: &syn::Signature, types: &TypeResolver, generics: Option<&GenericTypes>, this_type: &str, to_c: bool) {
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
				if types.skip_arg(&*arg.ty, generics) { continue; }
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
							types.print_to_c_conversion_inline_prefix(w, &*arg.ty, generics);
							write!(w, "{}", ident.ident).unwrap();
							types.print_to_c_conversion_inline_suffix(w, &*arg.ty, generics);
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
			if to_c {
				types.print_from_c_conversion_suffix(w, &*rtype, generics);
			} else {
				if first_seg_self(&*rtype).is_some() {
					write!(w, ";\n\t{} {{ inner: Box::into_raw(Box::new(ret)) }}", this_type).unwrap();
				} else {
					types.print_to_c_conversion_inline_suffix(w, &*rtype, generics);
				}
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

fn println_trait<'a, W: std::io::Write>(w: &mut W, t: &'a syn::ItemTrait, module_path: &str, types: &mut TypeResolver<'a>) {
	let trait_name = format!("{}", t.ident);
	match export_status(&t.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
		ExportStatus::Rename(_) => unimplemented!(),
	}
	println_docs(w, &t.attrs, "");

	macro_rules! walk_supertraits { ($( $pat: pat => $e: expr),*) => { {
		if t.colon_token.is_some() {
			for st in t.supertraits.iter() {
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
							let path = types.resolve_path(&supertrait.path);
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

	walk_supertraits!(
		("Clone", _) => write!(w, "#[derive(Clone)]\n").unwrap(),
		("std::cmp::Eq", _) => {},
		("std::hash::Hash", _) => {},
		("Send", _) => {}, ("Sync", _) => {},
		(s, _) => {
			if !s.starts_with("util::") { unimplemented!(); }
		}
	);
	write!(w, "#[repr(C)]\npub struct {} {{\n", trait_name).unwrap();
	write!(w, "\tpub this_arg: *mut c_void,\n").unwrap();
	let mut associated_types: HashMap<&syn::Ident, &syn::Ident> = HashMap::new();
	for item in t.items.iter() {
		match item {
			&syn::TraitItem::Method(ref m) => {
				match export_status(&m.attrs) {
					ExportStatus::NoExport => {
						write!(w, "\t//XXX: Need to export {}\n", m.sig.ident).unwrap();
						continue;
					},
					ExportStatus::Export => {},
					ExportStatus::TestOnly => continue,
					ExportStatus::Rename(_) => unimplemented!(),
				}
				if m.default.is_some() { unimplemented!(); }
				println_docs(w, &m.attrs, "\t");
				write!(w, "\tpub {}: extern \"C\" fn (", m.sig.ident).unwrap();
				print_method_params(w, &m.sig, &associated_types, "c_void", types, None);
				write!(w, ",\n").unwrap();
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
	walk_supertraits!(
		("Clone", _) => {},
		("std::cmp::Eq", _) => write!(w, "\tpub eq: extern \"C\" fn (this_arg: *const c_void, other_arg: *const c_void) -> bool,\n").unwrap(),
		("std::hash::Hash", _) => write!(w, "\tpub hash: extern \"C\" fn (this_arg: *const c_void) -> u64,\n").unwrap(),
		("Send", _) => {}, ("Sync", _) => {},
		(s, i) => {
			if !s.starts_with("util::") { unimplemented!(); }
			write!(w, "\tpub {}: crate::{},\n", i, s).unwrap();
		}
	);
	write!(w, "}}\n").unwrap();
	walk_supertraits!(
		("Send", _) => write!(w, "unsafe impl Send for {} {{}}\n", trait_name).unwrap(),
		("Sync", _) => write!(w, "unsafe impl Sync for {} {{}}\n", trait_name).unwrap(),
		("std::cmp::Eq", _) => {
			write!(w, "impl std::cmp::Eq for {} {{}}\n", trait_name).unwrap();
			write!(w, "impl std::cmp::PartialEq for {} {{\n", trait_name).unwrap();
			write!(w, "\tfn eq(&self, o: &Self) -> bool {{ (self.eq)(self.this_arg, o.this_arg) }}\n}}\n").unwrap();
		},
		("std::hash::Hash", _) => {
			write!(w, "impl std::hash::Hash for {} {{\n", trait_name).unwrap();
			write!(w, "\tfn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {{ hasher.write_u64((self.hash)(self.this_arg)) }}\n}}\n").unwrap();
		},
		("Clone", _) => {},
		(s, _) => {
			if s != "util::events::MessageSendEventsProvider" { unimplemented!(); }
			// We straight-up cheat here. Sadly this really requires knowledg of the fns in a trait
			// in another file, which we don't have any ability to get in the current setup
			write!(w, "impl lightning::{} for {} {{\n", s, trait_name).unwrap();
			write!(w, "\tfn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {{\n").unwrap();
			write!(w, "\t\tunimplemented!()\n\t}}\n}}\n").unwrap();
		}
	);
	write!(w, "\nuse {}::{} as ln{};\n", module_path, t.ident, trait_name).unwrap();
	write!(w, "impl ln{}", t.ident).unwrap();
	maybe_print_generics(w, &t.generics, types);
	write!(w, " for {} {{\n", trait_name).unwrap();
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
						write!(w, "unimplemented!();\n\t}}\n").unwrap();
						continue;
					},
					_ => {},
				}
				print_method_var_decl_body(w, &m.sig, "\t", types, None, true);
				write!(w, "(self.{})(", m.sig.ident).unwrap();
				print_method_call_params(w, &m.sig, types, None, "", true);

				write!(w, "\n\t}}\n").unwrap();
			},
			&syn::TraitItem::Type(ref t) => {
				if t.default.is_some() || t.generics.lt_token.is_some() { unimplemented!(); }
				let mut bounds_iter = t.bounds.iter();
				match bounds_iter.next().unwrap() {
					syn::TypeParamBound::Trait(tr) => {
						write!(w, "\ttype {} = crate::{};\n", t.ident, types.resolve_path(&tr.path)).unwrap();
					},
					_ => unimplemented!(),
				}
				if bounds_iter.next().is_some() { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}
	write!(w, "}}\n\n").unwrap();
	write!(w, "// We're essentially a pointer already, or at least a set of pointers, so allow us to be used\n").unwrap();
	write!(w, "// directly as a Deref trait in higher-level structs:\n").unwrap();
	write!(w, "impl std::ops::Deref for {} {{\n\ttype Target = Self;\n", trait_name).unwrap();
	write!(w, "\tfn deref(&self) -> &Self {{\n\t\tself\n\t}}\n}}\n").unwrap();
	types.trait_declared(&t.ident, t);
}

fn println_opaque<W: std::io::Write>(w: &mut W, ident: &syn::Ident, struct_name: &str, generics: &syn::Generics, attrs: &[syn::Attribute], module_path: &str, types: &TypeResolver, extra_headers: &mut File) {
	// If we directly read the original type by its original name, cbindgen hits
	// https://github.com/eqrion/cbindgen/issues/286 Thus, instead, we import it as a temporary
	// name and then reference it by that name, which works around the issue.
	write!(w, "\nuse {}::{} as ln{}Import;\ntype ln{} = ln{}Import", module_path, ident, ident, ident, ident).unwrap();
	maybe_print_generics(w, &generics, &types);
	write!(w, ";\n\n").unwrap();
	write!(extra_headers, "struct ln{}Opaque;\ntypedef struct ln{}Opaque LDKln{};\n", ident, ident, ident).unwrap();
	println_docs(w, &attrs, "");
	write!(w, "#[repr(C)]\npub struct {} {{\n\tpub(crate) inner: *const ln{},\n}}\n\n", struct_name, ident).unwrap();
	write!(w, "#[no_mangle]\npub extern \"C\" fn {}_free(this_ptr: {}) {{\n", struct_name, struct_name).unwrap();
	write!(w, "\tlet _ = unsafe {{ Box::from_raw(this_ptr.inner as *mut ln{}) }};\n}}\n", struct_name).unwrap();
}

fn println_struct<W: std::io::Write>(w: &mut W, s: &syn::ItemStruct, module_path: &str, types: &mut TypeResolver, extra_headers: &mut File) {
	let mut struct_name = &format!("{}", s.ident);
	let export = export_status(&s.attrs);
	match export {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
		ExportStatus::Rename(ref name) => { struct_name = &name; },
	}

	//XXX: Stupid hack:
	if &struct_name as &str == "Record" {
		types.struct_imported(&s.ident, struct_name.clone());
		return;
	}

	println_opaque(w, &s.ident, struct_name, &s.generics, &s.attrs, module_path, types, extra_headers);

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
					ExportStatus::Rename(_) => { unimplemented!(); },
				}

				if let Some(ident) = &field.ident {
					let ref_type = syn::Type::Reference(syn::TypeReference {
						and_token: syn::Token!(&)(Span::call_site()), lifetime: None, mutability: None,
						elem: Box::new(field.ty.clone()) });
					if types.understood_c_type(&ref_type, Some(&gen_types)) {
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_get_{}(this_ptr: *const {}) -> ", struct_name, ident, struct_name).unwrap();
						types.print_c_type(w, &ref_type, Some(&gen_types));
						write!(w, " {{\n\t").unwrap();
						types.print_to_c_conversion_new_var(w, &ident, &ref_type, Some(&gen_types));
						types.print_to_c_conversion_inline_prefix(w, &ref_type, Some(&gen_types));
						write!(w, "&unsafe {{ &*(*this_ptr).inner }}.{}", ident).unwrap();
						types.print_to_c_conversion_inline_suffix(w, &ref_type, Some(&gen_types));
						write!(w, "\n}}\n").unwrap();
					}

					if types.understood_c_type(&field.ty, Some(&gen_types)) {
						write!(w, "#[no_mangle]\npub extern \"C\" fn {}_set_{}(this_ptr: *mut {}, val: ", struct_name, ident, struct_name).unwrap();
						types.print_c_type(w, &field.ty, Some(&gen_types));
						write!(w, ") {{\n\t").unwrap();
						if types.print_from_c_conversion_new_var(w, &ident, &field.ty, Some(&gen_types)) {
							write!(w, "\n\t").unwrap();
						}
						write!(w, "unsafe {{ &mut *((*this_ptr).inner as *mut ln{}) }}.{} = ", s.ident, ident).unwrap();
						types.print_from_c_conversion_prefix(w, &field.ty, Some(&gen_types));
						write!(w, "val").unwrap();
						types.print_from_c_conversion_suffix(w, &field.ty, Some(&gen_types));
						write!(w, ";\n}}\n").unwrap();
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
				types.print_c_type(w, &field.ty, Some(&gen_types));
			}
			write!(w, ") -> {} {{\n\t", struct_name).unwrap();
			for field in fields.named.iter() {
				if types.print_from_c_conversion_new_var(w, &field.ident.as_ref().unwrap(), &field.ty, Some(&gen_types)) {
					write!(w, "\n\t").unwrap();
				}
			}
			write!(w, "{} {{ inner: Box::into_raw(Box::new(ln{} {{\n", struct_name, s.ident).unwrap();
			for field in fields.named.iter() {
				write!(w, "\t\t{}: ", field.ident.as_ref().unwrap()).unwrap();
				types.print_from_c_conversion_prefix(w, &field.ty, Some(&gen_types));
				write!(w, "{}_arg", field.ident.as_ref().unwrap()).unwrap();
				types.print_from_c_conversion_suffix(w, &field.ty, Some(&gen_types));
				write!(w, ",\n").unwrap();
			}
			write!(w, "\t}}))}}\n}}\n").unwrap();
		}
	}


	types.struct_imported(&s.ident, struct_name.clone());
}

fn println_impl<W: std::io::Write>(w: &mut W, i: &syn::ItemImpl, types: &TypeResolver) {
	if let &syn::Type::Path(ref p) = &*i.self_ty {
		if p.qself.is_some() { unimplemented!(); }
		if let Some(ident) = single_ident_generic_path_to_ident(&p.path) {
			if let Some(resolved_path) = types.maybe_resolve_path(&p.path) {
				let mut gen_types = GenericTypes::new();
				if !gen_types.learn_generics(&i.generics, types) {
					eprintln!("Not implementing anything for impl {} due to not understood generics", ident);
				}

				if i.defaultness.is_some() || i.unsafety.is_some() { unimplemented!(); }
				if let Some(trait_path) = i.trait_.as_ref() {
					if trait_path.0.is_some() { unimplemented!(); }
					if types.understood_c_path(&trait_path.1) {
eprintln!("WIP: IMPL {:?} FOR {}", trait_path.1, ident);
						//XXX: implement for basic things like ToString and implement traits
					}
				} else {
					let declared_type = types.get_declared_type(&ident).unwrap();
					for item in i.items.iter() {
						match item {
							syn::ImplItem::Method(m) => {
								if let syn::Visibility::Public(_) = m.vis {
									match export_status(&m.attrs) {
										ExportStatus::Export => {},
										ExportStatus::NoExport|ExportStatus::TestOnly => continue,
										ExportStatus::Rename(_) => unimplemented!(),
									}
									if m.defaultness.is_some() { unimplemented!(); }
									println_docs(w, &m.attrs, "");
									write!(w, "#[no_mangle]\npub extern \"C\" fn {}_{}(", ident, m.sig.ident).unwrap();
									let ret_type = match declared_type {
										DeclType::MirroredEnum => format!("{}", ident),
										DeclType::StructImported(newname) => format!("{}", newname),
										_ => unimplemented!(),
									};
									print_method_params(w, &m.sig, &HashMap::new(), &ret_type, types, Some(&gen_types));
									write!(w, " {{\n\t").unwrap();
									print_method_var_decl_body(w, &m.sig, "", types, Some(&gen_types), false);
									let mut takes_self = false;
									for inp in m.sig.inputs.iter() {
										if let syn::FnArg::Receiver(_) = inp {
											takes_self = true;
										}
									}
									if takes_self {
										write!(w, "unsafe {{ &*(*this_arg).inner }}.{}(", m.sig.ident).unwrap();
									} else {
										write!(w, "lightning::{}::{}(", resolved_path, m.sig.ident).unwrap();
									}
									print_method_call_params(w, &m.sig, types, Some(&gen_types), &ret_type, false);
									write!(w, "\n}}\n\n").unwrap();
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

fn println_enum<W: std::io::Write>(w: &mut W, e: &syn::ItemEnum, module_path: &str, types: &mut TypeResolver, extra_headers: &mut File) {
	match export_status(&e.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
		ExportStatus::Rename(_) => { unimplemented!(); },
	}

	for var in e.variants.iter() {
		if let syn::Fields::Unit = var.fields {} else {
			eprintln!("Skipping enum {} as it contains non-unit fields", e.ident);
			println_opaque(w, &e.ident, &format!("{}", e.ident), &e.generics, &e.attrs, module_path, types, extra_headers);
			types.enum_ignored(&e.ident);
			return;
		}
	}
	println_docs(w, &e.attrs, "");

	if e.generics.lt_token.is_some() {
		unimplemented!();
	}
	types.mirrored_enum_declared(&e.ident);
	write!(w, "#[repr(C)]\npub enum {} {{\n", e.ident).unwrap();
	for var in e.variants.iter() {
		assert_eq!(export_status(&var.attrs), ExportStatus::Export); // We can't partially-export a mirrored enum
		println_docs(w, &var.attrs, "\t");
		if let syn::Fields::Unit = var.fields {} else { unimplemented!(); }
		if var.discriminant.is_some() { unimplemented!(); }
		write!(w, "\t{},\n", var.ident).unwrap();
	}
	write!(w, "}}\nuse {}::{} as ln{};\nimpl {} {{\n", module_path, e.ident, e.ident, e.ident).unwrap();
	write!(w, "\t#[allow(unused)]\n\tpub(crate) fn to_ln(&self) -> ln{} {{\n\t\tmatch self {{\n", e.ident).unwrap();
	for var in e.variants.iter() {
		write!(w, "\t\t\t{}::{} => ln{}::{},\n", e.ident, var.ident, e.ident, var.ident).unwrap();
	}
	write!(w, "\t\t}}\n\t}}\n").unwrap();
	write!(w, "\t#[allow(unused)]\n\tpub(crate) fn from_ln(lnt: ln{}) -> Self {{\n\t\tmatch lnt {{\n", e.ident).unwrap();
	for var in e.variants.iter() {
		write!(w, "\t\t\tln{}::{} => {}::{},\n", e.ident, var.ident, e.ident, var.ident).unwrap();
	}
	write!(w, "\t\t}}\n\t}}\n}}\n").unwrap();
}

fn should_export(path: &str) -> bool {
	match path {
		f if f.contains("lib.rs") => true,
		f if f.contains("mod.rs") => true,
		f if f.contains("features.rs") => true,
		f if f.contains("config.rs") => true,
		f if f.contains("events.rs") => true,
		f if f.contains("errors.rs") => true,
		f if f.contains("logger.rs") => true,
		f if f.contains("peer_handler.rs") => true,
		f if f.contains("msgs.rs") => true,
		f if f.contains("chaininterface.rs") => true,
		f if f.contains("channelmanager.rs") => true,
		f if f.contains("chan_utils.rs") => true,
		f if f.contains("channelmonitor.rs") => true,
		f if f.contains("keysinterface.rs") => true,
		f if f.contains("transaction.rs") => true,
		_ => false,
	}
}

fn convert_file(path: &str, out_path: &str, orig_crate: &str, module: &str, header_file: &mut File) {
	if !should_export(path) { return; }
	eprintln!("Converting {}...", path);

	let mut file = File::open(path).expect("Unable to open file");
	let mut src = String::new();
	file.read_to_string(&mut src).expect("Unable to read file");
	let syntax = syn::parse_file(&src).expect("Unable to parse file");

	assert!(syntax.shebang.is_none()); // Not sure what this is, hope we dont have one

	let mut out = std::fs::OpenOptions::new().write(true).create(true).truncate(true)
		.open(&out_path).expect("Unable to open new src file");

	assert_eq!(export_status(&syntax.attrs), ExportStatus::Export);
	println_docs(&mut out, &syntax.attrs, "");

	let mut type_resolver = TypeResolver::new(module);
	let orig_module = orig_crate.to_string() + "::" + module;

	if path.ends_with("lib.rs") {
		write!(out, "#![allow(non_camel_case_types)]\n").unwrap();
		write!(out, "#![allow(non_snake_case)]\n").unwrap();
		write!(out, "#![allow(unused_imports)]\n").unwrap();
		write!(out, "#![allow(unused_variables)]\n").unwrap();
		write!(out, "mod c_types;\n").unwrap();
		write!(out, "mod bitcoin;\n").unwrap();
	} else {
		write!(out, "\nuse std::ffi::c_void;\nuse bitcoin::hashes::Hash;\n\n").unwrap();
	}

	for item in syntax.items.iter() {
		match item {
			syn::Item::Use(u) => type_resolver.process_use(&mut out, &u),
			syn::Item::Static(_) => {},
			syn::Item::Enum(e) => {
				if let syn::Visibility::Public(_) = e.vis {
					println_enum(&mut out, &e, &orig_module, &mut type_resolver, header_file);
				}
			},
			syn::Item::Impl(i) => {
				println_impl(&mut out, &i, &type_resolver);
			},
			syn::Item::Struct(s) => {
				if let syn::Visibility::Public(_) = s.vis {
					println_struct(&mut out, &s, &orig_module, &mut type_resolver, header_file);
				}
			},
			syn::Item::Trait(t) => {
				if let syn::Visibility::Public(_) = t.vis {
					println_trait(&mut out, &t, &orig_module, &mut type_resolver);
				}
			},
			syn::Item::Mod(m) => {
				if let syn::Visibility::Public(_) = m.vis {
					match export_status(&m.attrs) {
						ExportStatus::Export => {},
						ExportStatus::NoExport|ExportStatus::TestOnly => continue,
						ExportStatus::Rename(_) => unimplemented!(),
					}

					if m.content.is_some() { unimplemented!(); } // Probably mod tests {}
					let f_path = format!("{}/{}.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
					let new_mod = if module.is_empty() { format!("{}", m.ident) } else { format!("{}::{}", module, m.ident) };
					if let Ok(_) = File::open(&f_path) {

						if should_export(&f_path) {
							println_docs(&mut out, &m.attrs, "");
							write!(out, "pub mod {};\n", m.ident).unwrap();
							convert_file(&f_path, &format!("{}/{}.rs", (out_path.as_ref() as &Path).parent().unwrap().display(), m.ident),
								orig_crate, &new_mod, header_file);
						}
					} else {
						let f_path = format!("{}/{}/mod.rs", (path.as_ref() as &Path).parent().unwrap().display(), m.ident);
						if should_export(&f_path) {
							println_docs(&mut out, &m.attrs, "");
							write!(out, "pub mod {};\n", m.ident).unwrap();
							convert_file(&f_path, &format!("{}/{}/mod.rs", (out_path.as_ref() as &Path).parent().unwrap().display(), m.ident),
								orig_crate, &new_mod, header_file);
						}
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
						ExportStatus::Rename(_) => unimplemented!(),
					}
					if t.generics.lt_token.is_none() {
						println_opaque(&mut out, &t.ident, &format!("{}", t.ident), &t.generics, &t.attrs, &orig_module, &type_resolver, header_file);
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

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 6 {
		eprintln!("Usage: source/dir our/dir orig_crate module::path extra/includes.h");
		process::exit(1);
	}

	let mut header_file = std::fs::OpenOptions::new().write(true).append(true)
		.open(&args[5]).expect("Unable to open new header file");

	convert_file(&(args[1].clone() + "/lib.rs"), &(args[2].clone() + "lib.rs"), &args[3], &args[4], &mut header_file);

	header_file.flush().unwrap();
}
