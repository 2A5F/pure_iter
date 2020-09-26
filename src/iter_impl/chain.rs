use crate::iter::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct Chain<A, B> {
    a: Option<A>,
    b: B,
}
impl<A, B> Chain<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a: Some(a), b }
    }
}

impl<A: Iter, B: Iter<Item = A::Item>> Iter for Chain<A, B> {
    type Item = A::Item;

    #[inline]
    fn next(self) -> (Option<Self::Item>, Self) {
        if let Some(iter) = self.a {
            match iter.next() {
                (None, _) => {}
                (v, n) => {
                    let b = self.b;
                    return (v, Self { a: Some(n), b });
                }
            }
        }
        let (v, b) = self.b.next();
        (v, Self { a: None, b })
    }

    #[inline]
    fn count(self) -> usize {
        let a_count = match self.a {
            Some(a) => a.count(),
            None => 0,
        };
        a_count + self.b.count()
    }
}
