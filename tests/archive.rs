#![cfg(feature = "rkyv")]

extern crate roaring;

use proptest::arbitrary::any;
use proptest::collection::btree_set;
use proptest::proptest;
use rkyv;
use rkyv::AlignedVec;
use roaring::bitmap::ArchivedRoaringBitmap;
use roaring::RoaringBitmap;
use std::iter::FromIterator;

fn bytes(bitmap: &RoaringBitmap) -> AlignedVec {
    rkyv::to_bytes::<_, 256>(bitmap).unwrap()
}

fn archive(bytes: &AlignedVec) -> &ArchivedRoaringBitmap {
    unsafe { rkyv::archived_root::<RoaringBitmap>(&bytes) }
}

#[test]
fn array() {
    let original = (0..2000).collect::<RoaringBitmap>();
    let bytes = bytes(&original);
    let archived = archive(&bytes);
    let clone = RoaringBitmap::from_iter(archived);

    assert_eq!(clone, original);
}

#[test]
fn bitmap() {
    let original = (0..100_000).collect::<RoaringBitmap>();
    let bytes = bytes(&original);
    let archived = archive(&bytes);
    let clone = RoaringBitmap::from_iter(archived);

    assert_eq!(clone, original);
}

#[test]
fn arrays() {
    let original = (0..2000)
        .chain(1_000_000..1_002_000)
        .chain(2_000_000..2_001_000)
        .collect::<RoaringBitmap>();
    let bytes = bytes(&original);
    let archived = archive(&bytes);
    let clone = RoaringBitmap::from_iter(archived);

    assert_eq!(clone, original);
}

#[test]
fn bitmaps() {
    let original = (0..100_000)
        .chain(1_000_000..1_012_000)
        .chain(2_000_000..2_010_000)
        .collect::<RoaringBitmap>();
    let bytes = bytes(&original);
    let archived = archive(&bytes);
    let clone = RoaringBitmap::from_iter(archived);

    assert_eq!(clone, original);
}

proptest! {
    #[test]
    fn iter(values in btree_set(any::<u32>(), ..=10_000)) {
        let original = RoaringBitmap::from_sorted_iter(values.iter().cloned()).unwrap();
        let bytes = bytes(&original);
        let archived = archive(&bytes);
        // Iterator::eq != PartialEq::eq - cannot use assert_eq macro
        assert!(values.into_iter().eq(archived.iter()));
    }
}
