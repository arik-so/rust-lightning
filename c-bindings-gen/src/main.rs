use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

use proc_macro2::{TokenTree, Span};

mod types;
use types::*;

/// Prints the docs from a given attribute list unless its tagged no export
fn println_docs(attrs: &[syn::Attribute], prefix: &str) {
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
						println!("{}//! {}", prefix, lit);
					},
					_ => unimplemented!(),
				}
			},
			syn::AttrStyle::Outer => {
				match token_iter.next().unwrap() {
					TokenTree::Literal(lit) => {
						// TODO: This adds ""s around lit, which we don't want:
						println!("{}/// {}", prefix, lit);
					},
					_ => unimplemented!(),
				}
			},
		}
	}
}

fn print_method_params(sig: &syn::Signature, associated_types: &HashMap<&syn::Ident, &syn::Ident>, this_param: &str, types: &TypeResolver, generics: Option<&GenericTypes>) {
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
				print!("this_arg: *{} {}", if recv.mutability.is_some() { "mut" } else { "const" }, this_param);
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
						print!("{}{}: ", if first_arg { "" } else { ", " }, ident.ident);
						first_arg = false;
					}
					_ => unimplemented!(),
				}
				types.print_c_type(&*arg.ty, generics);
			}
		}
	}
	print!(")");
	match &sig.output {
		syn::ReturnType::Type(_, rtype) => {
			print!(" -> ");
			if let Some(mut remaining_path) = first_seg_self(&*rtype) {
				if let Some(associated_seg) = get_single_remaining_path_seg(&mut remaining_path) {
					let real_type = associated_types.get(associated_seg).unwrap();
					types.print_c_type(&syn::Type::Path(syn::TypePath { qself: None,
						path: syn::PathSegment {
							ident: (*real_type).clone(),
							arguments: syn::PathArguments::None
						}.into()
					}), generics);
				} else {
					print!("{}", this_param);
				}
			} else {
				types.print_c_type(&*rtype, generics);
			}
		},
		_ => {},
	}
}

fn print_method_var_decl_body(sig: &syn::Signature, extra_indent: &str, types: &TypeResolver, generics: Option<&GenericTypes>, to_c: bool) {
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
							if types.print_to_c_conversion_new_var(&ident.ident, &*arg.ty, generics) {
								print!("\n\t{}", extra_indent);
							}
						} else {
							if types.print_from_c_conversion_new_var(&ident.ident, &*arg.ty, generics) {
								print!("\n\t{}", extra_indent);
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
				types.print_from_c_conversion_prefix(&*rtype, generics);
			} else {
				if first_seg_self(&*rtype).is_some() {
					print!("let ret = ");
				} else {
					types.print_to_c_conversion_inline_prefix(&*rtype, generics);
				}
			}
		},
		_ => {},
	}
}

fn print_method_call_params(sig: &syn::Signature, types: &TypeResolver, generics: Option<&GenericTypes>, this_type: &str, to_c: bool) {
	let mut first_arg = true;
	for inp in sig.inputs.iter() {
		match inp {
			syn::FnArg::Receiver(recv) => {
				if !recv.attrs.is_empty() || recv.reference.is_none() { unimplemented!(); }
				if recv.reference.as_ref().unwrap().1.is_some() { unimplemented!(); }
				if to_c {
					print!("self.this_arg");
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
							print!(", ");
						}
						first_arg = false;
						if to_c {
							types.print_to_c_conversion_inline_prefix(&*arg.ty, generics);
							print!("{}", ident.ident);
							types.print_to_c_conversion_inline_suffix(&*arg.ty, generics);
						} else {
							types.print_from_c_conversion_prefix(&*arg.ty, generics);
							print!("{}", ident.ident);
							types.print_from_c_conversion_suffix(&*arg.ty, generics);
						}
					}
					_ => unimplemented!(),
				}
			}
		}
	}
	print!(")");
	match &sig.output {
		syn::ReturnType::Type(_, rtype) => {
			if to_c {
				types.print_from_c_conversion_suffix(&*rtype, generics);
			} else {
				if first_seg_self(&*rtype).is_some() {
					print!(";\n\t{} {{ inner: Box::into_raw(Box::new(ret)) }}", this_type);
				} else {
					types.print_to_c_conversion_inline_suffix(&*rtype, generics);
				}
			}
		}
		_ => {},
	}
}

