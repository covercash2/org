#![feature(str_strip)]

use ncurses::*;

use std::{fs::File, io::prelude::*, io::BufReader, str::{FromStr, Lines}};

const PATH: &'static str = "/mnt/space/notes/emacs.org";

const HEADER_CHAR: u8 = 42; // * character code

const STATUS_LABELS: [&'static str; 3] = ["TODO", "STARTED", "DONE"];

#[derive(Debug)]
struct OrgContent<'t> {
	text: &'t str,
	objects: Vec<OrgObject<'t>>,
}

#[derive(Debug)]
enum OrgObject<'t> {
	Header(Header<'t>, Vec<OrgObject<'t>>),
	Text(&'t str),
}

#[derive(Debug)]
struct Header<'t> {
	level: usize,
	title: &'t str,
	status: Option<&'t str>,
}

impl<'t> Header<'t> {
	fn new_root() -> Header<'t> {
		Header {
			level: 0,
			title: "root",
			status: None,
		}
	}

	fn header(level: usize, title: &'t str) -> Header<'t> {
		return Header {
			level: level,
			title: title,
			status: None,
		};
	}

	fn todo(level: usize, status: &'t str, title: &'t str) -> Header<'t> {
		return Header {
			level: level,
			status: Some(status),
			title: title,
		};
	}
}

fn parse_status<'t>(level: usize, status: &str, line: &'t str) -> Header<'t> {
	let (label, text) = line.split_at(status.len());

	return Header::todo(level, label, text);
}

fn parse_header_line<'t>(line: &'t str, possible_states: &[&str]) -> Option<Header<'t>> {
	let header_level = line.bytes().take_while(|byte| byte == &HEADER_CHAR).count();

	if header_level == 0 {
		return None;
	}

	let (_, text) = line.split_at(header_level);
	let text = text.trim();

	let header: Header = possible_states
		.iter()
		.find(|&&label| text.starts_with(label))
		.map(|&label| parse_status(header_level, label, text))
		.unwrap_or(Header::header(header_level, text));

	return Some(header);
}

struct Parser<'t> {
	text: &'t str,
	states: Vec<&'t str>,
	iterator: Lines<'t>,
	current_line: Option<&'t str>,
}

impl<'t> Parser<'t> {
	fn new(text: &'t str, possible_states: Vec<&'t str>) -> Parser<'t> {
		let mut iterator = text.lines();
		let current_line = iterator.next().expect("unable to read text");

		Parser {
			text: text,
			states: possible_states,
			iterator: iterator,
			current_line: Some(current_line),
		}
	}

	fn parse(&mut self) -> Result<OrgContent<'t>, &'static str> {
		let root_header = Header::new_root();

		match self.parse_objects(root_header) {
			OrgObject::Header(_root_header, objects) => {
				Ok(OrgContent {
					text: self.text,
					objects: objects,
				})
			},
			_ => {
				Err("unexpected error: Parser::parse_objects should return a OrgObject::Header")
			}
		}
	}

	fn get_line(&self) -> Option<&'t str> {
		self.current_line
	}

	fn advance_iterator(&mut self) {
		self.current_line = self.iterator.next();
	}

	fn parse_objects(&mut self, header: Header<'t>) -> OrgObject<'t> {
		let mut objects: Vec<OrgObject> = Vec::new();

		while let Some(line) = self.get_line() {
			match parse_header_line(line, &self.states) {
				Some(new_header) => {
					if new_header.level > header.level {
						objects.push(self.parse_objects(new_header));
						self.advance_iterator();
					} else {
						return OrgObject::Header(new_header, objects);
					}
				},
				None => {
					// just text
					let text = OrgObject::Text(line);
					objects.push(text);
					self.advance_iterator();
				}
			}
		}

		return OrgObject::Header(header, objects);
	}
}

fn main() -> std::io::Result<()> {
	let file = File::open(PATH)?;
	let text = {
		let mut buf_reader = BufReader::new(file);
		let mut text = String::new();
		buf_reader.read_to_string(&mut text)?;
		text
	};

	let mut parser = Parser::new(
		&text,
		STATUS_LABELS.to_vec(),
	);

	let content = parser.parse();

	return Ok(());
}
