import os
import re

all_types = set()
unhandled_types = set()
imports = set()

def handle_type(type):
	# for debugging
	all_types.add(type)
	is_handled = False

	argument = type
	return_type = None
	input_converter = None
	output_converter = None

	if type in ['u32', 'u16', 'u8', 'u64']:
		return_type = type
		is_handled = True
	elif type in ['[u8; 32]', 'Signature', 'PublicKey', 'Sha256dHash']:
		argument = '*const u8'
		is_handled = True

		imports.add('std::slice')

		input_converter = """\tlet {name} = unsafe {
\t\tassert!(!{name}.is_null());
\t\tslice::from_raw_parts({name}, 32)
\t};"""

		if type == 'Sha256dHash':
			imports.add('bitcoin_hashes::Hash')
			imports.add('bitcoin_hashes::sha256d::Hash as Sha256dHash')

			input_converter += "\n\tlet {name} = Sha256dHash::from_slice({name}).unwrap();"
		elif type == 'PublicKey':
			imports.add('secp256k1::PublicKey')

			input_converter += "\n\tlet {name} = PublicKey::from_slice({name}).unwrap();"
		elif type == '[u8; 32]':
			input_converter = """\tlet {name}_slice = unsafe {
\t\tassert!(!{name}.is_null());
\t\tslice::from_raw_parts({name}, 32)
\t};

\tlet mut {name} = [0u8; 32];
\t{name}.copy_from_slice({name}_slice);"""
		elif type == 'Signature':
			imports.add('secp256k1::Signature')

			input_converter = """\tlet {name} = unsafe {
\t\tassert!(!{name}.is_null());
\t\tslice::from_raw_parts({name}, 65)
\t};

\tlet {name} = Signature::from_der(&{name}).unwrap();"""



	elif type == 'OptionalField<Script>':
		# argument = '&BufferArgument'
		# imports.add('crate::buffer::BufferArgument')
		argument = None
		input_converter = "\tlet {name} = OptionalField::Absent;"
		imports.add('lightning::ln::msgs::OptionalField')

		return_type = None

		is_handled = True
	elif type == '&\'static str':
		argument = '*const c_char'
		imports.add('std::os::raw::c_char')
		is_handled = True

	if not is_handled:
		unhandled_types.add(type)

	return {
		'argument': argument,
		'return_type': return_type,
		'input_converter': input_converter,
		'output_converter': output_converter
	}

def generate_field_argument(field):
	field_name = field['name']
	field_type = field['type']
	type_details = handle_type(field_type)
	if type_details['argument'] is None:
		return None
	argument = f"{field_name}: {type_details['argument']}"
	return argument

def generate_struct_binding(struct_details):
	struct_name = camel_to_snake_case(struct_details['name'])

	def generate_constructor():
		arguments = [generate_field_argument(field) for field in struct_details['fields']]
		filtered_arguments = filter(None, arguments)
		argument_string = ', '.join(filtered_arguments)
		method_signature = f"wire_message_create_{struct_name}({argument_string}) -> *const {struct_details['name']}"
		field_names = ',\n\t\t'.join(field['name'] for field in struct_details['fields'])

		method_body = ""
		for field in struct_details['fields']:
			type_handlers = handle_type(field['type'])
			if type_handlers['input_converter'] is not None:
				input_converter = type_handlers['input_converter']
				current_input_converter = input_converter.replace('{name}', field['name'])
				method_body += current_input_converter + "\n\n"

		method_body += f"\tlet message = {struct_details['name']} {{ \n\t\t{field_names}\n\t}};\n\tBox::into_raw(Box::new(message))"
		return f'#[no_mangle]\npub extern "C" fn {method_signature} {{\n{method_body}\n}}'

	def generate_getters():
		getters = []
		for current_field in struct_details['fields']:
			field_name = camel_to_snake_case(current_field['name'])
			getter_name = f"wire_message_{struct_name}_get_{field_name}"
			type_handlers = handle_type(current_field['type'])
			getter_type = type_handlers['return_type']
			if getter_type is None:
				continue
			getter_signature = f"{getter_name}(message: &{struct_details['name']}) -> {getter_type}"
			converter = type_handlers['output_converter']
			getter_body = f"message.{current_field['name']}"
			if converter is not None:
				pass
			getter = f"#[no_mangle]\npub extern \"C\" fn {getter_signature} {{\n\t{getter_body}\n}}"
			print(getter)
			getters.append(getter)
		return getters

	print(generate_constructor())

	return {
		'constructor': generate_constructor(),
		'getters': generate_getters()
	}

