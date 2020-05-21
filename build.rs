use glob;
use krpc_mars_terraformer;
use tera::{Context, Tera};

use ron::de::from_reader;
use serde::Deserialize;
use std::fmt::Write as FmtWrite;
use std::io::Write;
use std::{collections::HashMap, fs::File};

use itertools::Itertools;

const LOAD_KRPC: bool = true;
const LOAD_PROTOCOL: bool = true;

#[derive(Debug, Deserialize)]
struct Protocol {
	values: Vec<(String, HashMap<String, String>)>,
}

enum Length {
	Known(usize),
	Unknown(),
}

fn incr(l: Length) -> Length {
	if let Length::Known(n) = l {
		return Length::Known(n + 1);
	} else {
		return l;
	}
}

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	if LOAD_KRPC {
		for path in glob::glob("krpc/*.json").unwrap().filter_map(Result::ok) {
			println!("cargo:rerun-if-changed={}", path.display());
		}

		krpc_mars_terraformer::run("krpc/", "src/krpc").expect("Could not terraform Mars :(");
	}
	if LOAD_PROTOCOL {
		load_protocol();
	}
}

#[allow(unused_must_use)]
fn load_protocol() {
	let input_path = format!("{}/protocol.ron", env!("CARGO_MANIFEST_DIR"));
	println!("cargo:rerun-if-changed={}", input_path);
	let f = File::open(&input_path).expect("Failed opening file");
	let protocol: Protocol = match from_reader(f) {
		Ok(x) => x,
		Err(e) => {
			println!("Failed to load protocol: {}", e);

			std::process::exit(1);
		}
	};

	let mut header = File::create(format!("{}/protocol.h", env!("CARGO_MANIFEST_DIR")))
		.expect("Error creating file");
	let mut rust = File::create(format!("{}/src/protocol.rs", env!("CARGO_MANIFEST_DIR")))
		.expect("Error creating file");
	let mut i = 0;
	let mut enum_elements = String::new();
	let mut repr_match_contents = String::new();
	let mut parser_token_new = String::new();
	let mut parser_token_end = String::new();
	let mut consts = String::new();
	let mut typenames = Vec::new();
	let mut header_ends = Vec::new();
	for (val, args) in protocol.values {
		let mut inner_parser_token_end = String::new();
		let mut parser_check_done = String::new();
		let mut complex_parser_check_done = String::new();
		let mut header_end = String::new();
		let mut header_end_done = String::new();
		let mut header_complex_check = String::new();
		let split = val.split_at(1);
		let classname = format!(
			"{}{}",
			split.0.to_string().to_uppercase(),
			split.1.to_string().to_lowercase()
		);
		typenames.push(classname.clone());
		writeln!(header, "// {} {:?}", val, args);
		// Enum / struct
		writeln!(header, "#define {}_VAL_ID {}", val.to_uppercase(), i);
		writeln!(
			consts,
			"pub const {}_VAL_ID: usize = {};",
			val.to_uppercase(),
			i
		);
		writeln!(
			parser_token_new,
			"{}_VAL_ID => self.kind = {}_VAL_ID,",
			val.to_uppercase(),
			val.to_uppercase()
		);
		writeln!(header, "typedef struct {{");
		let rs_match_key = format!(
			"{}({})",
			classname,
			args.clone()
				.iter()
				.map(|(name, _t)| name.clone())
				.collect::<Vec<String>>()
				.join(",")
		);

		writeln!(header_end, "case {}_VAL_ID:{{", val.to_uppercase());
		writeln!(header_end, "bool done = false;");
		let mut length = Length::Known(1);

		write!(enum_elements, "\t{}(", classname);
		for key in args.keys().sorted() {
			let t = args[key].clone();
			// writeln!(header, "// {} {}", key, t)?;
			let c = match t.as_ref() {
				"string" => "String".to_string(),
				"analog" => "word".to_string(),
				_ => t.clone(),
			};
			let r = match t.as_ref() {
				"string" => "String".to_string(),
				"analog" => "u16".to_string(),
				_ => t,
			};
			write!(enum_elements, "{},", r);
			writeln!(header, "\t{} {};", c, key);
		}
		writeln!(enum_elements, "),");
		writeln!(header, "}} {}Val;", classname);
		writeln!(
			header,
			"typedef void (* On{}Val)({}Val);\n",
			classname, classname
		);

		// Header repr fn
		writeln!(
			header,
			"String repr_{}Val({}Val v) {{",
			classname, classname
		);
		writeln!(header, "\tString s = \"\" + char({});", i);
		writeln!(repr_match_contents, "Self::{} => {{", rs_match_key);
		writeln!(parser_token_end, "{}_VAL_ID => {{", val.to_uppercase());
		writeln!(inner_parser_token_end, "let mut last_index = 1;");
		writeln!(header_end_done, "int last_index = 1;");
		writeln!(
			repr_match_contents,
			"res.push({}_VAL_ID as u8);",
			val.to_uppercase()
		);
		writeln!(complex_parser_check_done, "let mut byte_count = 1;");
		writeln!(header_complex_check, "int byte_count = 1;");
		let mut bool_names = Vec::new();
		for name in args.keys().sorted() {
			let t = args[name].clone();
			if t == "bool" {
				bool_names.push(name);
			} else if t == "string" {
				writeln!(header, "\ts += v.{};", name);
				writeln!(header, "\ts += char(255);");
				writeln!(
					repr_match_contents,
					"res.append(&mut {}.bytes().collect::<Vec<u8>>().clone());",
					name
				);
				writeln!(repr_match_contents, "res.push(255);");

				writeln!(inner_parser_token_end, "let mut {} = String::new();", name);
				writeln!(header_end_done, "String {} = \"\";", name);
				writeln!(
					inner_parser_token_end,
					"while self.tokens[last_index] != 255 {{"
				);
				writeln!(
					header_end_done,
					"while (tokens[last_index] != char(255)) {{"
				);
				writeln!(
					inner_parser_token_end,
					"{}.push(self.tokens[last_index] as char);",
					name
				);
				writeln!(
					header_end_done,
					"{} += char(tokens[last_index]);",
					name
				);
				writeln!(inner_parser_token_end, "last_index += 1;");
				writeln!(inner_parser_token_end, "}}");
				writeln!(inner_parser_token_end, "last_index += 1;");
				writeln!(header_end_done, "last_index ++;");
				writeln!(header_end_done, "}}");
				writeln!(header_end_done, "last_index ++;");

				writeln!(
					complex_parser_check_done,
					"if byte_count + 1 >= self.tokens.len() {{ return None }}"
				);
				writeln!(
					complex_parser_check_done,
					"while self.tokens[byte_count] != 255 {{"
				);
				writeln!(complex_parser_check_done, "byte_count += 1;");
				writeln!(
					complex_parser_check_done,
					"if byte_count >= self.tokens.len() {{ return None }}"
				);
				writeln!(complex_parser_check_done, "}}");
				writeln!(complex_parser_check_done, "byte_count += 1;");
				writeln!(header_complex_check, "while(tokens.charAt(byte_count) != char(255)) {{
					byte_count++;
					if (byte_count >= tokens.length()) {{ return; }}
				}}
				byte_count++;");
				length = Length::Unknown();
			} else if t == "analog" {
				// 10 bit -> word
				writeln!(header, "\ts += char(v.{} >> 8);", name);
				writeln!(header, "\ts += char(v.{} & 255);", name);
				writeln!(repr_match_contents, "res.push(({} >> 8) as u8);", name);
				writeln!(repr_match_contents, "res.push(({} & 255) as u8);", name);
				writeln!(inner_parser_token_end, "let mut {} = 0_u16;", name);
				writeln!(header_end_done, "int {} = 0;", name);
				writeln!(
					inner_parser_token_end,
					"{} |= (self.tokens[last_index] as u16) << 8;",
					name
				);
				writeln!(
					header_end_done,
					"{} |= int(tokens[last_index]) << 8;",
					name
				);
				writeln!(inner_parser_token_end, "last_index += 1;");
				writeln!(header_end_done, "last_index += 1;");
				writeln!(
					inner_parser_token_end,
					"{} |= self.tokens[last_index] as u16;",
					name
				);
				writeln!(
					header_end_done,
					"{} |= int(tokens[last_index]);",
					name
				);
				writeln!(inner_parser_token_end, "last_index += 1;");
				writeln!(header_end_done, "last_index += 1;");
				writeln!(complex_parser_check_done, "byte_count += 2;");
				writeln!(header_complex_check, "byte_count += 2;");
				length = incr(length);
				length = incr(length);
			}
		}
		if bool_names.len() > 0 {
			let mut m = 0;
			for k in 0..((bool_names.len() - bool_names.len() % 8) / 8) {
				write!(header, "\ts += char(0");
				write!(repr_match_contents, "res.push(0");
				for j in (k * 8)..(k * 8 + 8) {
					write!(header, "|(int(v.{})<<{})", bool_names[k * 8 + j], j);
					write!(
						repr_match_contents,
						"|((*{} as u8)<<({} as u8))",
						bool_names[k * 8 + j],
						j
					);
					writeln!(
						inner_parser_token_end,
						"let {} = (self.tokens[last_index] & 1 << {}) != 0;",
						bool_names[k * 8 + j],
						j
					);
					writeln!(
						header_end_done,
						"bool {} = (tokens[last_index] & (1 << {})) != 0;",
						bool_names[k * 8 + j],
						j
					);
				}
				writeln!(inner_parser_token_end, "last_index += 1;");
				writeln!(header_end_done, "last_index += 1;");
				writeln!(header, ");");
				writeln!(repr_match_contents, ");");
				writeln!(complex_parser_check_done, "byte_count += 1;");
				writeln!(header_complex_check, "byte_count++;");
				length = incr(length);
				m = k;
			}
			write!(header, "\ts += char(0");
			write!(repr_match_contents, "res.push(0");
			for j in 0..(bool_names.len() % 8) {
				write!(header, "|(int(v.{})<<{})", bool_names[m * 8 + j], j);
				write!(
					repr_match_contents,
					"|((*{} as u8)<<({} as u8))",
					bool_names[m * 8 + j],
					j
				);
				writeln!(
					inner_parser_token_end,
					"let {} = (self.tokens[last_index] & 1 << {}) != 0;",
					bool_names[m * 8 + j],
					j
				);
				writeln!(
					header_end_done,
					"bool {} = (tokens[last_index] & (1 << {})) != 0;",
					bool_names[m * 8 + j],
					j
				);
			}
			writeln!(inner_parser_token_end, "last_index += 1;");
			writeln!(header_end_done, "last_index += 1;");
			writeln!(complex_parser_check_done, "byte_count += 1;");
			writeln!(header_complex_check, "byte_count++;");
			length = incr(length);
			writeln!(header, ");");
			writeln!(repr_match_contents, ");");
		}

		writeln!(repr_match_contents, "}},");
		writeln!(header, "\treturn s;");
		writeln!(header, "}}");

		if let Length::Known(n) = length {
			writeln!(header, "#define {}_VAL_LEN {}", val.to_uppercase(), n);
			writeln!(
				consts,
				"pub const {}_VAL_LEN: usize = {};",
				val.to_uppercase(),
				n
			);
			writeln!(
				parser_check_done,
				"done = self.tokens.len() == {}_VAL_LEN;",
				val.to_uppercase()
			);
			writeln!(header_end, "done = (tokens.length() == {}_VAL_LEN);", val.to_uppercase());
		} else {
			writeln!(
				complex_parser_check_done,
				"done = self.tokens.len() == byte_count;"
			);
			write!(parser_check_done, "{}", complex_parser_check_done);
			writeln!(
				header_complex_check,
				"done = tokens.length() == byte_count;"
			);
			write!(header_end, "{}", header_complex_check);
		}
		writeln!(parser_token_end, "let mut done = false;");
		write!(parser_token_end, "{}", parser_check_done);
		writeln!(parser_token_end, "if done {{");
		write!(parser_token_end, "{}", inner_parser_token_end);
		writeln!(parser_token_end, "return Some(Value::{});", rs_match_key);
		writeln!(parser_token_end, "}}}},");

		writeln!(header_end_done, "{}Val ret;", classname);
		write!(header_end_done, "{}", args.iter().map(|(name, _)| format!("ret.{} = {};", name, name)).collect::<Vec<String>>().join("\n"));
		writeln!(header_end_done, "on{}Val(ret);tokens=\"\";", classname);

		writeln!(header_end, "if(done) {{\n{}}}", header_end_done);
		writeln!(header_end, "break;}}");
		header_ends.push(header_end);
		i += 1;
	}

	writeln!(
		header,
		"class Parser {{
	public:
		Parser({});
		void parse(byte);
	private:
		String tokens;
		{};
}};",
		typenames
			.iter()
			.map(|x| format!("On{}Val", x))
			.collect::<Vec<String>>()
			.join(", "),
		typenames
			.iter()
			.map(|x| format!("On{}Val on{}Val", x, x))
			.collect::<Vec<String>>()
			.join(";\n")
	);

	writeln!(
		header,
		"Parser::Parser({}) {{
	tokens = \"\";
	{}
}};",
		typenames
			.iter()
			.map(|x| format!("On{}Val tmpOn{}Val", x, x))
			.collect::<Vec<String>>()
			.join(", "),
		typenames
			.iter()
			.map(|x| format!("on{}Val = tmpOn{}Val;", x, x))
			.collect::<Vec<String>>()
			.join("\n")
	);

	writeln!(
		header,
		"void Parser::parse(byte b) {{
	if(tokens.length() == 0) {{
		switch (b) {{
			{}
			default:
				break;
		}}
	}} else {{
		tokens += char(b);
		switch(int(tokens.charAt(0))) {{
			{}
			default:
				break;
		}}
	}}
}};",
		typenames
			.iter()
			.map(|x| format!(
				"case {}_VAL_ID:\ntokens += char(b);break;",
				x.to_uppercase()
			))
			.collect::<Vec<String>>()
			.join("\n"),
		header_ends.join("\n")
	);
	let tera = match Tera::new("templates/*.rs") {
		Ok(t) => t,
		Err(e) => {
			println!("Parsing error(s): {}", e);
			::std::process::exit(1);
		}
	};
	let mut ctx = Context::new();
	ctx.insert("enum_elements", &enum_elements);
	ctx.insert("repr_match_contents", &repr_match_contents);
	ctx.insert("parser_token_new", &parser_token_new);
	ctx.insert("parser_token_end", &parser_token_end);
	ctx.insert("consts", &consts);
	let rendered = tera.render("protocol.rs", &ctx).expect("Rendering error");
	writeln!(rust, "{}", rendered);
}
