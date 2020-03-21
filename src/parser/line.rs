use crate::{
    error,
    object::{Bullet, Header, ListItem},
};

const TAG_CHAR: char = ':';
const HEADER_CHAR: u8 = 42; // * characer code
                            // TODO add * to list bullets
                            // or not. that seems stupid
const UNORDERED_LIST_BULLETS: [&'static str; 2] = ["-", "+"]; // [-, +]

pub enum Line<'t> {
    Text(&'t str),
    Header(Header<'t>),
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
    parse_header_line(line, possible_states)
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

/// parse status from the front of `text`
/// given the possible states.
/// returns the status and the remaining text, respectively,
/// or None if no status is found.
pub fn parse_status<'t>(text: &'t str, possible_states: &[&str]) -> Option<(&'t str, &'t str)> {
    possible_states
        .iter()
        .find(|&&state| text.starts_with(state))
        .map(|state| text.split_at(state.len()))
        .map(|(status, text)| (status, text.trim()))
}

/// parse tags from the end of `text` and
/// return the trimmed `text` and the tags
/// or `None` if there are no tags
fn parse_tags<'t>(text: &'t str) -> Option<(Vec<&'t str>, &'t str)> {
    text.trim()
        .rfind(char::is_whitespace)
        .map(|i| text.split_at(i))
        .and_then(|(text, tag_str)| {
            let tags: Vec<&'t str> = tag_str
                .split_terminator(TAG_CHAR)
                .filter(|tag| !tag.contains(' '))
                .filter(|tag| tag.len() != 0)
                .collect();

            if tags.len() == 0 {
                None
            } else {
                Some((tags, text.trim()))
            }
        })
}

pub fn parse_header_line<'t>(line: &'t str, possible_states: &[&str]) -> Option<Header<'t>> {
    let level = line.bytes().take_while(|byte| byte == &HEADER_CHAR).count();

    if level == 0 {
        return None;
    }

    // trim header markers, '*'
    let (_, text) = line.split_at(level);
    let text = text.trim();

    let (status, text) = match parse_status(text, possible_states) {
        Some((status, rem)) => (Some(status), rem),
        None => (None, text),
    };

    let (tags, text) = match parse_tags(text) {
        Some((tags, rem)) => (Some(tags), rem),
        None => (None, text),
    };

    let title = text;

    let header = Header::new(level, title, status, tags);

    return Some(header);
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
