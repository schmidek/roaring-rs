extern crate roaring;

use proptest::arbitrary::any;
use proptest::collection::btree_set;
use proptest::proptest;
use std::iter::FromIterator;

use roaring::RoaringBitmap;

#[test]
fn array() {
    let original = (0..2000).collect::<RoaringBitmap>();
    let clone = RoaringBitmap::from_iter(&original);
    let clone2 = RoaringBitmap::from_iter(original.clone());

    assert_eq!(clone, original);
    assert_eq!(clone2, original);
}

#[test]
fn bitmap() {
    let original = (0..6000).collect::<RoaringBitmap>();
    let clone = RoaringBitmap::from_iter(&original);
    let clone2 = RoaringBitmap::from_iter(original.clone());

    assert_eq!(clone, original);
    assert_eq!(clone2, original);
}

#[test]
fn arrays() {
    let original = (0..2000)
        .chain(1_000_000..1_002_000)
        .chain(2_000_000..2_001_000)
        .collect::<RoaringBitmap>();
    let clone = RoaringBitmap::from_iter(&original);
    let clone2 = RoaringBitmap::from_iter(original.clone());

    assert_eq!(clone, original);
    assert_eq!(clone2, original);
}

#[test]
fn bitmaps() {
    let original = (0..6000)
        .chain(1_000_000..1_012_000)
        .chain(2_000_000..2_010_000)
        .collect::<RoaringBitmap>();
    let clone = RoaringBitmap::from_iter(&original);
    let clone2 = RoaringBitmap::from_iter(original.clone());

    assert_eq!(clone, original);
    assert_eq!(clone2, original);
}

proptest! {
    #[test]
    fn iter(values in btree_set(any::<u32>(), ..=10_000)) {
        let bitmap = RoaringBitmap::from_sorted_iter(values.iter().cloned()).unwrap();
        // Iterator::eq != PartialEq::eq - cannot use assert_eq macro
        assert!(values.into_iter().eq(bitmap.into_iter()));
    }
}

#[test]
fn rev_iter() {
    use roaring::RoaringBitmap;

    let input = (1..3).chain(1_000_000..1_012_003).chain(2_000_001..2_000_003);
    let bitmap = input.clone().collect::<RoaringBitmap>();

    assert!(input.rev().eq(bitmap.into_iter().rev()));
}

#[derive(Clone, Debug)]
struct OutsideInIter<T>(bool, T);

impl<T, I> Iterator for OutsideInIter<I>
where
    I: DoubleEndedIterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.0 { self.1.next() } else { self.1.next_back() };
        self.0 = !self.0;
        res
    }
}

fn outside_in<U, I>(into_iter: I) -> OutsideInIter<U>
where
    U: DoubleEndedIterator,
    I: IntoIterator<IntoIter = U>,
{
    OutsideInIter(true, into_iter.into_iter())
}

// Sanity check that outside_in does what we expect
#[test]
fn outside_in_iterator() {
    let input = 0..10;
    assert!(outside_in(input).eq(vec![0, 9, 1, 8, 2, 7, 3, 6, 4, 5].into_iter()));
}

#[test]
fn interleaved_array() {
    use roaring::RoaringBitmap;

    let input = 0..100;
    let bitmap = input.clone().collect::<RoaringBitmap>();

    assert!(outside_in(input).eq(outside_in(bitmap)));
}

#[test]
fn interleaved_bitmap() {
    use roaring::RoaringBitmap;

    let input = 0..=4097;
    let bitmap = input.clone().collect::<RoaringBitmap>();

    assert!(outside_in(input).eq(outside_in(bitmap)));
}

proptest! {
    #[test]
    fn double_ended_iter(values in btree_set(any::<u32>(), 50_000..=100_000)) {
        let bitmap = RoaringBitmap::from_sorted_iter(values.iter().cloned()).unwrap();

        assert!(outside_in(values).eq(outside_in(bitmap)));
    }
}
