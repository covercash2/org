use std::str::Lines;

use super::{
    error,
    error::OrgError,
    object::{Header, ListItem, OrgContent, OrgObject},
};

mod line;
use line::Line;

type RawLine<'t> = (usize, &'t str);

pub struct Parser<'t> {
    text: &'t str,
    states: Vec<&'t str>,
    iterator: std::iter::Enumerate<Lines<'t>>,
    current_line: Option<RawLine<'t>>,
}

pub fn parse_org_text<'t, I: IntoIterator<Item = &'t str>>(
    text: &'t str,
    status_labels: I,
) -> error::Result<OrgContent<'t>> {
    Parser::new(text, status_labels.into_iter()).parse()
}

impl<'t> Parser<'t> {
    pub fn new<I: Iterator<Item = &'t str>>(text: &'t str, possible_states: I) -> Parser<'t> {
        let mut iterator = text.lines().enumerate();
        let current_line = iterator.next().expect("text is empty");

        let states: Vec<&str> = possible_states.collect();

        Parser {
            text,
            states,
            iterator,
            current_line: Some(current_line),
        }
    }

    pub fn parse(&mut self) -> error::Result<OrgContent<'t>> {
        let root_header = Header::new_root();

        match self.parse_objects(root_header) {
            OrgObject::Header(_root_header, objects) => Ok(OrgContent {
                text: self.text,
                objects,
            }),
            _ => Err(OrgError::Unexpected(
                "Parser::parse_objects should return a OrgObject::Header".into(),
            )),
        }
    }

    fn get_line(&self) -> Option<RawLine<'t>> {
        self.current_line
    }

    fn advance_iterator(&mut self) {
        self.current_line = self.iterator.next();
    }

    fn parse_objects(&mut self, header: Header<'t>) -> OrgObject<'t> {
        let mut objects: Vec<OrgObject> = Vec::new();

        while let Some((line_num, line)) = self.get_line() {
            match line::parse_line(line_num, line, &self.states) {
                Line::Header(_line_num, new_header) => {
                    if new_header.level() > header.level() {
                        objects.push(self.parse_objects(new_header));
                        self.advance_iterator();
                    } else {
                        return OrgObject::Header(new_header, objects);
                    }
                }
                Line::ListItem(_line_num, new_list_item) => {
                    objects.push(OrgObject::List(new_list_item));
                    self.advance_iterator();
                }
                Line::Text(_line_num, new_text) => {
                    objects.push(OrgObject::Text(new_text));
                    self.advance_iterator();
                }
            }
        }

        return OrgObject::Header(header, objects);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STATES: [&str; 3] = ["TODO", "STARTED", "DONE"];

    const TEST_STR: &str = "* TODO task header: with_tags :tag:anothertag:
	:DEADLINE: 

	:PROPERTIES:
	:END:

	* STARTED started task
	* TODO unorodered lists
	- not necessarily first
	- maybe not second
	- doesn't have to be third
	* plus sign list
	+ unordered lists
	+ don't have to start
	+ with a - like a sane person
	* STARTED ordered lists
	1. there
	2. needs
	3. to
	4. be
	5. ten
	6. of
	7. these
	8. so
	9. here's
	10. ten";

    #[test]
    fn parse_test_str() {
        let content = parse_org_text(&TEST_STR, TEST_STATES.to_vec()).unwrap();
    }
}
