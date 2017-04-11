
use super::*;
use core::mem;

#[repr(C)]
#[derive(Debug, Copy, Eq, Clone, PartialEq)]
struct Dummy1 {
    field1: u64,
    field2: u32,
    field3: u16,
    field4: u8,
    field5: u8,
}

unsafe impl Plain for Dummy1 {}

#[repr(C)]
#[derive(Debug, Copy, Eq, Clone, PartialEq)]
struct Dummy2 {
    field1: u8,
    field2: u8,
    field3: u16,
    field4: u32,
    field5: u64,
}

unsafe impl Plain for Dummy2 {}

#[test]
fn one_too_short() {
    let b = vec![0u8; mem::size_of::<Dummy1>()-1];

    let r = Dummy1::from_bytes(&b);
    assert!(r == Err(Error::TooShort));
}

#[test]
fn well_aligned() {
    let b = vec![0u8; mem::size_of::<Dummy1>()+1];

    // No failure.
    Dummy1::from_bytes(&b).unwrap();
}

#[test]
fn unaligned() {
    let b = vec![0u8; mem::size_of::<Dummy1>()+1];
    let b = &b[1..];

    let r = Dummy1::from_bytes(&b);
    assert!(r == Err(Error::BadAlignment));
}

#[test]
fn basic_function() {
    let t1 = Dummy1 {
        field1: 0xaaaaaaaaaaaaaaaau64,
        field2: 0xbbbbbbbbu32,
        field3: 0xccccu16,
        field4: 0xddu8,
        field5: 0xeeu8,
    };

    let r1: &Dummy2 = from_bytes(t1.as_bytes()).unwrap();

    assert!(r1.field1 == 0xaau8);
    assert!(r1.field2 == 0xaau8);
    assert!(r1.field3 == 0xaaaau16);
    assert!(r1.field4 == 0xaaaaaaaau32);
    assert!(r1.field5 == 0xbbbbbbbbccccddeeu64 || r1.field5 == 0xeeddccccbbbbbbbbu64);

    let r2 = r1.as_bytes();
    assert!(r2.len() == mem::size_of::<Dummy1>());
    assert!(r2[5] == 0xaa);

    // nop
    let r3 = r2.as_bytes();

    let r4 = Dummy1::from_bytes(r3).unwrap();

    let r5 = from_bytes::<Dummy2>(r4.as_bytes()).unwrap();

    let r6 = slice_from_bytes::<Dummy1>(r5.as_bytes()).unwrap();

    assert!(r6.len() == 1);
    assert!(t1 == r6[0]);
}
