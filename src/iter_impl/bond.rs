use crate::iter::*;

pub struct Bond<I: Iter>(pub(crate) I, pub(crate) I::Item);

impl<I: Iter> Iter for Bond<I>
where
    I::Item: Clone,
{
    /// (Last, Now)
    type Item = (I::Item, I::Item);

    fn next(self) -> (Option<Self::Item>, Self) {
        match self.0.next() {
            (Some(v), n) => (Some((self.1, v.clone())), Self(n, v)),
            (None, n) => (None, Self(n, self.1)),
        }
    }
}
