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

pub enum Object<'t> {
    Headline(Headline<'t>),
    Content(Content<'t>),
}

impl<'t> Document<'t> {
    pub fn headlines(&'t self) -> impl Iterator<Item = &'t HeadlineGroup<'t>> {
        self.root.all_headlines()
    }

    pub fn objects(&self) -> impl Iterator<Item = &'t Object<'t>> {
        // TODO
        std::iter::empty()
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
