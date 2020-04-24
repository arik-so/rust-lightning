all_types = set()
unhandled_types = set()
imports = set()


def fixed_length_byte_reader(length, extension=''):
	template = """\tlet {extended_name} = unsafe {
\t\tassert!(!{name}.is_null());
\t\tslice::from_raw_parts({name}, {length})
\t};"""
	named_template = template.replace('{extended_name}', '{name}' + extension)
	return named_template.replace('{length}', str(length))


def dynamic_length_byte_reader(extension=''):
	template = '\tlet {extended_name} = unsafe { {name}.to_vec() };'
	return template.replace('{extended_name}', '{name}' + extension)


def handle_type(rust_type):
	# for debugging
	all_types.add(rust_type)
	is_handled = False

	argument = rust_type
	return_type = None
	input_converter = None
	output_converter = None

	if rust_type in ['u32', 'u16', 'u8', 'u64']:
		return_type = rust_type
		is_handled = True
	elif rust_type in ['[u8; 32]', 'Signature', 'PublicKey', 'Sha256dHash']:
		argument = '*const u8'
		is_handled = True

		imports.add('std::slice')

		input_converter = fixed_length_byte_reader(32)

		if rust_type == 'Sha256dHash':
			imports.add('bitcoin_hashes::Hash')
			imports.add('bitcoin_hashes::sha256d::Hash as Sha256dHash')

			input_converter += "\n\tlet {name} = Sha256dHash::from_slice({name}).unwrap();"
		elif rust_type == 'PublicKey':
			imports.add('secp256k1::PublicKey')

			input_converter = fixed_length_byte_reader(33)
			input_converter += '\n\n\tlet {name} = PublicKey::from_slice({name}).unwrap();'

			return_type = '*const u8'
			output_converter = "{field}.serialize().as_ptr()"

		elif rust_type == '[u8; 32]':
			input_converter = fixed_length_byte_reader(32, '_slice')
			input_converter += "\n\n\tlet mut {name} = [0u8; 32];"
			input_converter += "\n\t{name}.copy_from_slice({name}_slice);"

			return_type = '*const u8'
			output_converter = "{field}.as_ptr()"

		elif rust_type == 'Signature':
			imports.add('secp256k1::Signature')

			input_converter = fixed_length_byte_reader(65)
			input_converter += '\n\n\tlet {name} = Signature::from_der(&{name}).unwrap();'

			return_type = '*const u8'
			output_converter = "{field}.serialize_der().as_ptr()"

	elif rust_type == 'Script':
		argument = '&BufferArgument'
		imports.add('crate::buffer::BufferArgument')
		imports.add('bitcoin::Script')

		input_converter = dynamic_length_byte_reader()
		input_converter += '\n\tlet {name} = Script::from({name});'

	elif rust_type == 'OptionalField<Script>':
		# argument = '&BufferArgument'
		# imports.add('crate::buffer::BufferArgument')
		argument = None
		input_converter = "\tlet {name} = OptionalField::Absent;"
		imports.add('lightning::ln::msgs::OptionalField')

		return_type = None

		is_handled = True
	elif rust_type == '&\'static str':
		argument = '*const c_char'
		imports.add('std::os::raw::c_char')
		is_handled = True

	if not is_handled:
		unhandled_types.add(rust_type)

	return {
		'argument': argument,
		'return_type': return_type,
		'input_converter': input_converter,
		'output_converter': output_converter
	}
