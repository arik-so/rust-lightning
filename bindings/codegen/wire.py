import os
import re

import config

from rust_types import handle_type, imports, all_types, unhandled_types


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
		method_signature = f"wire_message_create_{struct_name}({argument_string}) -> *const WireMessage{struct_details['name']}"
		field_names = ',\n\t\t'.join(field['name'] for field in struct_details['fields'])

		type_wrapper = f"pub struct WireMessage{struct_details['name']}(Raw{struct_details['name']});"

		raw_constructor_signature = f"wire_message_{struct_name}_from_raw(message: &Raw{struct_details['name']}) -> *const WireMessage{struct_details['name']}"
		raw_constructor_body = f"\tBox::into_raw(Box::new(WireMessage{struct_details['name']}(message.clone())))"

		method_body = ""
		for field in struct_details['fields']:
			type_handlers = handle_type(field['type'])
			if type_handlers['input_converter'] is not None:
				input_converter = type_handlers['input_converter']
				current_input_converter = input_converter.replace('{name}', field['name'])
				method_body += current_input_converter + "\n\n"

		method_body += f"\tlet message = Raw{struct_details['name']} {{ \n\t\t{field_names}\n\t}};\n\tBox::into_raw(Box::new(WireMessage{struct_details['name']}(message)))"
		# return f'{type_wrapper}\n\n#[no_mangle]\npub extern "C" fn {raw_constructor_signature} {{\n{raw_constructor_body}\n}}\n\n#[no_mangle]\npub extern "C" fn {method_signature} {{\n{method_body}\n}}'
		return f'{type_wrapper}\n\npub extern "C" fn {raw_constructor_signature} {{\n{raw_constructor_body}\n}}\n\n#[no_mangle]\npub extern "C" fn {method_signature} {{\n{method_body}\n}}'

	def generate_getters():
		getters = []
		for current_field in struct_details['fields']:
			field_name = camel_to_snake_case(current_field['name'])
			getter_name = f"wire_message_{struct_name}_get_{field_name}"
			type_handlers = handle_type(current_field['type'])
			getter_type = type_handlers['return_type']
			if getter_type is None:
				continue
			getter_signature = f"{getter_name}(message: &WireMessage{struct_details['name']}) -> {getter_type}"
			converter = type_handlers['output_converter']
			getter_body = f"message.0.{current_field['name']}"
			if converter is not None:
				getter_body = converter.replace('{field}', getter_body)
			getter = f"#[no_mangle]\npub extern \"C\" fn {getter_signature} {{\n\t{getter_body}\n}}"
			getters.append(getter)
		return getters

	return {
		'constructor': generate_constructor(),
		'getters': generate_getters()
	}


def generate_bindings(messages):
	bindings_file = os.path.dirname(__file__) + "/../../bindings/src/peers/gen_wire.rs"
	bindings_code = ""
	for struct_code in messages:
		struct_details = parse_struct(struct_code)
		if config.FILTER_MESSAGES and struct_details['name'] not in config.SUPPORTED_MESSAGES:
			continue  # let's get a few types working first

		imports.add(f"lightning::ln::msgs::{struct_details['name']} as Raw{struct_details['name']}")
		binding = generate_struct_binding(struct_details)

		bindings_code += f"\n{binding['constructor']}\n\n"

		bindings_code += '\n\n'.join(binding['getters']) + '\n\n'

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
		regex = r"pub ([a-z_][a-z_0-9]*): ([^,\n]+)"
		matches = re.finditer(regex, struct_code, re.MULTILINE)

		fields = []
		for matchNum, match in enumerate(matches, start=1):
			field_name = match.group(1)
			field_type = match.group(2)
			field = {'name': field_name, 'type': field_type}
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

	regex_filter = r"pub struct [a-zA-Z\-]+ {[^}]*}"
	matches = re.finditer(regex_filter, messages_code, re.MULTILINE)

	message_structs = []

	for matchNum, match in enumerate(matches, start=1):
		current_struct = match.group()
		message_structs.append(current_struct)

	return message_structs


def camel_to_snake_case(str):
	res = [str[0].lower()]
	for i in range(1, len(str)):
		current_char = str[i]

		previous_char = None
		next_char = None
		if i > 0:
			previous_char = str[i - 1]
		if i < len(str) - 1:
			next_char = str[i + 1]

		if current_char.isupper() and previous_char is not None:
			if previous_char.islower() or (next_char is not None and next_char.islower()):
				res.append('_')
				res.append(current_char.lower())
				continue
		res.append(current_char.lower())

	return ''.join(res)


message_structs = scan_wire_messages()
generate_bindings(message_structs)

# print(camel_to_snake_case('iOS'))
# print(camel_to_snake_case('addHTLCHere'))
# print(camel_to_snake_case('AddHTLC'))
print("All seen types:", all_types)
print("Unhandled types:", unhandled_types)
