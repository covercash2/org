use std::{fmt, fmt::Display};

#[derive(Debug)]
pub enum OrgObject<'t> {
    Header(Header<'t>, Vec<OrgObject<'t>>),
    List(ListItem<'t>),
    Text(&'t str),
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

#[derive(Debug)]
pub struct Header<'t> {
    level: usize,
    title: &'t str,
    status: Option<&'t str>,
}

impl<'t> Display for Header<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_indicator = "*".repeat(self.level);

        match self.status {
            Some(status) => write!(f, "{} {} {}", level_indicator, status, self.title),
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
        }
    }

    pub fn header(level: usize, title: &'t str) -> Header<'t> {
        return Header {
            level: level,
            title: title,
            status: None,
        };
    }

    pub fn todo(level: usize, status: &'t str, title: &'t str) -> Header<'t> {
        return Header {
            level: level,
            status: Some(status),
            title: title,
        };
    }

    pub fn level(&self) -> usize {
        self.level
    }
}

#[derive(Debug)]
pub struct ListItem<'t> {
    pub bullet: &'t str,
    pub content: &'t str,
}

impl<'t> Display for ListItem<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{} {}", self.bullet, self.content)
    }
}
