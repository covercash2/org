use std::{fmt, fmt::Display};

#[derive(Debug)]
pub struct OrgContent<'t> {
    pub text: &'t str,
    pub objects: Vec<OrgObject<'t>>,
}

#[derive(Debug)]
pub enum OrgObject<'t> {
    Header(Header<'t>, Vec<OrgObject<'t>>),
    List(ListItem<'t>),
    Text(&'t str),
}

#[derive(Debug)]
pub struct Header<'t> {
    level: usize,
    title: &'t str,
    status: Option<&'t str>,
    tags: Vec<&'t str>,
}

#[derive(Debug)]
pub struct ListItem<'t> {
    pub bullet: &'t str,
    pub content: &'t str,
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
            OrgObject::Text(string) => {
                write!(f, "{}", string)?;
            }
            OrgObject::List(list_item) => {
                write!(f, "{}", list_item)?;
            }
        }
        return Ok(());
    }
}

impl<'t> Display for Header<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_indicator = "*".repeat(self.level);
        let tag_string = self.tags.join(":");

        match self.status {
            Some(status) => write!(
                f,
                "{} {} {} {}",
                level_indicator, status, self.title, tag_string
            ),
            None => write!(f, "{} {}", level_indicator, self.title),
        }
    }
}

impl<'t> Header<'t> {
    pub fn new_root() -> Header<'t> {
        Header {
            level: 0,
            title: "root",
            status: None,
            tags: vec![],
        }
    }

    pub fn simple_header(level: usize, title: &'t str) -> Header<'t> {
        Header {
            level,
            title,
            status: None,
            tags: vec![],
        }
    }

    pub fn todo(level: usize, status: &'t str, title: &'t str) -> Header<'t> {
        Header {
            level,
            title,
            status: Some(status),
            tags: vec![],
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
