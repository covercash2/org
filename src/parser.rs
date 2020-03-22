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

pub fn parse_org_text<'t, I: IntoIterator<Item = &'t str>>(
    text: &'t str,
    status_labels: I,
) -> error::Result<OrgContent<'t>> {
    let labels: Vec<&str> = status_labels.into_iter().collect();
    let mut cursor = OrgCursor::new(text, |raw_line| raw_line_to_line(raw_line, &labels))?;
    let root = parse_header_objects(Header::new_root(), &mut cursor, &labels);
    Ok(OrgContent { text, root })
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

    while let Some(line) = cursor.current_line() {
        println!("{:?}", line);
        match line {
            Line::Header(_) => {
                if let Some(Line::Header(new_header)) = cursor.advance() {
                    // if `new_header` is a subheader
                    if new_header.level() > header.level() {
                        // recurse and add subheader
                        let sub_header = parse_header_objects(new_header, cursor, possible_states);
                        objects.push(sub_header);
                    } else {
                        break;
                    }
                }
            }
            Line::ListItem(_) => {
                objects.push(parse_list(cursor).into());
            }
            Line::Text(_) => {
                objects.push(parse_text(cursor));
            }
        }
    }

    return OrgObject::Header(header, Vec::new());
}

fn parse_list<'t, C: Cursor<'t>>(cursor: &mut C) -> Vec<ListItem<'t>> {
    let mut list_items = Vec::new();
    let mut bullet_opt: Option<Bullet> = None;
    while let Some(Line::ListItem(list_item)) = cursor.advance() {
        println!("list item: {}", list_item);
        let bullet = bullet_opt.get_or_insert(list_item.bullet);
        if list_item.bullet == *bullet {
            list_items.push(list_item)
        }
    }
    return list_items;
}

fn parse_text<'t, C: Cursor<'t>>(cursor: &mut C) -> OrgObject<'t> {
    let mut text_lines = Vec::new();
    while let Some(Line::Text(text_line)) = cursor.advance() {
        println!("text line: {}", text_line);
        text_lines.push(text_line);
    }
    OrgObject::Text(text_lines)
}

fn raw_line_to_line<'t>(raw_line: RawLine<'t>, possible_states: &[&str]) -> (usize, Line<'t>) {
    let (line_num, line) = raw_line;
    (line_num, line::parse_line(line, possible_states))
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

    const GOOD_LIST_0: &str = "- a good list
- not necessarily in order
- a third item";
    const GOOD_LIST_0_LEN: usize = 3;
    const GOOD_LIST_0_BULLET: Bullet = Bullet::Minus;

    const GOOD_LIST_1: &str = "+ another good list
+ featuring the plus sign
+ also, now with over 3 items
+ why have more than 1 indicator?
+ who knows";
    const GOOD_LIST_1_LEN: usize = 5;
    const GOOD_LIST_1_BULLET: Bullet = Bullet::Plus;

    #[test]
    fn parse_test_str() {
        let content = parse_org_text(&TEST_TEXT, TEST_STATES.to_vec()).unwrap();

        let headlines = content
            .root
            .sub_objects()
            .unwrap()
            .iter()
            .filter(|object| object.is_headline());
        let expected_headline_num = 5;
        assert!(headlines.count() == expected_headline_num);

        println!("{}", content.root);
    }

    fn check_list<'t>(
        list: &Vec<ListItem<'t>>,
        expected_len: usize,
        expected_bullet: Bullet,
    ) -> error::Result<()> {
        if list.len() != expected_len {
            return Err(OrgError::Unexpected(format!(
                "incorrect list length. expected: {}, found: {}",
                expected_len,
                list.len()
            )));
        }

        check_bullets(list, expected_bullet)
    }

    fn check_bullets<'t>(list: &Vec<ListItem<'t>>, expected_bullet: Bullet) -> error::Result<()> {
        match expected_bullet {
            Bullet::Numeric(_) => {
                match list
                    .iter()
                    .enumerate()
                    .find(|(num, item)| match item.bullet {
                        Bullet::Numeric(i) => *num != (i + 1),
                        _ => false,
                    }) {
                    Some((bad_bullet_index, bad_bullet)) => Err(OrgError::ParseError(
                        None,
                        format!(
                            "expected numeric bullet: {}, found: {}",
                            bad_bullet_index + 1,
                            bad_bullet,
                        ),
                    )),
                    _ => Ok(()),
                }
            }
            _ => match list.iter().find(|item| item.bullet != expected_bullet) {
                Some(item) => Err(OrgError::ParseError(
                    None,
                    format!(
                        "bad bullet was parsed: {}, expected: {}",
                        item.bullet, expected_bullet,
                    ),
                )),
                None => Ok(()),
            },
        }
    }

    fn get_cursor<'t>(s: &'t str) -> error::Result<impl Cursor<'t>> {
        OrgCursor::new(s, |raw_line| raw_line_to_line(raw_line, &TEST_STATES))
    }

    #[test]
    fn parse_lists() {
        for line in GOOD_LIST_0.lines() {
            println!("line: {}", line);
        }

        let mut cursor = get_cursor(GOOD_LIST_0).unwrap();

        let good_list_0 = parse_list(&mut cursor);
        check_list(&good_list_0, GOOD_LIST_0_LEN, GOOD_LIST_0_BULLET).unwrap();

        let mut cursor = get_cursor(GOOD_LIST_1).unwrap();

        let good_list_1 = parse_list(&mut cursor);
        check_list(&good_list_1, GOOD_LIST_1_LEN, GOOD_LIST_1_BULLET).unwrap();
    }
}
