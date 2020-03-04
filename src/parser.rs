use std::str::Lines;

use super::{
    object::{Header, OrgContent, ListItem, OrgObject},
};

pub struct Parser<'t> {
    text: &'t str,
    states: Vec<&'t str>,
    iterator: Lines<'t>,
    current_line: Option<&'t str>,
}

const HEADER_CHAR: u8 = 42; // * characer code
                            // TODO add * to list bullets
                            // or not. that seems stupid
const UNORDERED_LIST_BULLETS: [&'static str; 2] = ["-", "+"]; // [-, +]

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
                    // TODO aggregate text
                    let text = self.parse_content(line);

                    println!("{:?}", text);

                    objects.push(text);
                    self.advance_iterator();
                }
            }
        }

        return OrgObject::Header(header, objects);
    }

    fn parse_content(&mut self, current_line: &'t str) -> OrgObject<'t> {
        parse_list_item(current_line)
            .map(|list_item| OrgObject::List(list_item))
            .unwrap_or(OrgObject::Text(current_line))
    }
}

fn parse_list_item(line: &str) -> Option<ListItem> {
    parse_unordered_item(line, &UNORDERED_LIST_BULLETS)
        .or_else(|| parse_ordered_item(line))
        .map(|(bullet, content)| ListItem { bullet, content })
        .or(None)
}

fn parse_ordered_item<'t>(line: &'t str) -> Option<(&'t str, &'t str)> {
    line.find('.')
        .map(|dot_index| line.split_at(dot_index + 1))
        .filter(|(bullet, _)| {
            // check if all digits are numbers between 0-9
            bullet
                .chars()
                .take_while(|&ch| ch != '.')
                .all(|ch| ch.is_digit(10))
        })
        .map(|(bullet, rem)| {
            // trim '.' and whitespace
            let text = rem[1..].trim();
            (bullet, text)
        })
}

fn parse_unordered_item<'t>(
    line: &'t str,
    possible_bullets: &[&str],
) -> Option<(&'t str, &'t str)> {
    possible_bullets
        .iter()
        .filter(|&bullet| line.starts_with(bullet))
        .map(|bullet| line.split_at(bullet.len()))
        .next()
        .map(|(bullet, rem)| (bullet, rem.trim()))
}

fn parse_status<'t>(level: usize, status: &str, line: &'t str) -> Header<'t> {
    let (label, text) = line.split_at(status.len());

    return Header::todo(level, label, text.trim());
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
        .unwrap_or(Header::simple_header(header_level, text));

    return Some(header);
}
