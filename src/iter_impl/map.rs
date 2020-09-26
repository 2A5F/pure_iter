use crate::iter::Iter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct Map<I, F> {
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<I: Iter, F, U> Iter for Map<I, F>
where
    F: FnMut(I::Item) -> U,
{
    type Item = U;

    #[inline]
    fn next(mut self) -> (Option<Self::Item>, Self) {
        match self.iter.next() {
            (Some(v), n) => {
                let v = (self.f)(v);
                (Some(v), Map { iter: n, f: self.f })
            }
            (None, n) => (None, Map { iter: n, f: self.f }),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
