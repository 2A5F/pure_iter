pub use crate::iter_impl::*;
use std::ops::Add;

pub trait Iter: Sized {
    type Item;

    fn next(self) -> (Option<Self::Item>, Self);

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    #[inline]
    fn count(self) -> usize {
        #[inline]
        fn add<T>(count: usize, _: T) -> usize {
            Add::add(count, 1)
        }
        self.fold(0, add)
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }
        self.fold(None, some)
    }

    #[inline]
    fn nth(mut self, mut c: usize) -> (Option<Self::Item>, Self) {
        loop {
            match self.next() {
                (Some(v), n) => {
                    if c == 0 {
                        return (Some(v), n);
                    }
                    c -= 1;
                    self = n;
                }
                l => return l,
            }
        }
    }

    #[inline]
    fn step_by(self, step: usize) -> StepBy<Self> {
        StepBy::new(self, step)
    }

    #[inline]
    fn chain<U: IntoIter<Item = Self::Item>>(self, other: U) -> Chain<Self, U::IntoIter> {
        Chain::new(self, other.into_pure_iter())
    }

    #[inline]
    fn zip<U: IntoIter<Item = Self::Item>>(self, other: U) -> Zip<Self, U::IntoIter> {
        Zip::new(self, other.into_pure_iter())
    }

    #[inline]
    fn map<F, U>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> U,
    {
        Map { iter: self, f }
    }

    #[inline]
    fn for_each<F>(mut self, mut f: F)
    where
        F: FnMut(Self::Item),
    {
        while let (Some(v), n) = self.next() {
            f(v);
            self = n;
        }
    }

    #[inline]
    fn collect<U: FromIter<Self::Item>>(self) -> U {
        FromIter::from_iter(self)
    }

    #[inline]
    fn fold<F, U>(mut self, init: U, mut f: F) -> U
    where
        F: FnMut(U, Self::Item) -> U,
    {
        let mut accum = init;
        while let (Some(v), n) = self.next() {
            accum = f(accum, v);
            self = n;
        }
        accum
    }

    #[inline]
    fn iter(self) -> IterIterator<Self> {
        IterIterator::new(self)
    }
}

pub trait GetIter {
    type Iter: Iter;

    fn pure_iter(self) -> Self::Iter;
}

pub trait IntoIter {
    type Item;
    type IntoIter: Iter<Item = Self::Item>;

    fn into_pure_iter(self) -> Self::IntoIter;
}

impl<I: Iter> IntoIter for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_pure_iter(self) -> Self::IntoIter {
        self
    }
}

pub trait FromIter<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: Iter<Item = T>;
}

pub struct IterIterator<I> {
    iter: Option<I>,
}

impl<I> IterIterator<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter: Some(iter) }
    }
}

impl<I: Iter> Iterator for IterIterator<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (v, n) = self.iter.take().unwrap().next();
        self.iter = Some(n);
        v
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.as_ref().unwrap().size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.unwrap().count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.iter.unwrap().last()
    }

    #[inline]
    fn nth(&mut self, c: usize) -> Option<Self::Item> {
        let (v, n) = self.iter.take().unwrap().nth(c);
        self.iter = Some(n);
        v
    }

    #[inline]
    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        self.iter.unwrap().for_each(f)
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.unwrap().fold(init, f)
    }
}
