use crate::{
    error,
    headline::Headline,
    object::{Bullet, ListItem},
};

const TAG_CHAR: char = ':';
const HEADER_CHAR: u8 = 42; // * characer code
                            // TODO add * to list bullets
                            // or not. that seems stupid
const UNORDERED_LIST_BULLETS: [&'static str; 2] = ["-", "+"]; // [-, +]

#[derive(Debug)]
pub enum Line<'t> {
    Text(&'t str),
    Header(Headline<'t>),
    ListItem(ListItem<'t>),
}

impl<'t> Line<'t> {
    fn is_list_item(&self) -> bool {
        match self {
            Line::ListItem(_) => true,
            _ => false,
        }
    }

    fn is_header(&self) -> bool {
        match self {
            Line::Header(_) => true,
            _ => false,
        }
    }
}

// TODO only parse_line should be pub

pub fn parse_line<'t>(line: &'t str, possible_states: &[&str]) -> Line<'t> {
    Headline::parse(line, possible_states)
        .map(|header| Line::Header(header))
        .or(parse_list_item(line))
        .unwrap_or(Line::Text(line))
}

pub fn parse_list_item<'t>(line: &'t str) -> Option<Line<'t>> {
    line.trim()
        .find(' ')
        .map(|space_index| line.split_at(space_index))
        .and_then(|(bullet_str, rem)| Bullet::parse(bullet_str).map(|bullet| (bullet, rem.trim())))
        .map(|(bullet, content)| ListItem { bullet, content })
        .map(|list_item| Line::ListItem(list_item))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STATES: [&str; 3] = ["TODO", "STARTED", "DONE"];

    const GOOD_HEADERS: [&str; 9] = [
        "* a simple header",
        "* TODO a simple todo header",
        "* TODO a simple todo header :with:tags:",
        "** second level header",
        "*** third level header",
        "** STARTED second level started todo",
        "* STARTED first level started todo",
        "* DONE first level done todo",
        "* STARTED first level started :with:tags",
    ];

    const GOOD_LIST_ITEMS: [&str; 4] = [
        "- list item",
        "+ plus list item",
        "1. numbered list item",
        "6. sixth list item",
    ];

    #[test]
    fn test_test() {
        assert!(true);
    }

    #[test]
    fn test_good_headers() {
        // check that all lines are headers
        assert!(GOOD_HEADERS.iter().all(|line: &&str| {
            match parse_line(line, &TEST_STATES) {
                Line::Header(_header) => true,
                _ => false,
            }
        }))
    }

    #[test]
    fn test_good_list_items() {
        assert!(GOOD_LIST_ITEMS.iter().all(|line: &&str| {
            match parse_line(line, &TEST_STATES) {
                Line::ListItem(_list_item) => true,
                _ => false,
            }
        }))
    }
}
