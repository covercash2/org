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
	Header(Header<'t>, Option<&'t OrgObject<'t>>),
	Text(&'t str),
}

#[derive(Debug)]
struct Header<'t> {
	level: usize,
	text: &'t str,
	status: Option<&'t str>,
}

impl<'t> Header<'t> {
	fn header(
		level: usize,
		text: &'t str,
	) -> Header<'t> {
		return Header {
			level: level,
			text: text,
			status: None,
		};
	}

	fn todo(
		level: usize,
		status: &'t str,
		text: &'t str,
	) -> Header<'t> {
		return Header {
			level: level,
			text: text,
			status: Some(status),
		};
	}
}

fn parse_status<'t>(
	level: usize,
	status: &'static str,
	line: &'t str
) -> Header<'t> {
	let (label, text) = line.split_at(status.len());

	return Header::todo(
		level, label, text
	);
}

fn parse_header<'t>(
	possible_states: &'t [&'static str],
	line: &'t str,
) -> Option<Header<'t>> {
	println!("{}", line);

	let header_level = line.bytes().take_while(|byte| byte == &HEADER_CHAR).count();

	if header_level == 0 {
		return None;
	}

	let (_, text) = line.split_at(header_level);
	let text = text.trim();

	let header: Header = possible_states
		.iter()
		.find(|&&label| {
			text.starts_with(label)
		})
		.map(|&label| {
			parse_status(header_level, label, text)
		})
		.unwrap_or(Header::header(header_level, text));

	return Some(header);
}

fn parse_org_text<'t>(text: &'t String) -> OrgContent<'t> {
	let lines = text.split("\n");

	let mut objects = Vec::new();

	let mut current_header: Option<OrgObject> = None;

	let possible_states = &STATUS_LABELS;

	for line in lines {
		let header = parse_header(possible_states, line);
		println!("{:?}", header);
		//objects.push(object);
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
