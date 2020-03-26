use std::{fmt, fmt::Display, iter};

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

    pub fn headlines(&self) -> impl Iterator<Item = &'t Headline<'t>> {
        //HeadlineObject::from_object(&self.root).map(|headline_object| headline_object.headlines())
        iter::empty()
    }
}

#[derive(Debug)]
pub enum OrgObject<'t> {
    Headline(Headline<'t>, Vec<OrgObject<'t>>),
    List(Vec<ListItem<'t>>),
    Text(Vec<&'t str>),
}

#[derive(Debug)]
pub enum Content<'t> {
    // TODO Drawer(drawer, content)
    List(Vec<ListItem<'t>>),
    Text(&'t str),
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

    fn sub_headlines(&self) -> impl Iterator<Item = HeadlineObject<'t>> {
        let iterator = self
            .subobjects
            .iter()
            .filter_map(|subobject| HeadlineObject::from_object(subobject));

        HeadlineObjects::from(iterator)
    }
}

struct HeadlineObjects<'t, I>
where
    I: Iterator<Item = HeadlineObject<'t>>,
{
    iterator: I,
}

impl<'t, I> From<I> for HeadlineObjects<'t, I>
where
    I: Iterator<Item = HeadlineObject<'t>>,
{
    fn from(iterator: I) -> Self {
        HeadlineObjects { iterator }
    }
}

impl<'t, I> Iterator for HeadlineObjects<'t, I>
where
    I: Iterator<Item = HeadlineObject<'t>>,
{
    type Item = HeadlineObject<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

// struct Headlines<'t, I>
// where
//     I: Iterator<Item = HeadlineObject<'t>>,
// {
//     stack: Vec<I>,
//     s: Vec<dyn Iterator<Item = HeadlineObject<'t>>>,
// }

// impl<'t, I> From<I> for Headlines<'t, I>
// where
//     I: Iterator<Item = HeadlineObject<'t>>,
// {
//     fn from(iterator: I) -> Self {
//         let stack = vec![iterator];
//         Headlines { stack }
//     }
// }

// impl<'t, I> Iterator for Headlines<'t, I>
// where
//     I: Iterator<Item = HeadlineObject<'t>>,
// {
//     type Item = &'t Headline<'t>;

//     fn next(&mut self) -> Option<Self::Item> {
//         // get the iterator on top of the stack
//         let mut iterator = self.stack.last()?;

//         match iterator.next() {
//             Some(headline_object) => {
//                 let new_iterator: I = headline_object.sub_headlines();
//                 self.stack.push(new_iterator);
//                 Some(headline_object.headline)
//             }
//         }
//     }
// }

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

    #[test]
    fn headline_iterator() {
        let content = crate::parser::parse_org_text(count_up_headlines, test_states.to_vec())
            .expect("could not parse test string");

        let sub_headlines: Vec<HeadlineObject<'_>> = HeadlineObject::from_object(&content.root)
            .expect("could not parse root object as header")
            .sub_headlines()
            .collect();

        assert!(
            sub_headlines.len() == 5,
            "wrong number of sub-headlines. expected: 5, found {}",
            sub_headlines.len()
        );

        let headlines: Vec<&Headline<'_>> = content.headlines().collect();

        assert!(
            headlines.len() == 12,
            "wrong number of headlines. expected: 12, found {}",
            headlines.len()
        );
    }
}
