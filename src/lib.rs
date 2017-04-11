//! A small Rust library that allows users to reinterpret data of certain types safely.
//!
//! This crate provides an unsafe trait [`Plain`](trait.Plain.html), which the user
//! of the crate uses to mark types for which operations of this library are safe.
//! See [`Plain`](trait.Plain.html) for the contractual obligation.
//!
//! Other than that, everything else in this crate is perfectly safe to use as long
//! as the `Plain` trait is not implemented on inadmissible types (similar to how
//! `Send` and `Sync` in the standard library work).
//!
//! # Examples
//!
//! To start using the crate, simply do `extern crate plain;`.
//!
//! If you want your plain types to have methods from this crate, also include `use plain.Plain;`.
//!
//! Then it's just a matter of marking the right types and using them.
//!
//! ```
//!
//! extern crate plain;
//! use plain::Plain;
//!
//!
//!
//! #[repr(C)]
//! #[derive(Default)]
//! struct ELF64Header {
//!     pub e_ident: [u8; 16],
//!     pub e_type: u16,
//!     pub e_machine: u16,
//!     pub e_version: u32,
//!     pub e_entry: u64,
//!     pub e_phoff: u64,
//!     pub e_shoff: u64,
//!     pub e_flags: u32,
//!     pub e_ehsize: u16,
//!     pub e_phentsize: u16,
//!     pub e_phnum: u16,
//!     pub e_shentsize: u16,
//!     pub e_shnum: u16,
//!     pub e_shstrndx: u16,
//! }
//!
//! // SAFE: ELF64Header satisfies all the requirements of `Plain`.
//! unsafe impl Plain for ELF64Header {}
//!
//! fn reinterpret_buffer(buf: &[u8]) -> &ELF64Header {
//!     match plain::from_bytes(buf) {
//!         Err(_) => panic!("The buffer is either too short or not aligned!"),
//!         Ok(elfref) => elfref,
//!     }
//! }
//!
//! fn copy_from_buffer(buf: &[u8]) -> ELF64Header {
//!     let mut h = ELF64Header::default();
//!     h.as_mut_bytes().copy_from_slice(buf);
//!     h
//! }
//!
//! #[repr(C)]
//! struct ArrayEntry {
//!     pub name: [u8; 64],
//!     pub tag: u32,
//!     pub score: u32,
//! }
//!
//! // SAFE: ArrayEntry satisfies all the requirements of `Plain`.
//! unsafe impl Plain for ArrayEntry {}
//!
//! fn array_from_bytes(buf: &[u8]) -> &[ArrayEntry] {
//!     // NOTE: length is not a concern here,
//!     // since slice_from_bytes() can return empty slice.
//!
//!     match plain::slice_from_bytes(buf) {
//!         Err(_) => panic!("The buffer is not aligned!"),
//!         Ok(arr) => arr,
//!     }
//! }
//!
//! # fn main() {}
//!
//! ```
//!
//! # Comparison to [`pod`](https://crates.io/crates/pod)
//!
//! [`pod`](https://crates.io/crates/pod) is another crate created to help working with plain data.
//! The major difference between `pod` and `plain` is scope.
//!
//! `plain` currently provides only a few functions (+method wrappers) and its implementation
//! involves very few lines of unsafe code. It can be used in `no_std` code. Also, it doesn't
//! deal with [endianness](https://en.wikipedia.org/wiki/Endianness) in any way,
//! so it is only suitable for certain kinds of low-level work.
//!
//! `pod`, on the other hand, provides a wide arsenal
//! of various methods, most of which may be unnecessary for a given use case.
//! It has dependencies on `std` as well as other crates, but among other things
//! it provides tools to handle endianness properly.
//!
//! In short, `plain` is much, much _plainer_...

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

use core::{mem, slice};