fn maybe_print_generics(generics: &syn::Generics, types: &TypeResolver) {
	let mut gen_types = GenericTypes::new();
	assert!(gen_types.learn_generics(generics, types));
	if !generics.params.is_empty() {
		print!("<");
		for (idx, generic) in generics.params.iter().enumerate() {
			match generic {
				syn::GenericParam::Type(type_param) => {
					let bound = type_param.bounds.iter().next().unwrap();
					if let syn::TypeParamBound::Trait(trait_bound) = bound {
						assert_simple_bound(&trait_bound);
						print!("{}{}", if idx != 0 { ", " } else { "" }, gen_types.maybe_resolve_ident(&type_param.ident).unwrap());
					}
				},
				syn::GenericParam::Lifetime(lt) => {
					print!("{}'{}", if idx != 0 { ", " } else { "" }, lt.lifetime.ident);
				},
				_ => unimplemented!(),
			}
		}
		print!(">");
	}
}

fn println_trait<'a>(t: &'a syn::ItemTrait, module_path: &str, types: &mut TypeResolver<'a>) {
	let trait_name = format!("{}", t.ident);
	match export_status(&t.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
		ExportStatus::Rename(_) => unimplemented!(),
	}
	println_docs(&t.attrs, "");

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
		("Clone", _) => println!("#[derive(Clone)]"),
		("std::cmp::Eq", _) => {},
		("std::hash::Hash", _) => {},
		("Send", _) => {}, ("Sync", _) => {},
		(s, _) => {
			if !s.starts_with("util::") { unimplemented!(); }
		}
	);
	println!("#[repr(C)]\npub struct {} {{", trait_name);
	println!("\tpub this_arg: *mut c_void,");
	let mut associated_types: HashMap<&syn::Ident, &syn::Ident> = HashMap::new();
	for item in t.items.iter() {
		match item {
			&syn::TraitItem::Method(ref m) => {
				match export_status(&m.attrs) {
					ExportStatus::NoExport => {
						println!("\t//XXX: Need to export {}", m.sig.ident);
						continue;
					},
					ExportStatus::Export => {},
					ExportStatus::TestOnly => continue,
					ExportStatus::Rename(_) => unimplemented!(),
				}
				if m.default.is_some() { unimplemented!(); }
				println_docs(&m.attrs, "\t");
				print!("\tpub {}: extern \"C\" fn (", m.sig.ident);
				print_method_params(&m.sig, &associated_types, "c_void", types, None);
				println!(",");
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
		("std::cmp::Eq", _) => println!("\tpub eq: extern \"C\" fn (this_arg: *const c_void, other_arg: *const c_void) -> bool,"),
		("std::hash::Hash", _) => println!("\tpub hash: extern \"C\" fn (this_arg: *const c_void) -> u64,"),
		("Send", _) => {}, ("Sync", _) => {},
		(s, i) => {
			if !s.starts_with("util::") { unimplemented!(); }
			println!("\tpub {}: crate::{},", i, s);
		}
	);
	println!("}}");
	walk_supertraits!(
		("Send", _) => println!("unsafe impl Send for {} {{}}", trait_name),
		("Sync", _) => println!("unsafe impl Sync for {} {{}}", trait_name),
		("std::cmp::Eq", _) => {
			println!("impl std::cmp::Eq for {} {{}}", trait_name);
			println!("impl std::cmp::PartialEq for {} {{", trait_name);
			println!("\tfn eq(&self, o: &Self) -> bool {{ (self.eq)(self.this_arg, o.this_arg) }}\n}}");
		},
		("std::hash::Hash", _) => {
			println!("impl std::hash::Hash for {} {{", trait_name);
			println!("\tfn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {{ hasher.write_u64((self.hash)(self.this_arg)) }}\n}}");
		},
		("Clone", _) => {},
		(s, _) => {
			if s != "util::events::MessageSendEventsProvider" { unimplemented!(); }
			// We straight-up cheat here. Sadly this really requires knowledg of the fns in a trait
			// in another file, which we don't have any ability to get in the current setup
			println!("impl lightning::{} for {} {{", s, trait_name);
			println!("\tfn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {{");
			println!("\t\tunimplemented!()\n\t}}\n}}");
		}
	);
	println!("\nuse {}::{} as ln{};", module_path, t.ident, trait_name);
	print!("impl ln{}", t.ident);
	maybe_print_generics(&t.generics, types);
	println!(" for {} {{", trait_name);
	for item in t.items.iter() {
		match item {
			syn::TraitItem::Method(m) => {
				if let ExportStatus::TestOnly = export_status(&m.attrs) { continue; }
				if m.default.is_some() { unimplemented!(); }
				if m.sig.constness.is_some() || m.sig.asyncness.is_some() || m.sig.unsafety.is_some() ||
						m.sig.abi.is_some() || m.sig.variadic.is_some() {
					unimplemented!();
				}
				print!("\tfn {}", m.sig.ident);
				types.print_rust_generic_param(m.sig.generics.params.iter());
				print!("(");
				for inp in m.sig.inputs.iter() {
					match inp {
						syn::FnArg::Receiver(recv) => {
							if !recv.attrs.is_empty() || recv.reference.is_none() { unimplemented!(); }
							print!("&");
							if let Some(lft) = &recv.reference.as_ref().unwrap().1 {
								print!("'{} ", lft.ident);
							}
							if recv.mutability.is_some() {
								print!("mut self");
							} else {
								print!("self");
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
									print!(", {}{}: ", if types.skip_arg(&*arg.ty, None) { "_" } else { "" }, ident.ident);
								}
								_ => unimplemented!(),
							}
							types.print_rust_type(&*arg.ty);
						}
					}
				}
				print!(")");
				match &m.sig.output {
					syn::ReturnType::Type(_, rtype) => {
						print!(" -> ");
						types.print_rust_type(&*rtype)
					},
					_ => {},
				}
				print!(" {{\n\t\t");
				match export_status(&m.attrs) {
					ExportStatus::NoExport => {
						println!("unimplemented!();\n\t}}");
						continue;
					},
					_ => {},
				}
				print_method_var_decl_body(&m.sig, "\t", types, None, true);
				print!("(self.{})(", m.sig.ident);
				print_method_call_params(&m.sig, types, None, "", true);

				println!("\n\t}}");
			},
			&syn::TraitItem::Type(ref t) => {
				if t.default.is_some() || t.generics.lt_token.is_some() { unimplemented!(); }
				let mut bounds_iter = t.bounds.iter();
				match bounds_iter.next().unwrap() {
					syn::TypeParamBound::Trait(tr) => {
						println!("\ttype {} = crate::{};", t.ident, types.resolve_path(&tr.path));
					},
					_ => unimplemented!(),
				}
				if bounds_iter.next().is_some() { unimplemented!(); }
			},
			_ => unimplemented!(),
		}
	}
	println!("}}\n");
	println!("// We're essentially a pointer already, or at least a set of pointers, so allow us to be used");
	println!("// directly as a Deref trait in higher-level structs:");
	println!("impl std::ops::Deref for {} {{\n\ttype Target = Self;", trait_name);
	println!("\tfn deref(&self) -> &Self {{\n\t\tself\n\t}}\n}}");
	types.trait_declared(&t.ident, t);
}

