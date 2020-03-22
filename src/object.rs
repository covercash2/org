use std::{fmt, fmt::Display};

use crate::{error, error::OrgError};

#[derive(Debug)]
pub struct OrgContent<'t> {
    pub text: &'t str,
    pub root: OrgObject<'t>,
}

impl<'t> OrgContent<'t> {
    pub fn objects(&self) -> Option<&Vec<OrgObject<'t>>> {
        match &self.root {
            OrgObject::Header(_header, objects) => Some(&objects),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum OrgObject<'t> {
    Header(Header<'t>, Vec<OrgObject<'t>>),
    List(Vec<ListItem<'t>>),
    Text(Vec<&'t str>),
}

impl<'t> OrgObject<'t> {
    pub fn is_headline(&self) -> bool {
        match self {
            OrgObject::Header(_, _) => true,
            _ => false,
        }
    }

    pub fn sub_objects(&self) -> Option<&Vec<OrgObject<'t>>> {
        match self {
            OrgObject::Header(_, sub_objects) => Some(sub_objects),
            _ => None,
        }
    }
}

impl<'t> From<Vec<ListItem<'t>>> for OrgObject<'t> {
    fn from(list_items: Vec<ListItem<'t>>) -> OrgObject<'t> {
        OrgObject::List(list_items)
    }
}

#[derive(Debug)]
pub struct Header<'t> {
    level: usize,
    title: &'t str,
    status: Option<&'t str>,
    tags: Option<Vec<&'t str>>,
}

#[derive(Debug)]
pub struct ListItem<'t> {
    pub bullet: Bullet,
    pub content: &'t str,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bullet {
    Minus,
    Plus,
    Numeric(usize),
}

impl Bullet {
    pub fn parse(s: &str) -> Option<Bullet> {
        match s {
            "-" => Some(Bullet::Minus),
            "+" => Some(Bullet::Plus),
            _ => s.split('.').next().and_then(|num_str| {
                num_str
                    .parse::<usize>()
                    .map(|num| Bullet::Numeric(num))
                    .ok()
            }),
        }
    }
}

impl<'t> Display for OrgObject<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrgObject::Header(header, objects) => {
                write!(f, "{}", header)?;

                for object in objects {
                    write!(f, "{}", object)?;
                }
            }
            OrgObject::Text(lines) => {
                for line in lines {
                    write!(f, "{}\n", line)?;
                }
            }
            OrgObject::List(list) => {
                for list_item in list {
                    write!(f, "{}", list_item)?;
                }
            }
        }
        return Ok(());
    }
}

impl<'t> Display for Header<'t> {
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

impl<'t> Header<'t> {
    pub fn new(
        level: usize,
        title: &'t str,
        status: Option<&'t str>,
        tags: Option<Vec<&'t str>>,
    ) -> Header<'t> {
        Header {
            level,
            title,
            status,
            tags,
        }
    }

    pub fn new_root() -> Header<'t> {
        Header {
            level: 0,
            title: "root",
            status: None,
            tags: None,
        }
    }

    pub fn simple_header(level: usize, title: &'t str) -> Header<'t> {
        Header {
            level,
            title,
            status: None,
            tags: None,
        }
    }

    pub fn todo(level: usize, status: &'t str, title: &'t str) -> Header<'t> {
        Header {
            level,
            title,
            status: Some(status),
            tags: None,
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }
}

impl<'t> Display for ListItem<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.bullet, self.content)
    }
}

impl Display for Bullet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bullet::Minus => write!(f, "-"),
            Bullet::Plus => write!(f, "+"),
            Bullet::Numeric(i) => write!(f, "{}.", i),
        }
    }
}