/// A trait for plain reinterpretable data.
///
/// A type can be [`Plain`](trait.Plain.html) if it is `#repr(C)` and only contains
/// data with no possible invalid values. Specifically,
/// `bool`, `char`, `enum`s, tuples, pointers and references are not
/// `Plain`. On the other hand, arrays of a `Plain` type, and
/// structures where all members are plain, are usually okay.
///
/// On top of this, implicit padding may technically be uninitialized bytes,
/// therefore reading them might constitute undefined behavior by definition.
/// As such, you currently must not apply `Plain` to structures with implicit
/// padding. I.e. the size of the whole struct (as returned by `mem::size_of`)
/// must be equal to the sum of sizes of its fields. The easiest way to assure
/// this is learning padding rules for `#repr(C)` and explicitly provide
/// padding as dummy fields where appropriate.
///
/// All methods of this trait are implemented automatically as wrappers
/// for crate-level funtions.
///
pub unsafe trait Plain {
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Result<&Self, Error>
        where Self: Sized
    {
        self::from_bytes(bytes)
    }

    #[inline]
    fn slice_from_bytes(bytes: &[u8]) -> Result<&[Self], Error>
        where Self: Sized
    {
        self::slice_from_bytes(bytes)
    }

    #[inline]
    fn slice_from_bytes_len(bytes: &[u8], len: usize) -> Result<&[Self], Error>
        where Self: Sized
    {
        self::slice_from_bytes_len(bytes, len)
    }

    #[inline]
    fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut Self, Error>
        where Self: Sized
    {
        self::from_mut_bytes(bytes)
    }

    #[inline]
    fn slice_from_mut_bytes(bytes: &mut [u8]) -> Result<&mut [Self], Error>
        where Self: Sized
    {
        self::slice_from_mut_bytes(bytes)
    }

    #[inline]
    fn slice_from_mut_bytes_len(bytes: &mut [u8], len: usize) -> Result<&mut [Self], Error>
        where Self: Sized
    {
        self::slice_from_mut_bytes_len(bytes, len)
    }

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self::as_bytes(self)
    }

    #[inline(always)]
    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self::as_mut_bytes(self)
    }
}

unsafe impl Plain for u8 {}
unsafe impl Plain for u16 {}
unsafe impl Plain for u32 {}
unsafe impl Plain for u64 {}
unsafe impl Plain for usize {}

unsafe impl Plain for i8 {}
unsafe impl Plain for i16 {}
unsafe impl Plain for i32 {}
unsafe impl Plain for i64 {}
unsafe impl Plain for isize {}

unsafe impl Plain for f32 {}
unsafe impl Plain for f64 {}