def generate_bindings(messages):
	bindings_file = os.path.dirname(__file__) + "/../../bindings/src/peers/gen_wire.rs"
	bindings_code = ""
	for struct_code in messages:
		struct_details = parse_struct(struct_code)
		if struct_details['name'] not in ['Ping', 'Pong', 'OpenChannel', 'AcceptChannel', 'FundingCreated', 'FundingSigned', 'FundingLocked']:
			continue # let's get a few types working first

		imports.add(f"lightning::ln::msgs::{struct_details['name']}")
		binding = generate_struct_binding(struct_details)

		bindings_code += f"\n{binding['constructor']}\n\n"

		bindings_code += '\n\n'.join(binding['getters']) + '\n\n'

		print(struct_details)
		print(binding)

	bindings_code = '\n'.join([f"use {current_import};" for current_import in imports]) + "\n\n" + bindings_code

	f = open(bindings_file, "w")
	f.write(bindings_code)
	f.close()


def parse_struct(struct_code):
	# extract name
	def parse_name():
		regex = r"pub struct ([a-zA-Z\-]+) {"
		matches = re.finditer(regex, struct_code, re.MULTILINE)
		match = next(matches)
		group = match.group(1)
		return group

	def parse_fields():
		regex = r"pub ([a-z_]+): ([^,\n]+)"
		matches = re.finditer(regex, struct_code, re.MULTILINE)

		fields = []
		for matchNum, match in enumerate(matches, start=1):
			field_name = match.group(1)
			field_type = match.group(2)
			field = { 'name': field_name, 'type': field_type }
			fields.append(field)
		return fields

	return {
		'name': parse_name(),
		'fields': parse_fields()
	}


def scan_wire_messages():
	messages_file = os.path.dirname(__file__) + "/../../lightning/src/ln/msgs.rs"
	with open(messages_file) as f:
		messages_code = f.read()
	# print(messages_code)

	regex_filter = r"pub struct [a-zA-Z\-]+ {[^}]*}"
	matches = re.finditer(regex_filter, messages_code, re.MULTILINE)
	print(matches)

	message_structs = []

	for matchNum, match in enumerate(matches, start=1):
		current_struct = match.group()
		message_structs.append(current_struct)

	# print ("Match {matchNum} was found at {start}-{end}: {match}".format(matchNum = matchNum, start = match.start(), end = match.end(), match = match.group()))
	#
	# for groupNum in range(0, len(match.groups())):
	# 	groupNum = groupNum + 1
	#
	# 	print ("Group {groupNum} found at {start}-{end}: {group}".format(groupNum = groupNum, start = match.start(groupNum), end = match.end(groupNum), group = match.group(groupNum)))

	return message_structs

def camel_to_snake_case(str):
	res = [str[0].lower()]
	for i in range(1, len(str)):
		current_char = str[i]

		previous_char = None
		next_char = None
		if i > 0:
			previous_char = str[i-1]
		if i < len(str)-1:
			next_char = str[i+1]

		if current_char.isupper() and previous_char is not None:
			if previous_char.islower() or (next_char is not None and next_char.islower()):
				res.append('_')
				res.append(current_char.lower())
				continue
		res.append(current_char.lower())

	return ''.join(res)

message_structs = scan_wire_messages()
generate_bindings(message_structs)

print(camel_to_snake_case('iOS'))
print(camel_to_snake_case('addHTLCHere'))
print(camel_to_snake_case('AddHTLC'))
print(all_types)
print(unhandled_types)
