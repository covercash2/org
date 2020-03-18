use std::{fmt, fmt::Display};

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
    List(ListItem<'t>),
    Text(&'t str),
}

impl<'t> OrgObject<'t> {
    pub fn is_header(&self) -> bool {
	match self {
	    OrgObject::Header(_, _) => true,
	    _ => false,
	}
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
        let tag_string = self.tags.as_ref().map(|tags| tags.join(":"));

        match (self.status, tag_string) {
            (Some(status), None) => {
		write!(
		    f,
		    "{} {} {}",
		    level_indicator, status, self.title
		)
	    },
	    (None, Some(tag_string)) => {
		write!(
		    f,
		    "{} {} {}",
		    level_indicator, self.title, tag_string,
		)
	    }
	    (Some(status), Some(tag_string)) => {
		write!(
		    f,
		    "{} {} {} :{}:",
		    level_indicator, status, self.title, tag_string,
		)
	    }
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
	    level, title, status, tags,
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
