use crate::headline::HeadlineGroup;

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

pub struct Headlines<'a> {
    stack: Vec<SubHeadlines<'a>>,
}
impl<'a> From<SubHeadlines<'a>> for Headlines<'a> {
    fn from(sub_headlines: SubHeadlines<'a>) -> Self {
        Headlines {
            stack: vec![sub_headlines],
        }
    }
}
impl<'a> Iterator for Headlines<'a> {
    type Item = &'a HeadlineGroup<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        find_next_headline(&mut self.stack)
    }
}

fn find_next_headline<'a>(stack: &mut Vec<SubHeadlines<'a>>) -> Option<&'a HeadlineGroup<'a>> {
    while let Some(iterator) = stack.last_mut() {
        match iterator.next() {
            Some(headline_group) => {
                // put sub header iterator on the stack
                stack.push(headline_group.sub_headlines());
                return Some(headline_group);
            }
            None => {
                // end of top iterator
                stack.pop();
            }
        }
    }
    None
}
