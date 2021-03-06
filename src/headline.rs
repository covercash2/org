use std::{fmt, fmt::Display};

use crate::{content::Content, iter::*, object::Object};

#[derive(Debug)]
pub struct HeadlineGroup<'t> {
    pub headline: Headline<'t>,
    pub content: Option<Vec<Content<'t>>>,
    pub sub_headlines: Option<Vec<HeadlineGroup<'t>>>,
}

impl<'t> HeadlineGroup<'t> {
    pub fn content(&'t self) -> impl Iterator<Item = &'t Content<'t>> {
        self.content.iter().flat_map(|content| content.iter())
    }

    pub fn content_len(&self) -> usize {
        self.content
            .as_ref()
            .map(|content| content.len())
            .unwrap_or(0)
    }

    pub fn sub_headlines(&'t self) -> SubHeadlines<'_> {
        (&self.sub_headlines).into()
    }

    pub fn sub_headlines_len(&'t self) -> usize {
        self.sub_headlines
            .as_ref()
            .map(|sub_headlines| sub_headlines.len())
            .unwrap_or(0)
    }

    pub fn sub_objects(&'t self) -> SubObjects<'_> {
        self.into_iter()
    }

    pub fn all_headlines(&self) -> Headlines<'_> {
        self.sub_headlines().into()
    }

    pub fn all_objects(&'t self) -> impl Iterator<Item = Object<'t>> {
        AllObjects::new(self)
    }
}

#[derive(Debug)]
pub struct Headline<'t> {
    level: usize,
    title: &'t str,
    status: Option<&'t str>,
    tags: Option<Vec<&'t str>>,
}

impl<'t> Headline<'t> {
    pub fn new_root() -> Self {
        Headline {
            level: 0,
            title: "root",
            status: None,
            tags: None,
        }
    }

    pub fn parse(line: &'t str, possible_states: &[&str]) -> Option<Headline<'t>> {
        parse_headline(line, possible_states)
    }

    pub fn level(&self) -> usize {
        self.level
    }
    pub fn title(&self) -> &'t str {
        self.title
    }
}

fn parse_headline<'t>(line: &'t str, possible_states: &[&str]) -> Option<Headline<'t>> {
    let level = line.chars().take_while(|&ch| ch == '*').count();

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

    let headline = Headline {
        level,
        title,
        status,
        tags,
    };

    return Some(headline);
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
                .split_terminator(':')
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

impl<'t> Display for Headline<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_indicator = "*".repeat(self.level);
        let tag_string = self.tags.as_ref().map(|tags| tags.join(":"));

        match (self.status, tag_string) {
            (Some(status), None) => write!(f, "{} {} {}", level_indicator, status, self.title),
            (None, Some(tag_string)) => {
                write!(f, "{} {} {}", level_indicator, self.title, tag_string,)
            }
            (Some(status), Some(tag_string)) => write!(
                f,
                "{} {} {} :{}:",
                level_indicator, status, self.title, tag_string,
            ),
            (None, None) => write!(f, "{} {}", level_indicator, self.title),
        }
    }
}

impl<'t> Display for HeadlineGroup<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.headline)?;

        match &self.content {
            Some(content) => {
                for object in content {
                    write!(f, "{}", object)?;
                }
            }
            _ => {}
        }

        for headline in self.sub_headlines() {
            write!(f, "{}", headline)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_states: [&str; 3] = ["TODO", "STARTED", "DONE"];

    const count_up_headlines: &str = "* 1
** 2
* 3
** 4
* 5
* 6
** 7
*** 8
** 9
*** 10
** 11
* 12";

    const good_headlines: [&str; 3] = [
        "* a good headline",
        "* TODO a good headline with a todo",
        "** a second level headline",
    ];

    #[test]
    fn test_parsing() {
        assert!(good_headlines.iter().all(|headline_str| Headline::parse(
            headline_str,
            &test_states
        )
        .is_some()))
    }

    #[test]
    fn headline_iterator() {
        let content = crate::parser::parse_org_text(count_up_headlines, test_states.to_vec())
            .expect("could not parse test string");

        let sub_headlines: Vec<&HeadlineGroup<'_>> = content.root.sub_headlines().collect();

        assert!(
            sub_headlines.len() == 5,
            "wrong number of sub-headlines. expected: 5, found {}",
            sub_headlines.len()
        );

        let headlines: Vec<&HeadlineGroup> = content.headlines().collect();

        assert!(
            headlines.len() == 12,
            "wrong number of headlines. expected: 12, found {}",
            headlines.len()
        );
    }
}