unsafe impl<S> Plain for [S] where S: Plain {}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    TooShort,
    BadAlignment,
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
fn check_slice_size<T>(bytes: &[u8], len: usize) -> Result<(), Error> {
    if bytes.len() < len * mem::size_of::<T>() {
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

/// Safely converts a reference to an immutable
/// byte slice of appropriate length.
///
/// This function cannot fail.
///
#[inline]
pub fn as_bytes<S>(s: &S) -> &[u8]
    where S: Plain + ?Sized
{
    // Even though slices can't under normal circumstances be cast
    // to pointers, here in generic code it works.
    // This means that `s` can be a slice or a regular reference,
    // at the caller's discretion.
    let bptr = s as *const S as *const u8;
    let bsize = mem::size_of_val(s);
    unsafe { slice::from_raw_parts(bptr, bsize) }
}

/// Safely converts a reference to a mutable
/// byte slice of appropriate length.
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

/// Safely converts a byte slice to a reference.
///
/// However, if the byte slice is not long enough
/// to contain target type, or if it doesn't
/// satisfy the type's alignment requirements,
/// the function returns an error.
///
/// However, the function will not fail when the
/// byte slice is longer than necessary, since it is
/// a common practice to interpret the beginning of
/// a slice as a fixed-size header.
///
/// In most cases it's preferrable to allocate
/// a value/slice of the target type and use
/// [`as_mut_bytes()`](fn.as_mut_bytes.html) to copy
/// data instead. That way, any issues with alignment
/// are implicitly avoided.
///
#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<&T, Error>
    where T: Plain
{
    try!(check_instance_size::<T>(bytes));
    try!(check_alignment::<T>(bytes));
    Ok(unsafe { &*(bytes.as_ptr() as *const T) })
}

/// Similar to [`from_bytes()`](fn.from_bytes.html),
/// except that the output is a slice of T, instead
/// of a reference to a single T. All concerns about
/// alignment also apply here, but size is handled
/// differently.
///
/// The result slice's length is set to be
/// `bytes.len() / size_of::<T>()`, and there
/// are no requirements for input size. I.e.
/// the result may be empty slice, and the input
/// slice doesn't necessarily have to end on `T`'s
/// boundary. The latter has pragmatic reasons: If the
/// length of the array is not known in advance,
/// e.g. if it's terminated by a special element,
/// it's perfectly legal to turn the whole rest
/// of data into `&[T]` and set the proper length
/// after inspecting the array.
///
/// In most cases it's preferrable to allocate
/// a value/slice of the target type and use
/// [`as_mut_bytes()`](fn.as_mut_bytes.html) to copy
/// data instead. That way, any issues with alignment
/// are implicitly avoided.
///
/// ## Example
///
/// ```rust,should_panic
/// use plain::Plain;
/// let bytes = &[ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ];
/// let nums: &[u32] = u32::slice_from_bytes(bytes).unwrap();
/// // Oops! This `unwrap()` will actually panic, in some cases!
/// // The byte slice is NOT aligned! Don't write code like this!
///
/// // If the above doesn't panic, this holds:
/// assert_eq!(nums.len(), 3);
/// # assert!(false);
/// ```
#[inline]
pub fn slice_from_bytes<T>(bytes: &[u8]) -> Result<&[T], Error>
    where T: Plain
{
    let len = bytes.len() / mem::size_of::<T>();
    slice_from_bytes_len(bytes, len)
}


/// Same as [`slice_from_bytes()`](fn.slice_from_bytes.html),
/// except that it takes explicit length of the result slice.
///
/// If the input slice can't satisfy the length, returns error.
/// The input slice is allowed to be longer than necessary.
#[inline]
pub fn slice_from_bytes_len<T>(bytes: &[u8], len: usize) -> Result<&[T], Error>
    where T: Plain
{
    try!(check_alignment::<T>(bytes));
    try!(check_slice_size::<T>(bytes, len));
    Ok(unsafe { slice::from_raw_parts(bytes.as_ptr() as *const T, len) })
}

/// See [`from_bytes()`](fn.from_bytes.html).
///
/// Does the same, except with mutable references.
#[inline]
pub fn from_mut_bytes<T>(bytes: &mut [u8]) -> Result<&mut T, Error>
    where T: Plain
{
    try!(check_instance_size::<T>(bytes));
    try!(check_alignment::<T>(bytes));
    Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut T) })
}

/// See [`slice_from_bytes()`](fn.slice_from_bytes.html).
///
/// Does the same, except with mutable references.
#[inline]
pub fn slice_from_mut_bytes<T>(bytes: &mut [u8]) -> Result<&mut [T], Error>
    where T: Plain
{
    let len = bytes.len() / mem::size_of::<T>();
    slice_from_mut_bytes_len(bytes, len)
}

/// See [`slice_from_bytes_len()`](fn.slice_from_bytes_len.html).
///
/// Does the same, except with mutable references.
#[inline]
pub fn slice_from_mut_bytes_len<T>(bytes: &mut [u8], len: usize) -> Result<&mut [T], Error>
    where T: Plain
{
    try!(check_alignment::<T>(bytes));
    try!(check_slice_size::<T>(bytes, len));
    Ok(unsafe { slice::from_raw_parts_mut(bytes.as_ptr() as *mut T, len) })
}

#[cfg(test)]
mod tests;
