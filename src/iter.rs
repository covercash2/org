use crate::headline::{Headline, HeadlineGroup};

pub struct SubHeadlines<'t> {
    sub_headlines: &'t Option<Vec<HeadlineGroup<'t>>>,
    index: usize,
}
impl<'t> Iterator for SubHeadlines<'t> {
    type Item = &'t HeadlineGroup<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sub_headlines
            .as_ref()
            .and_then(|sub_headlines| sub_headlines.get(self.index))
            .map(|sub_headline| {
                self.index += 1;
                sub_headline
            })
    }
}
impl<'t> From<&'t Option<Vec<HeadlineGroup<'t>>>> for SubHeadlines<'t> {
    fn from(sub_headlines: &'t Option<Vec<HeadlineGroup>>) -> Self {
        SubHeadlines {
            sub_headlines,
            index: 0,
        }
    }
}

pub struct Headlines<'t> {
    stack: Vec<SubHeadlines<'t>>,
}
impl<'t> Headlines<'t> {
    pub fn new(headline: &'t HeadlineGroup<'t>) -> Headlines<'t> {
        Headlines {
            stack: vec![headline.sub_headlines()],
        }
    }
}
impl<'t> From<SubHeadlines<'t>> for Headlines<'t> {
    fn from(sub_headlines: SubHeadlines<'t>) -> Self {
        Headlines {
            stack: vec![sub_headlines],
        }
    }
}
impl<'t> Iterator for Headlines<'t> {
    type Item = &'t HeadlineGroup<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        let iterator = self.stack.last_mut()?;

        let next_headline = iterator.next()?;

        let next_iterator = next_headline.sub_headlines();

        self.stack.push(next_iterator);

        None
    }
}
