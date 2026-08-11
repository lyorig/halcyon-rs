use std::sync::atomic::{AtomicUsize, Ordering};

use halcyon::boxed::Box as SdlBox;

use rustest::test;

static DROPS: AtomicUsize = AtomicUsize::new(0);

struct DropCounter;

impl Drop for DropCounter {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn boxed_new_deref() {
    let b = SdlBox::new(42);
    assert_eq!(*b, 42);

    let mut b = SdlBox::new(1);
    *b += 1;
    assert_eq!(*b, 2);
}

#[test]
fn boxed_new_zst() {
    let b = SdlBox::new(());
    assert_eq!(*b, ());
}

#[test]
fn boxed_into_raw_roundtrip() {
    let b = SdlBox::new(5);
    let raw = SdlBox::into_raw(b);
    let b = unsafe { SdlBox::from_raw(raw) };
    assert_eq!(*b, 5);
}

#[test]
fn boxed_leak() {
    let b = SdlBox::new(7);
    let leaked: &'static mut i32 = b.leak();
    assert_eq!(*leaked, 7);
    *leaked = 8;
    assert_eq!(*leaked, 8);
}

#[test]
fn boxed_as_mut_ptr() {
    let mut b = SdlBox::new(9);
    unsafe {
        *b.as_mut_ptr() = 10;
    }
    assert_eq!(*b, 10);
    assert!(!b.as_ptr().is_null());
}

#[test]
fn boxed_slice() {
    let b = SdlBox::from_slice(&[1, 2, 3]);
    assert_eq!(&*b, &[1, 2, 3]);
    assert_eq!(b.len(), 3);
    assert_eq!(b.iter().sum::<i32>(), 6);

    let b = SdlBox::from_iter(0..5);
    assert_eq!(b.len(), 5);
    assert_eq!(b[3], 3);
}

#[test]
fn boxed_slice_into_iter() {
    let b = SdlBox::from_slice(&[4, 5, 6]);
    assert_eq!(b.into_iter().sum::<i32>(), 15);
}

#[test]
fn boxed_slice_into_iter_drops_remaining() {
    DROPS.store(0, Ordering::Relaxed);

    let b = SdlBox::from_iter((0..5).map(|_| DropCounter));
    let mut iter = b.into_iter();

    assert!(iter.next().is_some()); // drops the yielded element
    drop(iter); // drops the four unyielded ones

    assert_eq!(DROPS.load(Ordering::Relaxed), 5);
}

#[test]
fn boxed_array_into_slice() {
    let b: SdlBox<[i32]> = SdlBox::new([1, 2, 3]).into();
    assert_eq!(&*b, &[1, 2, 3]);
}

#[test]
fn boxed_traits() {
    assert_eq!(SdlBox::new(3), SdlBox::new(3));
    assert!(SdlBox::new(1) < SdlBox::new(2));
    assert_eq!(SdlBox::new(5).clone(), SdlBox::new(5));
    assert_eq!(format!("{}", SdlBox::new(5)), "5");
    assert_eq!(format!("{:?}", SdlBox::new(5)), "5");

    let a = SdlBox::from_slice(&[1, 2]);
    assert_eq!(a.clone(), a);
    assert_eq!(SdlBox::<[i32]>::default().len(), 0);
}
