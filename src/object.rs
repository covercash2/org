use std::{fmt, fmt::Display, iter};

use crate::headline::{Headline, HeadlineGroup};

#[derive(Debug)]
pub struct Document<'t> {
    pub text: &'t str,
    pub root: HeadlineGroup<'t>,
}

impl<'t> Document<'t> {
    pub fn headlines(&'t self) -> impl Iterator<Item = &'t HeadlineGroup<'t>> {
        self.root.all_sub_headlines()
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

        let sub_headlines: Vec<&HeadlineGroup<'_>> = content.root.sub_headlines().collect();

        assert!(
            sub_headlines.len() == 5,
            "wrong number of sub-headlines. expected: 5, found {}",
            sub_headlines.len()
        );

        let headlines: Vec<&HeadlineGroup<'_>> = content.headlines().collect();

        headlines.iter().for_each(|headline| {
            println!("{}", headline);
        });

        assert!(
            headlines.len() == 12,
            "wrong number of headlines. expected: 12, found {}",
            headlines.len()
        );
    }
}
