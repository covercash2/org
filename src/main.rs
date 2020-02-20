#![feature(str_strip)]

use ncurses::*;

use std::{fs::File, io::prelude::*, io::BufReader};

const PATH: &'static str = "/mnt/space/notes/emacs.org";

const HEADER_CHAR: u8 = 42; // * character code

const STATUS_LABELS: [&'static str; 3] = ["TODO", "STARTED", "DONE"];

#[derive(Debug)]
struct OrgContent<'t> {
	text: &'t String,
	objects: Vec<OrgObject<'t>>,
}

#[derive(Debug)]
enum OrgObject<'t> {
	Header(Header<'t>),
	TodoHeader(TodoHeader<'t>),
	Text(&'t str),
}

#[derive(Debug)]
struct Header<'t> {
	level: usize,
	text: &'t str,
}

#[derive(Debug)]
struct TodoHeader<'t> {
	level: usize,
	status: &'t str,
	text: &'t str,
}

impl<'t> OrgObject<'t> {
	fn header(level: usize, text: &'t str) -> OrgObject<'t> {
		return OrgObject::Header(Header {
			level: level,
			text: text,
		});
	}

	fn todo(level: usize, status: &'t str, text: &'t str) -> OrgObject<'t> {
		return OrgObject::TodoHeader(TodoHeader {
			level: level,
			status: status,
			text: text,
		});
	}
}

fn parse_header<'t, I: Iterator<Item = &'t &'t str>>(
	possible_states: &mut I,
	line: &'t str,
) -> Option<OrgObject<'t>> {
	let header_level = line.bytes().take_while(|byte| byte == &HEADER_CHAR).count();

	if header_level == 0 {
		return None;
	}

	let (_, text) = line.split_at(header_level);
	let text = text.trim();

	let status_label: Option<&str> = possible_states
		.find(|&&label| text.starts_with(label))
		.map(|&label| label);

	let object = match status_label {
		Some(label) => {
			let (label, text) = text.split_at(label.len());
			OrgObject::todo(header_level, label, text.trim())
		}
		None => OrgObject::header(header_level, text.trim()),
	};

	return Some(object);
}

fn parse_line<'t>(line: &'t str) -> OrgObject<'t> {
	let mut possible_states = STATUS_LABELS.iter();

	return parse_header(&mut possible_states, line)
		.unwrap_or(OrgObject::Text(line));
}

fn parse_org_text<'t>(text: &'t String) -> OrgContent<'t> {
	let lines = text.split("\n");

	let mut objects = Vec::new();

	for line in lines {
		let object = parse_line(line);
		println!("{:?}", object);
		objects.push(object);
	}

	return OrgContent {
		text: text,
		objects: objects,
	};
}

fn main() -> std::io::Result<()> {
	let file = File::open(PATH)?;
	let text = {
		let mut buf_reader = BufReader::new(file);
		let mut text = String::new();
		buf_reader.read_to_string(&mut text)?;
		text
	};

	let content = parse_org_text(&text);

	return Ok(());
}
