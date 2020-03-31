use crate::{headline::HeadlineGroup, object::Object};

pub struct SubObjects<'t> {
    headline_group: &'t HeadlineGroup<'t>,
    index: usize,
}

impl<'t> IntoIterator for &'t HeadlineGroup<'t> {
    type Item = Object<'t>;
    type IntoIter = SubObjects<'t>;

    fn into_iter(self) -> Self::IntoIter {
        SubObjects {
            headline_group: self,
            index: 0,
        }
    }
}

impl<'t> Iterator for SubObjects<'t> {
    type Item = Object<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.headline_group
            .content
            .as_ref()
            // check for content[index]
            .and_then(|content| content.get(self.index))
            .map(Object::from)
            // or check for sub_headlines[(self.index - content.len())]
            .or_else(|| {
                let index = self.index - self.headline_group.content_len();
                self.headline_group
                    .sub_headlines
                    .as_ref()
                    .and_then(|sub_headlines| sub_headlines.get(index))
                    .map(Object::from)
            })
            // increment iterator if an object is found
            .map(|object| {
                self.index += 1;
                object
            });
	println!("{:?}", next);
	next
    }
}

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

pub struct AllObjects<'a> {
    stack: Vec<SubObjects<'a>>,
}
impl<'a> Iterator for AllObjects<'a> {
    type Item = Object<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        find_next_object(&mut self.stack)
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

trait Node {
    type Data;

    fn data(&self) -> Self::Data;
}

trait Tree {
    type Node: Node;
    type SubNodes: Iterator<Item = Self::Node>;
    type AllNodes: Iterator<Item = Self::Node>;

    fn sub_nodes(&self) -> Self::SubNodes;
    fn all_nodes(&self) -> Self::AllNodes;
}

// trait Recursive<'a> {
//     type Item: 'a;
//     type Iterator: Iterator<Item = &'a Self::Item>;
//     fn sub_items(&'a self) -> Self::Iterator;
// }

// impl<'t> Recursive<'t> for HeadlineGroup<'t> {
//     type Item = HeadlineGroup<'t>;
//     type Iterator = SubHeadlines<'t>;
//     fn sub_items(&'t self) -> Self::Iterator {
//         self.sub_headlines()
//     }
// }

fn find_next_object<'a>(stack: &mut Vec<SubObjects<'a>>) -> Option<Object<'a>> {
    while let Some(iterator) = stack.last_mut() {
        match iterator.next() {
            Some(object) => {
                if let Object::Headline(headline_group) = object {
                    stack.push(headline_group.sub_objects());
                }
                return Some(object);
            }
            None => {
                stack.pop();
            }
        }
    }
    None
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
