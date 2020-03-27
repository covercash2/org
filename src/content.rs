use std::{fmt, fmt::Display};

#[derive(Debug)]
pub enum Content<'t> {
    // TODO Drawer(drawer, content)
    List(Vec<ListItem<'t>>),
    Text(Vec<&'t str>),
}

impl<'t> From<Vec<ListItem<'t>>> for Content<'t> {
    fn from(items: Vec<ListItem<'t>>) -> Self {
        Content::List(items)
    }
}

impl<'t> Display for Content<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::List(items) => {
                for item in items {
                    write!(f, "{}", item)?;
                }
            }
            Content::Text(lines) => {
                for line in lines {
                    write!(f, "{}", line)?;
                }
            }
        }
        Ok(())
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
