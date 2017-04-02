
use super::Plain;

use core::{mem, slice};
use core::cmp::min;

pub trait Methods: Plain {
    #[inline]
    unsafe fn from_raw_bytes<'a>(ptr: *const u8, bytes: usize) -> &'a Self;
    #[inline]
    unsafe fn from_raw_bytes_mut<'a>(ptr: *mut u8, bytes: usize) -> &'a mut Self;

    #[inline]
    fn reinterpret<T>(&self) -> &T
        where T: Methods + ?Sized
    {
        self::reinterpret(self)
    }

    #[inline]
    fn reinterpret_with_len<T>(&self, max_items: usize) -> &[T]
        where T: Methods
    {
        self::reinterpret_with_len(self, max_items)
    }

    #[inline]
    fn reintepret_mut<T>(&mut self) -> &mut T
        where T: Methods + ?Sized
    {
        self::reinterpret_mut(self)
    }

    #[inline]
    fn reintepret_mut_with_len<T>(&mut self, max_items: usize) -> &mut [T]
        where T: Methods
    {
        self::reinterpret_mut_with_len(self, max_items)
    }

    // Just a bunch of shorter names for the most common cases.

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self::as_bytes(self)
    }

    #[inline]
    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self::as_mut_bytes(self)
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> &Self {
        self::reinterpret(bytes)
    }

    #[inline]
    fn from_mut_bytes(bytes: &mut [u8]) -> &mut Self {
        self::reinterpret_mut(bytes)
    }
}

impl<S> Methods for S
    where S: Plain + Sized + Copy
{
    #[inline]
    unsafe fn from_raw_bytes<'a>(ptr: *const u8, bytes: usize) -> &'a S {
        assert!(mem::size_of::<S>() <= bytes,
                "plain::reinterpret(): input is too short for target type");

        let align_offset = (ptr as usize) % mem::align_of::<S>();
        assert_eq!(align_offset, 0, "plain::reinterpret(): badly aligned input");

        &*(ptr as *const S)
    }

    #[inline]
    unsafe fn from_raw_bytes_mut<'a>(ptr: *mut u8, bytes: usize) -> &'a mut S {
        assert!(mem::size_of::<S>() <= bytes,
                "plain::reinterpret(): input is too short for target type");

        let align_offset = (ptr as usize) % mem::align_of::<S>();
        assert_eq!(align_offset, 0, "plain::reinterpret(): badly aligned input");

        &mut *(ptr as *mut S)
    }
}

impl<S> Methods for [S]
    where S: Plain + Sized + Copy
{
    #[inline]
    unsafe fn from_raw_bytes<'a>(ptr: *const u8, bytes: usize) -> &'a [S] {
        let align_offset = (ptr as usize) % mem::align_of::<S>();
        assert_eq!(align_offset, 0, "plain::reinterpret(): badly aligned input");

        let len = bytes / mem::size_of::<S>();
        slice::from_raw_parts(ptr as *const S, len)
    }

    #[inline]
    unsafe fn from_raw_bytes_mut<'a>(ptr: *mut u8, bytes: usize) -> &'a mut [S] {
        let align_offset = (ptr as usize) % mem::align_of::<S>();
        assert_eq!(align_offset, 0, "plain::reinterpret(): badly aligned input");

        let len = bytes / mem::size_of::<S>();
        slice::from_raw_parts_mut(ptr as *mut S, len)
    }
}

#[inline]
pub fn reinterpret<T, S>(s: &S) -> &T
    where S: ?Sized,
          T: Methods + ?Sized
{

    // Even though slices can't under normal circumstances be cast
    // to pointers, here in generic code it works.
    // This means that `s` can be a slice or a regular reference,
    // at the caller's discretion.
    let bptr = s as *const S as *const u8;
    let bsize = mem::size_of_val(s);
    unsafe { T::from_raw_bytes(bptr, bsize) }
}

#[inline]
pub fn reinterpret_mut<T, S>(s: &mut S) -> &mut T
    where S: Plain + ?Sized,
          T: Methods + ?Sized
{

    let bptr = s as *mut S as *mut u8;
    let bsize = mem::size_of_val(s);
    unsafe { T::from_raw_bytes_mut(bptr, bsize) }
}

#[inline]
pub fn reinterpret_with_len<T, S>(s: &S, max_items: usize) -> &[T]
    where S: ?Sized,
          T: Methods
{

    let ptr = s as *const S;
    let bsize = mem::size_of_val(s);
    let align_offset = (ptr as *const u8 as usize) % mem::align_of::<T>();

    assert_eq!(align_offset, 0, "plain::reinterpret(): badly aligned input");

    let result_len = min(bsize / mem::size_of::<T>(), max_items);

    unsafe { slice::from_raw_parts(ptr as *const T, result_len) }
}

#[inline]
pub fn reinterpret_mut_with_len<T, S>(s: &mut S, max_items: usize) -> &mut [T]
    where S: Plain + ?Sized,
          T: Methods
{

    let ptr = s as *mut S;
    let bsize = mem::size_of_val(s);
    let align_offset = (ptr as *const u8 as usize) % mem::align_of::<T>();

    assert_eq!(align_offset, 0, "plain::reinterpret(): badly aligned input");

    let result_len = min(bsize / mem::size_of::<T>(), max_items);

    unsafe { slice::from_raw_parts_mut(ptr as *mut T, result_len) }
}

#[inline]
pub fn as_bytes<S>(s: &S) -> &[u8]
    where S: ?Sized
{

    reinterpret(s)
}

#[inline]
pub fn as_mut_bytes<S>(s: &mut S) -> &mut [u8]
    where S: Plain + ?Sized
{

    reinterpret_mut(s)
}


#[cfg(test)]
mod tests {

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
    #[should_panic]
    fn one_too_short() {
        let b = vec![0u8; mem::size_of::<Dummy1>()-1];

        // Assert fails inside.
        Dummy1::from_bytes(&b);
    }

    #[test]
    fn well_aligned() {
        let b = vec![0u8; mem::size_of::<Dummy1>()+1];

        // No failure.
        Dummy1::from_bytes(&b);
    }

    #[test]
    #[should_panic]
    fn unaligned() {
        let b = vec![0u8; mem::size_of::<Dummy1>()+1];
        let b = &b[1..];

        // Assert fails inside.
        Dummy1::from_bytes(&b);
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

        let r1: &Dummy2 = t1.reinterpret();

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

        let r4 = Dummy1::from_bytes(r3);

        let r5 = reinterpret::<Dummy2, _>(r4);

        let r6 = reinterpret::<[Dummy1], _>(r5);

        assert!(t1 == r6[0])
    }
}
