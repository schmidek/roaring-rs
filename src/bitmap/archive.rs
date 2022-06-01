use crate::bitmap::container::ArchivedContainer;
use crate::bitmap::store::ArchivedStore;
use crate::bitmap::ArchivedRoaringBitmap;
use std::{iter, slice};

/// An iterator for `ArchivedRoaringBitmap`.
pub struct Iter<'a> {
    inner: iter::Flatten<slice::Iter<'a, ArchivedContainer>>,
    size_hint: u64,
}

impl Iter<'_> {
    fn new(containers: &[ArchivedContainer]) -> Iter {
        let size_hint = containers.iter().map(|c| c.len()).sum();
        Iter { inner: containers.iter().flatten(), size_hint }
    }
}

impl Iterator for Iter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.size_hint = self.size_hint.saturating_sub(1);
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size_hint < usize::MAX as u64 {
            (self.size_hint as usize, Some(self.size_hint as usize))
        } else {
            (usize::MAX, None)
        }
    }
}

impl ArchivedRoaringBitmap {
    pub fn iter(&self) -> Iter {
        Iter::new(&self.containers)
    }

    pub fn len(&self) -> u64 {
        self.containers.iter().map(|container| container.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.containers.is_empty()
    }
}

impl<'a> IntoIterator for &'a ArchivedRoaringBitmap {
    type Item = u32;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl ArchivedContainer {
    pub fn len(&self) -> u64 {
        self.store.len()
    }
}

impl<'a> IntoIterator for &'a ArchivedStore {
    type Item = u16;
    type IntoIter = crate::bitmap::store::Iter<'a>;
    fn into_iter(self) -> crate::bitmap::store::Iter<'a> {
        match self {
            ArchivedStore::Array(vec) => crate::bitmap::store::Iter::Array(vec.iter()),
            ArchivedStore::Bitmap(bits) => crate::bitmap::store::Iter::BitmapBorrowed(bits.iter()),
        }
    }
}

impl ArchivedStore {
    pub fn len(&self) -> u64 {
        match self {
            ArchivedStore::Array(vec) => vec.len(),
            ArchivedStore::Bitmap(bits) => bits.len(),
        }
    }
}
