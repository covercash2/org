use std::str::Lines;

use super::{
    error,
    error::OrgError,
    object::{Header, ListItem, OrgContent, OrgObject},
};

mod line;

pub struct Parser<'t> {
    text: &'t str,
    states: Vec<&'t str>,
    iterator: Lines<'t>,
    current_line: Option<&'t str>,
}

pub fn parse_org_text<'t, I: Iterator<Item = &'t str>>(
    text: &'t str,
    status_labels: I,
) -> error::Result<OrgContent<'t>> {
    Parser::new(text, status_labels).parse()
}

impl<'t> Parser<'t> {
    pub fn new<I: Iterator<Item = &'t str>>(text: &'t str, possible_states: I) -> Parser<'t> {
        let mut iterator = text.lines();
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

    fn get_line(&self) -> Option<&'t str> {
        self.current_line
    }

    fn advance_iterator(&mut self) {
        self.current_line = self.iterator.next();
    }

    fn parse_objects(&mut self, header: Header<'t>) -> OrgObject<'t> {
        let mut objects: Vec<OrgObject> = Vec::new();

        while let Some(line) = self.get_line() {
            match line::parse_header_line(line, &self.states) {
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
                    // TODO aggregate text
                    let text = self.parse_content(line);

                    objects.push(text);
                    self.advance_iterator();
                }
            }
        }

        return OrgObject::Header(header, objects);
    }

    fn parse_content(&mut self, current_line: &'t str) -> OrgObject<'t> {
        line::parse_list_item(current_line)
            .map(|list_item| OrgObject::List(list_item))
            .unwrap_or(OrgObject::Text(current_line))
    }
}
