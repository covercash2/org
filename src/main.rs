#![feature(str_strip)]

use ncurses::*;

use std::{fs::File, io::prelude::*, io::BufReader, str::{FromStr, Lines}};

pub mod object;
pub mod parser;

const PATH: &'static str = "/mnt/space/notes/emacs.org";

const STATUS_LABELS: [&'static str; 3] = ["TODO", "STARTED", "DONE"];

#[derive(Debug)]
pub struct OrgContent<'t> {
	text: &'t str,
	objects: Vec<object::OrgObject<'t>>,
}

fn main() -> std::io::Result<()> {
	let file = File::open(PATH)?;
	let text = {
		let mut buf_reader = BufReader::new(file);
		let mut text = String::new();
		buf_reader.read_to_string(&mut text)?;
		text
	};

	let mut parser = parser::Parser::new(
		&text,
		STATUS_LABELS.to_vec(),
	);

	let content = parser.parse();

	println!("{:?}", content);

	return Ok(());
}
