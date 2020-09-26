use crate::iter::*;
use std::cmp;

pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<A, B> Zip<A, B> {
    #[inline]
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Iter, B: Iter> Iter for Zip<A, B> {
    type Item = (A::Item, B::Item);

    #[inline]
    fn next(self) -> (Option<Self::Item>, Self) {
        let (a, an) = self.a.next();
        if a.is_none() {
            return (None, Self { a: an, b: self.b });
        }
        let (b, bn) = self.b.next();
        let n = Self { a: an, b: bn };
        if b.is_none() {
            return (None, n);
        }
        (Some((a.unwrap(), b.unwrap())), n)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_lower, a_upper) = self.a.size_hint();
        let (b_lower, b_upper) = self.b.size_hint();

        let lower = cmp::min(a_lower, b_lower);

        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => Some(cmp::min(x, y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        };

        (lower, upper)
    }
}
