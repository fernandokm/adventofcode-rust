use std::{
    borrow::Borrow,
    ops::{Bound, RangeBounds},
};

use num_traits::{Bounded, CheckedAdd, CheckedSub, One, Zero};

use super::{coords::P2, signed::Signed};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct GridSpec<T> {
    pub range_0: (Bound<T>, Bound<T>),
    pub range_1: (Bound<T>, Bound<T>),
}

impl<T> GridSpec<T> {
    pub fn new(range_0: (Bound<T>, Bound<T>), range_1: (Bound<T>, Bound<T>)) -> Self {
        Self { range_0, range_1 }
    }

    pub fn new_indexed(rows: T, cols: T) -> Self
    where
        T: Zero,
    {
        Self::new(
            (Bound::Included(T::zero()), Bound::Excluded(rows)),
            (Bound::Included(T::zero()), Bound::Excluded(cols)),
        )
    }
}

impl<T> GridSpec<T>
where
    T: Clone + Zero + One + CheckedAdd + CheckedSub + PartialOrd,
{
    #[must_use]
    pub fn step(&self, point: &P2<T>, step_by: &P2<Signed<T>>) -> Option<P2<T>> {
        let result = point.checked_add_signed(step_by)?;

        if self.range_0.contains(&result.0) && self.range_1.contains(&result.1) {
            Some(result)
        } else {
            None
        }
    }

    #[must_use]
    pub fn directions_arr() -> [P2<Signed<T>>; 4] {
        [
            P2(Signed::one(), Signed::zero()),
            P2(Signed::zero(), Signed::one()),
            P2(-Signed::one(), Signed::zero()),
            P2(Signed::zero(), -Signed::one()),
        ]
    }

    pub fn directions() -> impl Iterator<Item = P2<Signed<T>>> {
        Self::directions_arr().into_iter()
    }

    pub fn neighbors<'a>(&'a self, point: &'a P2<T>) -> impl 'a + Iterator<Item = P2<T>> {
        Self::directions().filter_map(|offset| self.step(point, &offset))
    }

    pub fn step_to_end<'a>(
        &'a self,
        point: P2<T>,
        step_by: impl 'a + Borrow<P2<Signed<T>>>,
    ) -> impl 'a + Iterator<Item = P2<T>> {
        let mut next_point = Some(point);
        std::iter::from_fn(move || {
            let result = next_point.take()?;
            next_point = self.step(&result, step_by.borrow());
            Some(result)
        })
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = P2<T>>
    where
        T: Bounded,
    {
        let start_0 = first_value(&self.range_0);
        let start_1 = first_value(&self.range_1);
        let should_iter = start_0.is_some() && start_1.is_some();

        let start_0 = start_0.unwrap_or_else(T::zero);
        let start_1 = start_1.unwrap_or_else(T::zero);

        let step_by_0 = P2(Signed::one(), Signed::zero());
        let step_by_1 = P2(Signed::zero(), Signed::one());

        self.step_to_end(P2(start_0, start_1), step_by_0)
            .flat_map(move |p| self.step_to_end(p, step_by_1.clone()))
            .take_while(move |_| should_iter)
    }
}

impl<T, B> From<B> for GridSpec<T>
where
    T: Clone,
    B: RangeBounds<P2<T>>,
{
    fn from(range: B) -> Self {
        let map_bound = |bound, f: fn(&P2<T>) -> T| match bound {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(x) => Bound::Included(f(x)),
            Bound::Excluded(x) => Bound::Excluded(f(x)),
        };

        let low_0 = map_bound(range.start_bound(), |P2(x, _)| x.clone());
        let low_1 = map_bound(range.start_bound(), |P2(_, y)| y.clone());
        let high_0 = map_bound(range.end_bound(), |P2(x, _)| x.clone());
        let high_1 = map_bound(range.end_bound(), |P2(_, y)| y.clone());
        Self {
            range_0: (low_0, high_0),
            range_1: (low_1, high_1),
        }
    }
}

fn first_value<T>(range: &impl RangeBounds<T>) -> Option<T>
where
    T: Clone + Bounded + CheckedAdd + One,
{
    match range.start_bound() {
        Bound::Included(x) => Some(x.clone()),
        Bound::Excluded(x) => x.checked_add(&T::one()),
        Bound::Unbounded => Some(T::min_value()),
    }
}
