use std::{fmt, fmt::Display, iter};

use crate::headline::HeadlineGroup;

#[derive(Debug)]
pub struct Document<'t> {
    pub text: &'t str,
    pub root: HeadlineGroup<'t>,
}

impl<'t> Document<'t> {
    pub fn headlines(&'t self) -> impl Iterator<Item = &'t HeadlineGroup<'t>> {
        self.root.all_headlines()
    }
}

impl<'t> Display for Document<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
