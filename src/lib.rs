#![doc = include_str!("../README.md")]
#![no_std]

use core::cmp::Ordering;

#[doc = include_str!("../README.md")]
pub trait IterFirstMaxExt: Iterator + Sized {
    /// Get max element, if equal, return the first equal element
    #[inline]
    fn first_max(self) -> Option<Self::Item>
    where Self::Item: Ord,
    {
        self.first_max_by(Ord::cmp)
    }

    /// Get max element, if equal, return the first equal element
    #[inline]
    fn first_max_by<F>(self, mut f: F) -> Option<Self::Item>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.reduce(move |a, b| {
            if f(&a, &b).is_lt() {
                b
            } else {
                a
            }
        })
    }

    /// Get max element, if key equal, return the first key equal element
    #[inline]
    fn first_max_by_key<B, F>(self, mut f: F) -> Option<Self::Item>
    where F: FnMut(&Self::Item) -> B,
          B: Ord,
    {
        #[inline]
        fn cmp<B: Ord, T>((a, _): &(B, T), (b, _): &(B, T)) -> Ordering {
            a.cmp(b)
        }
        self.map(move |elem| (f(&elem), elem))
            .first_max_by(cmp)?
            .1
            .into()
    }
}
impl<I: Iterator> IterFirstMaxExt for I { }


/// Similar the [`IterFirstMaxExt`]
///
///
/// ```rust
/// use iter_first_max::IterLastMinExt as _;
///
/// let arr = [(1, 0), (1, 1), (1, 2), (2, 3), (2, 4)];
/// let first = arr.iter().min_by_key(|n| n.0);
/// let last  = arr.iter().last_min_by_key(|n| n.0);
///
/// assert_eq!(first, Some(&(1, 0)));
/// assert_eq!(last,  Some(&(1, 2)));
/// ```
pub trait IterLastMinExt: Iterator + Sized {
    /// Get min element, if equal, return the last equal element
    #[inline]
    fn last_min(self) -> Option<Self::Item>
    where Self::Item: Ord,
    {
        self.last_min_by(Ord::cmp)
    }

    /// Get min element, if equal, return the last equal element
    #[inline]
    fn last_min_by<F>(self, mut f: F) -> Option<Self::Item>
    where F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.reduce(move |a, b| {
            if f(&a, &b).is_ge() {
                b
            } else {
                a
            }
        })
    }

    /// Get min element, if key equal, return the last key equal element
    #[inline]
    fn last_min_by_key<B, F>(self, mut f: F) -> Option<Self::Item>
    where F: FnMut(&Self::Item) -> B,
          B: Ord,
    {
        #[inline]
        fn cmp<B: Ord, T>((a, _): &(B, T), (b, _): &(B, T)) -> Ordering {
            a.cmp(b)
        }
        self.map(move |elem| (f(&elem), elem))
            .last_min_by(cmp)?
            .1
            .into()
    }
}
impl<I: Iterator> IterLastMinExt for I { }
