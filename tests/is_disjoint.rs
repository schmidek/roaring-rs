extern crate roaring;

use roaring::bitmap::lazy::LazyRoaringBitmap;
use roaring::RoaringBitmap;

#[test]
fn array() {
    let bitmap1 = (0..2000).collect::<RoaringBitmap>();
    let bitmap2 = (4000..6000).collect::<RoaringBitmap>();
    assert!(bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn array_not() {
    let bitmap1 = (0..4000).collect::<RoaringBitmap>();
    let bitmap2 = (2000..6000).collect::<RoaringBitmap>();
    assert!(!bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn bitmap() {
    let bitmap1 = (0..6000).collect::<RoaringBitmap>();
    let bitmap2 = (10000..16000).collect::<RoaringBitmap>();
    assert!(bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn lazy_bitmap() {
    let bitmap1 = (0..6000).collect::<RoaringBitmap>();
    let bitmap2 = (10000..16000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}

#[test]
fn bitmap_not() {
    let bitmap1 = (0..10000).collect::<RoaringBitmap>();
    let bitmap2 = (5000..15000).collect::<RoaringBitmap>();
    assert!(!bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn lazy_bitmap_not() {
    let bitmap1 = (0..10000).collect::<RoaringBitmap>();
    let bitmap2 = (5000..15000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(!bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}

#[test]
fn arrays() {
    let bitmap1 = (0..2000)
        .chain(1_000_000..1_002_000)
        .chain(2_000_000..2_002_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..102_000).chain(1_100_000..1_102_000).collect::<RoaringBitmap>();
    assert!(bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn lazy_arrays() {
    let bitmap1 = (0..2000)
        .chain(1_000_000..1_002_000)
        .chain(2_000_000..2_002_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..102_000).chain(1_100_000..1_102_000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}

#[test]
fn arrays_not() {
    let bitmap1 = (0..2_000)
        .chain(1_000_000..1_002_000)
        .chain(2_000_000..2_002_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..102_000).chain(1_001_000..1_003_000).collect::<RoaringBitmap>();
    assert!(!bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn lazy_arrays_not() {
    let bitmap1 = (0..2_000)
        .chain(1_000_000..1_002_000)
        .chain(2_000_000..2_002_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..102_000).chain(1_001_000..1_003_000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(!bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}

#[test]
fn bitmaps() {
    let bitmap1 = (0..6000)
        .chain(1_000_000..1_006_000)
        .chain(2_000_000..2_006_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..106_000).chain(1_100_000..1_106_000).collect::<RoaringBitmap>();
    assert!(bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn lazy_bitmaps() {
    let bitmap1 = (0..6000)
        .chain(1_000_000..1_006_000)
        .chain(2_000_000..2_006_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..106_000).chain(1_100_000..1_106_000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}

#[test]
fn bitmaps_not() {
    let bitmap1 = (0..6000)
        .chain(1_000_000..1_006_000)
        .chain(2_000_000..2_006_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..106_000).chain(1_004_000..1_008_000).collect::<RoaringBitmap>();
    assert!(!bitmap1.is_disjoint(&bitmap2));
}

#[test]
fn lazy_bitmaps_not() {
    let bitmap1 = (0..6000)
        .chain(1_000_000..1_006_000)
        .chain(2_000_000..2_006_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000..106_000).chain(1_004_000..1_008_000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(!bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}

#[test]
fn lazy_gaps_not() {
    let bitmap1 = (0..6000)
        .chain(1_000_000..1_006_000)
        .chain(2_000_000..2_006_000)
        .chain(100_000_000..100_006_000)
        .collect::<RoaringBitmap>();
    let bitmap2 = (100_000_000..100_006_000).collect::<RoaringBitmap>();
    let mut buffer = vec![];
    bitmap2.serialize_into(&mut buffer).unwrap();
    let bitmap2 = LazyRoaringBitmap::deserialize_from(&buffer[..]).unwrap();
    assert!(!bitmap1.lazy_is_disjoint(&bitmap2).unwrap());
}