fn println_opaque(ident: &syn::Ident, struct_name: &str, generics: &syn::Generics, attrs: &[syn::Attribute], module_path: &str, types: &TypeResolver, extra_headers: &mut File) {
	// If we directly read the original type by its original name, cbindgen hits
	// https://github.com/eqrion/cbindgen/issues/286 Thus, instead, we import it as a temporary
	// name and then reference it by that name, which works around the issue.
	print!("\nuse {}::{} as ln{}Import;\ntype ln{} = ln{}Import", module_path, ident, ident, ident, ident);
	maybe_print_generics(&generics, &types);
	println!(";\n");
	write!(extra_headers, "struct ln{}Opaque;\ntypedef struct ln{}Opaque LDKln{};\n", ident, ident, ident).unwrap();
	println_docs(&attrs, "");
	println!("#[repr(C)]\npub struct {} {{\n\tpub(crate) inner: *const ln{},\n}}\n", struct_name, ident);
}

fn println_struct(s: &syn::ItemStruct, module_path: &str, types: &mut TypeResolver, extra_headers: &mut File) {
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

	println_opaque(&s.ident, struct_name, &s.generics, &s.attrs, module_path, types, extra_headers);

	eprintln!("exporting fields for {}", struct_name);
	if let syn::Fields::Named(fields) = &s.fields {
		let mut gen_types = GenericTypes::new();
		if !gen_types.learn_generics(&s.generics, types) {
			eprintln!("Not implementing anything for struct {} due to not understood generics", struct_name);
		}

		for field in fields.named.iter() {
			if let syn::Visibility::Public(_) = field.vis {
				let export = export_status(&field.attrs);
				match export {
					ExportStatus::Export => {},
					ExportStatus::NoExport|ExportStatus::TestOnly => continue,
					ExportStatus::Rename(_) => { unimplemented!(); },
				}

				if let Some(ident) = &field.ident {
					let ref_type = syn::Type::Reference(syn::TypeReference {
						and_token: syn::Token!(&)(Span::call_site()), lifetime: None, mutability: None,
						elem: Box::new(field.ty.clone()) });
					if types.understood_c_type(&ref_type, Some(&gen_types)) {
						print!("#[no_mangle]\npub extern \"C\" fn {}_get_{}(this_ptr: *const {}) -> ", struct_name, ident, struct_name);
						types.print_c_type(&ref_type, Some(&gen_types));
						print!(" {{\n\t");
						types.print_to_c_conversion_new_var(&ident, &ref_type, Some(&gen_types));
						types.print_to_c_conversion_inline_prefix(&ref_type, Some(&gen_types));
						print!("&unsafe {{ &*(*this_ptr).inner }}.{}", ident);
						types.print_to_c_conversion_inline_suffix(&ref_type, Some(&gen_types));
						println!("\n}}");
					}

					if types.understood_c_type(&field.ty, Some(&gen_types)) {
						print!("#[no_mangle]\npub extern \"C\" fn {}_set_{}(this_ptr: *mut {}, val: ", struct_name, ident, struct_name);
						types.print_c_type(&field.ty, Some(&gen_types));
						print!(") {{\n\t");
						types.print_from_c_conversion_new_var(&ident, &field.ty, Some(&gen_types));
						print!("unsafe {{ &mut *((*this_ptr).inner as *mut ln{}) }}.{} = ", s.ident, ident);
						types.print_from_c_conversion_prefix(&field.ty, Some(&gen_types));
						print!("val");
						types.print_from_c_conversion_suffix(&field.ty, Some(&gen_types));
						println!(";\n}}");
					}
				}
			}
		}
	}

	types.struct_imported(&s.ident, struct_name.clone());
}

