use std::iter::Enumerate;
use std::str::Lines;

use super::{
    content::{Bullet, Content, ListItem},
    error,
    error::OrgError,
    headline::{Headline, HeadlineGroup},
    object::Document,
};

mod line;
use line::Line;

type RawLine<'t> = (usize, &'t str);

pub fn parse_org_text<'t, I: IntoIterator<Item = &'t str>>(
    text: &'t str,
    status_labels: I,
) -> error::Result<Document<'t>> {
    let labels: Vec<&str> = status_labels.into_iter().collect();
    let mut cursor = OrgCursor::new(text, |raw_line| raw_line_to_line(raw_line, &labels))?;
    let root: HeadlineGroup = parse_headline_objects(Headline::new_root(), &mut cursor, &labels)?;
    Ok(Document { text, root })
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

fn parse_headline_objects<'t, C: Cursor<'t>>(
    headline: Headline<'t>,
    cursor: &mut C,
    possible_states: &[&str],
) -> error::Result<HeadlineGroup<'t>> {
    let mut content: Option<LimitedVec<Content<'t>>> = None;
    let mut sub_headlines: Option<LimitedVec<HeadlineGroup<'t>>> = None;

    while let Some(line) = cursor.current_line() {
        match line {
            Line::Header(new_headline) => {
                if new_headline.level() > headline.level() {
                    let new_header = cursor
                        .advance()
                        .and_then(|line| match line {
                            Line::Header(headline) => Some(headline),
                            _ => None,
                        })
                        .ok_or(OrgError::ParseError(
                            cursor.current_line_number(),
                            "cursor returned a bad value".to_string(),
                        ))?;

                    // recurse and add subheader
                    let sub_header = parse_headline_objects(new_header, cursor, possible_states)?;
                    sub_headlines
                        .get_or_insert(Default::default())
                        .push(sub_header)?;
                } else {
                    break;
                }
            }
            Line::ListItem(_) => {
                content
                    .get_or_insert(Default::default())
                    .push(parse_list(cursor).into())?;
            }
            Line::Text(_) => {
                let text = parse_text(cursor);
                content.get_or_insert(Default::default()).push(text)?;
            }
        }
    }

    return Ok(HeadlineGroup {
        headline,
        content: content.map(LimitedVec::take),
        sub_headlines: sub_headlines.map(LimitedVec::take),
    });
}

struct LimitedVec<T> {
    vec: Vec<T>,
    limit: usize,
}

impl<T> Default for LimitedVec<T> {
    fn default() -> Self {
        LimitedVec {
            vec: Vec::new(),
            limit: 64, // TODO 64 might be a little low
        }
    }
}

impl<T> LimitedVec<T> {
    fn take(self) -> Vec<T> {
        self.vec
    }
}

trait Limited<T> {
    fn limit(&self) -> usize;
    fn push(&mut self, item: T) -> error::Result<()>;
}
impl<T> Limited<T> for LimitedVec<T> {
    fn limit(&self) -> usize {
        self.limit
    }
    fn push(&mut self, item: T) -> error::Result<()> {
        if self.vec.len() < self.limit() {
            self.vec.push(item);
            Ok(())
        } else {
            Err(error::OrgError::Unexpected(format!(
                "capacity reached in limited vec: limit == {}",
                self.limit()
            )))
        }
    }
}

fn parse_list<'t, C: Cursor<'t>>(cursor: &mut C) -> Vec<ListItem<'t>> {
    let mut list_items = Vec::new();
    let mut bullet_opt: Option<Bullet> = None;
    while let Some(Line::ListItem(item)) = cursor.current_line() {
        let bullet = bullet_opt.get_or_insert(item.bullet);
        if !item.bullet.matches(bullet) {
            break;
        }
        match cursor.advance() {
            Some(Line::ListItem(item)) => list_items.push(item),
            _ => eprintln!("unexpected error"),
        }
    }
    return list_items;
}

fn parse_text<'t, C: Cursor<'t>>(cursor: &mut C) -> Content<'t> {
    let mut text_lines = Vec::new();
    while let Some(Line::Text(_)) = cursor.current_line() {
        match cursor.advance() {
            Some(Line::Text(line)) => text_lines.push(line),
            _ => eprintln!("unexpected error"),
        }
    }
    Content::Text(text_lines)
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
10. ten
"; // end with a new line

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
        println!("{}", TEST_TEXT);

        let content = parse_org_text(&TEST_TEXT, TEST_STATES.to_vec()).unwrap();

        println!("parsed output:");
        println!("{}", content);

        println!("objects:");
        for object in content.objects() {
            println!("{:?}", object);
        }

        let headlines: Vec<_> = content.headlines().collect();

        let object_num: usize = content.objects().count();

        let expected_headline_num: usize = 5;

        assert_eq!(headlines.len(), expected_headline_num);

        let expected_objects: usize = 9;

        assert_eq!(object_num, expected_objects);

        let expected_lines: usize = 26;
        let test_text_lines = TEST_TEXT.lines().count();
        let content_lines = format!("{}", content).lines().count();

        assert_eq!(test_text_lines, expected_lines);
        assert_eq!(content_lines, test_text_lines);
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
