
use super::Plain;

use core::{mem, slice};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    TooShort,
    BadAlignment,
}

/// This trait exposes all the functionality of this crate
/// as methods on applicable types.
///
/// It is implemented automatically for every type marked with
/// the [`Plain`](trait.Plain.html) trait, as well as their slices,
/// so you should never need to implement it yourself.
///
pub trait Methods: Plain {
    /// Same as [`from_bytes()`](fn.from_bytes.html), except attached to T.
    #[inline]
    fn from_bytes<'a>(bytes: &'a [u8]) -> Result<&'a Self, Error>;

    /// Same as [`from_mut_bytes()`](fn.from_mut_bytes.html), except attached to T.
    #[inline]
    fn from_mut_bytes<'a>(bytes: &'a mut [u8]) -> Result<&'a mut Self, Error>;

    /// Same as [`as_bytes()`](fn.as_bytes.html), except as a method of T.
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self::as_bytes(self)
    }

    /// Same as [`as_mut_bytes()`](fn.as_mut_bytes.html), except as a method of T.
    #[inline(always)]
    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self::as_mut_bytes(self)
    }
}

#[inline(always)]
fn check_instance_size<T>(bytes: &[u8]) -> Result<(), Error> {
    if bytes.len() < mem::size_of::<T>() {
        // slice is too short for target type
        Err(Error::TooShort)
    } else {
        Ok(())
    }
}

#[inline(always)]
fn check_alignment<T>(bytes: &[u8]) -> Result<(), Error> {
    let align_offset = (bytes.as_ptr() as usize) % mem::align_of::<T>();
    
    if align_offset != 0 {
        // badly aligned slice
        Err(Error::BadAlignment)
    } else {
        Ok(())
    }
}

impl<S> Methods for S
    where S: Plain + Sized
{
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<&S, Error> {
        try!(check_instance_size::<S>(bytes));
        try!(check_alignment::<S>(bytes));
        Ok(unsafe { &*(bytes.as_ptr() as *const S) })
    }

    #[inline]
    fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut S, Error> {
        try!(check_instance_size::<S>(bytes));
        try!(check_alignment::<S>(bytes));
        Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut S) })
    }
}

impl<S> Methods for [S]
    where S: Plain + Sized
{
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<&[S], Error> {
        try!(check_alignment::<S>(bytes));
        let len = bytes.len() / mem::size_of::<S>();
        Ok(unsafe { slice::from_raw_parts(bytes.as_ptr() as *const S, len) })
    }

    #[inline]
    fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut [S], Error> {
        try!(check_alignment::<S>(bytes));
        let len = bytes.len() / mem::size_of::<S>();
        Ok(unsafe { slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut S, len) })
    }
}

/// Safely converts a reference to any type to an immutable
/// byte slice of appropriate length.
/// 
/// Since the result can't be used to modify the source
/// structure, this is perfectly safe to do on any type,
/// even if mostly useless for most Rust types.
///
/// This function cannot fail.
///
/// <strong>Beware:</strong> Types that are not `#repr(C)` mostly have
/// unspecified layout, and in fact can even change
/// layout from build to build. Two types with exactly
/// same definition can even have different layout in 
/// the same build!
/// It's perfectly safe to read the data and e.g.
/// print it on screen for educational value, but
/// interpreting the data to mean anything is bound
/// to cause bugs. `#repr(C)` structures don't have
/// this problem as their layout is perfectly well
/// defined.
///
#[inline]
pub fn as_bytes<S>(s: &S) -> &[u8]
    where S: ?Sized
{
    // Even though slices can't under normal circumstances be cast
    // to pointers, here in generic code it works.
    // This means that `s` can be a slice or a regular reference,
    // at the caller's discretion.
    let bptr = s as *const S as *const u8;
    let bsize = mem::size_of_val(s);
    unsafe { slice::from_raw_parts(bptr, bsize) }
}

/// Safely converts a reference to a [`Plain`](trait.Plain.html) type to a mutable
/// byte slice of appropriate length.
/// 
/// In contrast to [`as_bytes()`](fn.as_bytes.html), argument must be `Plain`,
/// in order to prevent invoking UB via writing illegal
/// values into it.
///
/// This function cannot fail.
///
#[inline]
pub fn as_mut_bytes<S>(s: &mut S) -> &mut [u8]
    where S: Plain + ?Sized
{
    let bptr = s as *mut S as *mut u8;
    let bsize = mem::size_of_val(s);
    unsafe { slice::from_raw_parts_mut(bptr, bsize) }
}

/// Safely converts a byte slice to a [`Plain`](trait.Plain.html) type
/// reference.
///
/// However, if the byte slice is not long enough
/// to contain target type, or if it doesn't
/// satisfy the type's alignment requirements,
/// the function returns an error.
///
/// If the target type is a slice, its length will be
/// set appropriately to the length of the input slice.
/// E.g. a byte array of length 16 would result in
/// a slice `&[u32]` with 4 entries.
///
/// It is currently unspecified what happens when
/// the result is a slice and the input slice length
/// is not a multiple of the result type.
///
/// In most cases it's much preferrable to allocate
/// a value/slice of the target type and use
/// [`as_mut_bytes()`](fn.as_mut_bytes.html) instead.
///
#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<&T, Error>
    where T: Methods + ?Sized
{
    T::from_bytes(bytes)
}

/// See [`from_bytes()`](fn.from_bytes.html).
///
/// Does the same, except with mutable references.
#[inline]
pub fn from_mut_bytes<T>(bytes: &mut [u8]) -> Result<&mut T, Error>
    where T: Methods + ?Sized
{
    T::from_mut_bytes(bytes)
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

        let r6 = from_bytes::<[Dummy1]>(r5.as_bytes()).unwrap();

        assert!(r6.len() == 1);
        assert!(t1 == r6[0]);
    }
}
