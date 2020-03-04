#![feature(str_strip)]

use ncurses::*;

use std::{fs::File, io::prelude::*, io::BufReader};

mod config;
pub mod object;
pub mod parser;

fn main() -> std::io::Result<()> {
    let config = config::Config::from_command_line_parameters()
        .expect("could not parse command line parameters");

    let file = File::open(config.file_path)?;
    let text = {
        let mut buf_reader = BufReader::new(file);
        let mut text = String::new();
        buf_reader.read_to_string(&mut text)?;
        text
    };

    let label_iter = config.status_labels.iter().map(AsRef::as_ref);

    let mut parser = parser::Parser::new(&text, label_iter);

    let content = parser.parse();

    content
        .map(|content| {
            for object in content.objects {
                println!("{}", object);
            }
        })
        .expect("error parsing text");

    return Ok(());
}
