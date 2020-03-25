#![feature(str_strip)]

use ncurses::*;

use std::{fs::File, io::prelude::*, io::BufReader};

mod config;
mod headline;
pub mod object;
pub mod parser;
pub mod error;

mod peek_while;

fn main() -> error::Result<()> {
    let config = config::Config::from_command_line_parameters()
        .expect("could not parse command line parameters");

    let file = File::open(config.file_path)?;
    let text = {
        let mut buf_reader = BufReader::new(file);
        let mut text = String::new();
        buf_reader.read_to_string(&mut text)?;
        text
    };

    let labels: Vec<&str> = config.status_labels.iter().map(AsRef::as_ref).collect();

    let content = parser::parse_org_text(&text, labels);

    content
        .map(|content| {
	    if let Some(objects) = content.objects() {
		for object in objects {
		    println!("{}", object);
		}
	    }
        })
        .expect("error parsing text");

    return Ok(());
}
