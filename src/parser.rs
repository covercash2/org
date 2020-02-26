use std::str::Lines;

use super::{
    object::{Header, OrgObject},
    OrgContent,
};

pub struct Parser<'t> {
    text: &'t str,
    states: Vec<&'t str>,
    iterator: Lines<'t>,
    current_line: Option<&'t str>,
}

const HEADER_CHAR: u8 = 42; // * characer code

impl<'t> Parser<'t> {
    pub fn new<I: Iterator<Item = &'t str>>(text: &'t str, possible_states: I) -> Parser<'t> {
        let mut iterator = text.lines();
        let current_line = iterator.next().expect("unable to read text");

        let states: Vec<&str> = possible_states.collect();

        Parser {
            text,
            states,
            iterator,
            current_line: Some(current_line),
        }
    }

    pub fn parse(&mut self) -> Result<OrgContent<'t>, &'static str> {
        let root_header = Header::new_root();

        match self.parse_objects(root_header) {
            OrgObject::Header(_root_header, objects) => Ok(OrgContent {
                text: self.text,
                objects,
            }),
            _ => Err("unexpected error: Parser::parse_objects should return a OrgObject::Header"),
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
                    if new_header.level() > header.level() {
                        objects.push(self.parse_objects(new_header));
                        self.advance_iterator();
                    } else {
                        return OrgObject::Header(new_header, objects);
                    }
                }
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
