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

def dynamic_length_byte_array_reader(extension=''):
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
	elif rust_type in ['[u8; 32]', 'Signature', 'PublicKey', 'Sha256dHash', 'PaymentHash', 'PaymentPreimage']:
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

		elif rust_type == 'PaymentHash':
			imports.add('lightning::ln::channelmanager::PaymentHash')

			input_converter = fixed_length_byte_reader(32, '_slice')
			input_converter += "\n\n\tlet mut {name} = [0u8; 32];"
			input_converter += "\n\t{name}.copy_from_slice({name}_slice);"
			input_converter += "\n\n\tlet {name} = PaymentHash({name});"

			return_type = '*const u8'
			output_converter = "{field}.0.as_ptr()"

		elif rust_type == 'PaymentPreimage':
			imports.add('lightning::ln::channelmanager::PaymentPreimage')

			input_converter = fixed_length_byte_reader(32, '_slice')
			input_converter += "\n\n\tlet mut {name} = [0u8; 32];"
			input_converter += "\n\t{name}.copy_from_slice({name}_slice);"
			input_converter += "\n\n\tlet {name} = PaymentPreimage({name});"

			return_type = '*const u8'
			output_converter = "{field}.0.as_ptr()"

	elif rust_type == 'Script':
		argument = '&BufferArgument'
		imports.add('crate::buffer::BufferArgument')
		imports.add('crate::buffer::BufferResponse')
		imports.add('bitcoin::Script')

		input_converter = dynamic_length_byte_reader()
		input_converter += '\n\tlet {name} = Script::from({name});'

		return_type = '*mut BufferResponse'
		output_converter = 'let buffer: BufferResponse = {field}.clone().as_bytes().into();'
		output_converter += '\n\tbuffer.into_mut_ptr()'

		is_handled = True

	elif rust_type == 'Vec<Signature>':
		argument = '&BufferArgumentArray'
		imports.add('crate::buffer::BufferArgumentArray')
		imports.add('crate::buffer::BufferResponseArray')
		imports.add('secp256k1::Signature')

		input_converter = '\t// For the sake of convenience, each element can be of variable length'
		input_converter += '\n'+dynamic_length_byte_array_reader('_raw')
		input_converter += '''\n\tlet mut {name} = vec![];
\tfor current_signature in {name}_raw {
\t\t{name}.push(Signature::from_der(&current_signature).unwrap());
\t}'''

		return_type = '*mut BufferResponseArray'

		output_converter = '''let mut buffers = vec![];
\tfor current_signature in {field}.iter() {
\t\tbuffers.push(current_signature.serialize_der().to_vec());
\t}
\tlet buffer_response: BufferResponseArray = buffers.into();
\tbuffer_response.into_mut_ptr()'''

		# output_converter = '// Script objects need to be cloned to be returned'
		# output_converter += '\n\tlet buffer: BufferResponse = {field}.clone().into_bytes().into();'
		# output_converter += '\n\tbuffer.into_mut_ptr()'

		is_handled = True

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
