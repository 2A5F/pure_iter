use crate::iter::*;

impl<T> FromIter<T> for Vec<T> {
    #[inline]
    fn from_iter<I>(mut iter: I) -> Self
    where
        I: Iter<Item = T>,
    {
        let mut vec: Vec<T> = match iter.size_hint() {
            (_, Some(max)) => Vec::with_capacity(max),
            (0, None) => Vec::new(),
            (min, None) => Vec::with_capacity(min),
        };
        loop {
            match iter.next() {
                (Some(v), n) => {
                    vec.push(v);
                    iter = n;
                }
                (None, _) => {
                    return vec;
                }
            }
        }
    }
}
