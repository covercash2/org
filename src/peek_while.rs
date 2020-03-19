use std::iter::Peekable;

pub struct PeekWhile<'a, I, F>
where
    I: 'a + Iterator,
{
    iterator: &'a mut Peekable<I>,
    predicate: F,
}

impl<'a, I, F> Iterator for PeekWhile<'a, I, F>
where
    I: Iterator,
    I::Item: std::fmt::Debug,
    F: FnMut(&<I as Iterator>::Item) -> bool,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let predicate = &mut self.predicate;
        let resume = self.iterator.peek().map(|x| predicate(x)).unwrap_or(true);

        if resume {
            self.iterator.next()
        } else {
            None
        }
    }
}

pub trait PeekableExt<'a, I>: Iterator
where
    I: 'a + Iterator,
{
    fn peek_while<F>(&'a mut self, predicate: F) -> PeekWhile<'a, I, F>
    where
        Self: Sized,
        F: FnMut(&<Self as Iterator>::Item) -> bool;
}

impl<'a, I> PeekableExt<'a, I> for Peekable<I>
where
    I: 'a + Iterator,
{
    fn peek_while<F>(&'a mut self, predicate: F) -> PeekWhile<I, F>
    where
        F: FnMut(&<Self as Iterator>::Item) -> bool,
    {
        PeekWhile {
            iterator: self,
            predicate,
        }
    }
}

// TODO tests
