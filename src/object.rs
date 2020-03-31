use std::{fmt, fmt::Display};

use crate::{
    content::Content,
    headline::{Headline, HeadlineGroup},
};

#[derive(Debug)]
pub struct Document<'t> {
    pub text: &'t str,
    pub root: HeadlineGroup<'t>,
}

#[derive(Debug)]
pub enum Object<'t> {
    Headline(&'t HeadlineGroup<'t>),
    Content(&'t Content<'t>),
}

impl<'t> Document<'t> {
    pub fn headlines(&'t self) -> impl Iterator<Item = &'t HeadlineGroup<'t>> {
        self.root.all_headlines()
    }

    pub fn objects(&'t self) -> impl Iterator<Item = Object<'t>> {
        self.root.all_objects()
    }
}

impl<'t> From<&'t HeadlineGroup<'t>> for Object<'t> {
    fn from(headline: &'t HeadlineGroup<'t>) -> Self {
        Object::Headline(headline)
    }
}

impl<'t> From<&'t Content<'t>> for Object<'t> {
    fn from(content: &'t Content<'t>) -> Self {
        Object::Content(content)
    }
}

impl<'t> Display for Document<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for headline in self.root.sub_headlines() {
            writeln!(f, "{}", headline)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
