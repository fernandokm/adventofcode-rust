#[allow(clippy::module_name_repetitions)]
pub trait IterExt {
    type Item;

    fn take_until_inclusive<P>(self, predicate: P) -> TakeUntilInclusive<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeUntilInclusive::new(self, predicate)
    }
}

impl<I: Iterator> IterExt for I {
    type Item = I::Item;
}

pub struct TakeUntilInclusive<I, P> {
    iter: I,
    predicate: P,
    stopped: bool,
}

impl<I, P> TakeUntilInclusive<I, P> {
    pub fn new(iter: I, predicate: P) -> Self {
        Self {
            iter,
            predicate,
            stopped: false,
        }
    }
}

impl<I, P> Iterator for TakeUntilInclusive<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped {
            return None;
        }
        let val = self.iter.next()?;
        if (self.predicate)(&val) {
            self.stopped = true;
        }
        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}
