use crate::iter::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct StepBy<I> {
    iter: I,
    step: usize,
    first_take: bool,
}

impl<I> StepBy<I> {
    #[inline]
    pub(crate) fn new(iter: I, step: usize) -> Self {
        assert!(step != 0);
        Self {
            iter,
            step: step - 1,
            first_take: true,
        }
    }

    #[inline]
    pub(self) fn new_body(iter: I, step: usize) -> Self {
        Self {
            iter,
            step,
            first_take: false,
        }
    }
}

impl<I: Iter> Iter for StepBy<I> {
    type Item = I::Item;

    #[inline]
    fn next(self) -> (Option<Self::Item>, Self) {
        if self.first_take {
            let (v, n) = self.iter.next();
            (v, Self::new_body(n, self.step))
        } else {
            let (v, n) = self.iter.nth(self.step);
            (v, Self::new_body(n, self.step))
        }
    }
}
