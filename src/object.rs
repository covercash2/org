use std::{fmt, fmt::Display};

use crate::{error, error::OrgError, headline::Headline};

#[derive(Debug)]
pub struct OrgContent<'t> {
    pub text: &'t str,
    pub root: OrgObject<'t>,
}

impl<'t> OrgContent<'t> {
    pub fn objects(&self) -> Option<&Vec<OrgObject<'t>>> {
        match &self.root {
            OrgObject::Headline(_headline, objects) => Some(&objects),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum OrgObject<'t> {
    Headline(Headline<'t>, Vec<OrgObject<'t>>),
    List(Vec<ListItem<'t>>),
    Text(Vec<&'t str>),
}

struct HeadlineObject<'t> {
    pub headline: &'t Headline<'t>,
    pub subobjects: &'t Vec<OrgObject<'t>>,
}

impl<'t> HeadlineObject<'t> {
    fn from_object(object: &'t OrgObject<'t>) -> Option<Self> {
        match object {
            OrgObject::Headline(headline, subobjects) => Some(HeadlineObject {
                headline,
                subobjects,
            }),
            _ => None,
        }
    }

    fn headlines(&self) -> impl Iterator<Item = HeadlineObject> {
        self.subobjects
            .iter()
            .filter_map(|subobject| HeadlineObject::from_object(subobject))
    }
}

impl<'t> OrgObject<'t> {
    pub fn is_headline(&self) -> bool {
        match self {
            OrgObject::Headline(_, _) => true,
            _ => false,
        }
    }

    pub fn subobjects(&self) -> Option<&Vec<OrgObject<'_>>> {
        HeadlineObject::from_object(self).map(|headline_object| headline_object.subobjects)
    }
}

impl<'t> From<Vec<ListItem<'t>>> for OrgObject<'t> {
    fn from(list_items: Vec<ListItem<'t>>) -> OrgObject<'t> {
        OrgObject::List(list_items)
    }
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
            OrgObject::Headline(headline, objects) => {
                write!(f, "{}", headline)?;

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