fn println_impl(i: &syn::ItemImpl, types: &TypeResolver) {
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
									println_docs(&m.attrs, "");
									print!("#[no_mangle]\npub extern \"C\" fn {}_{}(", ident, m.sig.ident);
									let ret_type = match declared_type {
										DeclType::MirroredEnum => format!("{}", ident),
										DeclType::StructImported(newname) => format!("{}", newname),
										_ => unimplemented!(),
									};
									print_method_params(&m.sig, &HashMap::new(), &ret_type, types, Some(&gen_types));
									print!(" {{\n\t");
									print_method_var_decl_body(&m.sig, "", types, Some(&gen_types), false);
									let mut takes_self = false;
									for inp in m.sig.inputs.iter() {
										if let syn::FnArg::Receiver(_) = inp {
											takes_self = true;
										}
									}
									if takes_self {
										print!("unsafe {{ &*(*this_arg).inner }}.{}(", m.sig.ident);
									} else {
										print!("lightning::{}::{}(", resolved_path, m.sig.ident);
									}
									print_method_call_params(&m.sig, types, Some(&gen_types), &ret_type, false);
									println!("\n}}\n");
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

fn println_enum(e: &syn::ItemEnum, module_path: &str, types: &mut TypeResolver, extra_headers: &mut File) {
	match export_status(&e.attrs) {
		ExportStatus::Export => {},
		ExportStatus::NoExport|ExportStatus::TestOnly => return,
		ExportStatus::Rename(_) => { unimplemented!(); },
	}

	for var in e.variants.iter() {
		if let syn::Fields::Unit = var.fields {} else {
			eprintln!("Skipping enum {} as it contains non-unit fields", e.ident);
			println_opaque(&e.ident, &format!("{}", e.ident), &e.generics, &e.attrs, module_path, types, extra_headers);
			types.enum_ignored(&e.ident);
			return;
		}
	}
	println_docs(&e.attrs, "");

	if e.generics.lt_token.is_some() {
		unimplemented!();
	}
	types.mirrored_enum_declared(&e.ident);
	println!("#[repr(C)]\npub enum {} {{", e.ident);
	for var in e.variants.iter() {
		assert_eq!(export_status(&var.attrs), ExportStatus::Export); // We can't partially-export a mirrored enum
		println_docs(&var.attrs, "\t");
		if let syn::Fields::Unit = var.fields {} else { unimplemented!(); }
		if var.discriminant.is_some() { unimplemented!(); }
		println!("\t{},", var.ident);
	}
	println!("}}\nuse {}::{} as ln{};\nimpl {} {{", module_path, e.ident, e.ident, e.ident);
	println!("\t#[allow(unused)]\n\tpub(crate) fn to_ln(&self) -> ln{} {{\n\t\tmatch self {{", e.ident);
	for var in e.variants.iter() {
		println!("\t\t\t{}::{} => ln{}::{},", e.ident, var.ident, e.ident, var.ident);
	}
	println!("\t\t}}\n\t}}");
	println!("\t#[allow(unused)]\n\tpub(crate) fn from_ln(lnt: ln{}) -> Self {{\n\t\tmatch lnt {{", e.ident);
	for var in e.variants.iter() {
		println!("\t\t\tln{}::{} => {}::{},", e.ident, var.ident, e.ident, var.ident);
	}
	println!("\t\t}}\n\t}}\n}}\n");
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 5 {
		eprintln!("Usage: file/path.rs orig_crate module::path extra/includes.h");
		process::exit(1);
	}

	let mut file = File::open(&args[1]).expect("Unable to open file");
	let mut src = String::new();
	file.read_to_string(&mut src).expect("Unable to read file");
	let syntax = syn::parse_file(&src).expect("Unable to parse file");

	let mut header_file = std::fs::OpenOptions::new().write(true).append(true)
		.open(&args[4]).expect("Unable to open new header file");

	assert!(syntax.shebang.is_none()); // Not sure what this is, hope we dont have one

	assert_eq!(export_status(&syntax.attrs), ExportStatus::Export);
	println_docs(&syntax.attrs, "");

	let mut type_resolver = TypeResolver::new(&args[3]);
	let orig_module = args[2].clone() + "::" + &args[3];
	// First pass, resolve types:

	println!("\nuse std::ffi::c_void;\nuse bitcoin::hashes::Hash;\n");

	for item in syntax.items.iter() {
		match item {
			syn::Item::Use(u) => type_resolver.process_use(&u),
			syn::Item::Static(_) => {},
			syn::Item::Enum(e) => {
				if let syn::Visibility::Public(_) = e.vis {
					println_enum(&e, &orig_module, &mut type_resolver, &mut header_file);
				}
			},
			syn::Item::Impl(i) => {
				println_impl(&i, &type_resolver);
			},
			syn::Item::Struct(s) => {
				if let syn::Visibility::Public(_) = s.vis {
					println_struct(&s, &orig_module, &mut type_resolver, &mut header_file);
				}
			},
			syn::Item::Trait(t) => {
				if let syn::Visibility::Public(_) = t.vis {
					println_trait(&t, &orig_module, &mut type_resolver);
				}
			},
			syn::Item::Mod(_m) => {
				//XXX
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
						println_opaque(&t.ident, &format!("{}", t.ident), &t.generics, &t.attrs, &orig_module, &type_resolver, &mut header_file);
					}
				}
			},
			syn::Item::Fn(_c) => {
			},
			syn::Item::Macro(_) => {},
			_ => unimplemented!(),
		}
	}
	header_file.flush().unwrap();
}
