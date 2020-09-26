use crate::iter::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct SliceIter<'a, T>(&'a [T]);

impl<T> Copy for SliceIter<'_, T> {}
impl<T> Clone for SliceIter<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> SliceIter<'_, T> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0
    }
}

impl<'a, T> Iter for SliceIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(self) -> (Option<Self::Item>, Self) {
        if self.0.is_empty() {
            (None, self)
        } else {
            (Some(&self.0[0]), Self(&self.0[1..]))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.0.len();
        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.0.len()
    }

    #[inline]
    fn nth(self, c: usize) -> (Option<Self::Item>, Self) {
        if c >= self.0.len() {
            (None, Self(&self.0[self.0.len() - 1..]))
        } else {
            (Some(&self.0[c]), Self(&self.0[c + 1..]))
        }
    }
}

impl<T> AsRef<[T]> for SliceIter<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0
    }
}

impl<'a, T> GetIter for &'a [T] {
    type Iter = SliceIter<'a, T>;

    #[inline]
    fn pure_iter(self) -> Self::Iter {
        SliceIter(self)
    }
}

impl<'a, T> IntoIter for &'a [T] {
    type Item = &'a T;
    type IntoIter = SliceIter<'a, T>;

    #[inline]
    fn into_pure_iter(self) -> Self::IntoIter {
        self.pure_iter()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct SliceIterMut<'a, T>(&'a mut [T]);

impl<'a, T> SliceIterMut<'a, T> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        self.0
    }

    #[inline]
    pub fn into_slice(self) -> &'a mut [T] {
        self.0
    }
}

impl<'a, T> Iter for SliceIterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(self) -> (Option<Self::Item>, Self) {
        if self.0.is_empty() {
            (None, self)
        } else {
            let (first, last) = self.0.split_first_mut().unwrap();
            (Some(first), Self(last))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.0.len();
        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.0.len()
    }

    #[inline]
    fn nth(self, c: usize) -> (Option<Self::Item>, Self) {
        let len = self.0.len();
        if c >= len {
            (None, Self(&mut self.0[len - 1..]))
        } else {
            let (v, last) = self.0.split_at_mut(c);
            (Some(&mut v[c]), Self(last))
        }
    }
}

impl<T> AsRef<[T]> for SliceIterMut<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0
    }
}

impl<T> AsMut<[T]> for SliceIterMut<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0
    }
}

impl<'a, T> GetIter for &'a mut [T] {
    type Iter = SliceIterMut<'a, T>;

    #[inline]
    fn pure_iter(self) -> Self::Iter {
        SliceIterMut(self)
    }
}

impl<'a, T> IntoIter for &'a mut [T] {
    type Item = &'a mut T;
    type IntoIter = SliceIterMut<'a, T>;

    #[inline]
    fn into_pure_iter(self) -> Self::IntoIter {
        self.pure_iter()
    }
}
