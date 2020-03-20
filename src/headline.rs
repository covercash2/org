use std::{fmt, fmt::Display};

use crate::object::OrgObject;

pub struct HeadlineGroup<'t> {
    headline: Headline<'t>,
    objects: Option<Vec<OrgObject<'t>>>,
    subheaders: Option<Vec<Headline<'t>>>,
}

pub struct Headline<'t> {
    pub level: usize,
    pub title: &'t str,
    pub status: Option<&'t str>,
    pub tags: Option<Vec<&'t str>>,
}

impl<'t> Headline<'t> {
    pub fn parse(line: &'t str, possible_states: &[&str]) -> Option<Headline<'t>> {
        parse_headline(line, possible_states)
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

    let header = Headline {
        level,
        title,
        status,
        tags,
    };

    return Some(header);
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
