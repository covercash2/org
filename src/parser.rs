use std::iter::Enumerate;
use std::str::Lines;

use super::{
    error,
    error::OrgError,
    object::{Bullet, Header, ListItem, OrgContent, OrgObject},
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
        let root = self.parse_objects(root_header);

        if !root.is_header() {
            Err(OrgError::Unexpected(
                "Parser::parse_objects should return a OrgObject::Header".into(),
            ))
        } else {
            Ok(OrgContent {
                text: self.text,
                root,
            })
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

        while let Some((_line_num, line)) = self.get_line() {
            match line::parse_line(line, &self.states) {
                Line::Header(new_header) => {
                    if new_header.level() > header.level() {
                        objects.push(self.parse_objects(new_header));
                        self.advance_iterator();
                    } else {
                        return OrgObject::Header(new_header, objects);
                    }
                }
                Line::ListItem(new_list_item) => {
                    objects.push(OrgObject::List(new_list_item));
                    self.advance_iterator();
                }
                Line::Text(new_text) => {
                    objects.push(OrgObject::Text(new_text));
                    self.advance_iterator();
                }
            }
        }

        return OrgObject::Header(header, objects);
    }
}

trait Cursor<'t> {
    fn advance(&mut self) -> Option<Line<'t>>;
    fn current_line(&self) -> Option<&Line<'t>>;
    fn current_line_number(&self) -> Option<usize>;
}

struct OrgCursor<'t, F> {
    current_line: Option<Line<'t>>,
    current_line_number: Option<usize>,
    iterator: Enumerate<Lines<'t>>,
    transform: F,
}

impl<'t, F> OrgCursor<'t, F>
where
    F: Fn(RawLine<'t>) -> (usize, Line<'t>),
{
    fn new(text: &'t str, transform: F) -> error::Result<OrgCursor<'t, F>> {
        let mut iterator = text.lines().enumerate();
        let current_line = iterator.next().map(&transform);
        let (current_line_number, current_line) = current_line.ok_or(OrgError::ParseError(
            Some(0),
            "cannot parse empty text".to_string(),
        ))?;
        let (current_line_number, current_line) = (Some(current_line_number), Some(current_line));

        Ok(OrgCursor {
            current_line,
            current_line_number,
            iterator,
            transform,
        })
    }
}

impl<'t, F> Cursor<'t> for OrgCursor<'t, F>
where
    F: Fn(RawLine<'t>) -> (usize, Line<'t>),
{
    /// advance the iterator
    /// and return the current line (not the reference).
    fn advance(&mut self) -> Option<Line<'t>> {
        let last_line = self.current_line.take();

        match self.iterator.next().map(&self.transform) {
            Some((line_num, line)) => {
                self.current_line_number.replace(line_num);
                self.current_line.replace(line);
            }
            None => {
                self.current_line_number = None;
                self.current_line = None;
            }
        }

        last_line
    }

    fn current_line(&self) -> Option<&Line<'t>> {
        self.current_line.as_ref()
    }

    fn current_line_number(&self) -> Option<usize> {
        self.current_line_number
    }
}

fn parse_header_objects<'t, C: Cursor<'t>>(
    header: Header<'t>,
    cursor: &mut C,
    possible_states: &[&str],
) -> OrgObject<'t> {
    let mut objects = Vec::new();

    let mut bullet: Option<Bullet> = None;
    let mut list: Option<Vec<ListItem>> = None;
    let mut text: Option<Vec<&str>> = None;

    while let Some(line) = cursor.advance() {
        match line {
            Line::Header(new_header) => {
                // recurse and add subheader
                if let Some(Line::Header(new_header)) = cursor.advance() {
                    let sub_header = parse_header_objects(new_header, cursor, possible_states);
                    objects.push(sub_header);
                }
            }
            Line::ListItem(list_item) => list.get_or_insert(Vec::new()).push(list_item),
            Line::Text(text_line) => text.get_or_insert(Vec::new()).push(text_line),
        }
    }

    return OrgObject::Header(header, Vec::new());
}

fn parse_list<'t, C: Cursor<'t>>(cursor: &mut C) -> error::Result<Vec<ListItem<'t>>> {
    return Err(OrgError::Unexpected("not implemented".to_string()));
}

fn str_to_line<'t>(raw_line: RawLine<'t>, possible_states: &[&str]) -> (usize, Line<'t>) {
    (raw_line.0, line::parse_line(raw_line.1, possible_states))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STATES: [&str; 3] = ["TODO", "STARTED", "DONE"];

    const TEST_TEXT: &str = "* TODO task header: with_tags :tag:anothertag:
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

    const GOOD_LIST: &str = "- a good list
	- not necessarily in order
	- a third item";

    #[test]
    fn parse_test_str() {
        let content = parse_org_text(&TEST_TEXT, TEST_STATES.to_vec()).unwrap();
    }

    #[test]
    fn parse_lists() {
        let mut good_list_cursor =
            OrgCursor::new(GOOD_LIST, |raw_line| str_to_line(raw_line, &TEST_STATES)).unwrap();

        match parse_list(&mut good_list_cursor) {
            Err(e) => panic!("{}", e),
            Ok(_vec_list_item) => {}
        }
    }
}
