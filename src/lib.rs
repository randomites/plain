
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
use std::prelude::*;

/// A trait for plain reinterpretable data.
///
/// A type can be Plain if it is #repr(C) and only contains
/// data with no possible invalid values. Specifically, 
/// bool, char, enums, tuples, pointers and references are not
/// Plain. On the other hand, statically sized arrays
/// of Plain type, and structures where all members
/// are Plain, are usually okay.
///
pub unsafe trait Plain {}

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

// TODO: check if floats are valid for every bit sequence.
// unsafe impl Plain for f32 {}
// unsafe impl Plain for f64 {}

unsafe impl<S> Plain for [S] where S: Plain {}

mod methods;
pub use self::methods::*;


/*
pub trait Slice {
    /// Converts reference to one Plain type into another,
    /// automatically determining appropriate length if
    /// T is an array.
    ///
    fn reinterpret<T>(&self) -> &[T]
        where T: Plain;

    /// Converts slice of one type into a slice of another,
    /// with output length provided as argument.
    ///
    /// == Panics ==
    /// The function will panic if the requested length 
    /// can't be satisfied by input slice.
    ///
    unsafe fn reinterpret_with_len<T>(&self, len: usize) -> &[T]
        where T: Copy;

    /// Converts a slice to a slice of bytes.
    fn as_bytes(&self) -> &[u8];
    
    /// Converts a slice into a mutable slice of bytes,
    /// allowing direct access to the former's representation in memory.
    ///
    /// == Safety ==
    /// It is not safe to modify bytes that correspond to types with
    /// invalid values. In particular, the input slice shouldn't contain values
    /// of bool or enum types. If such values are present, they must not be
    /// modified in a way that results in invalid values.
    ///
    unsafe fn as_mut_bytes(&mut self) -> &mut [u8];
}



impl<S> Slice for [S] where S: Plain {

    fn reinterpret<T>(&self) -> &[T]
        where T: Plain {
        /// This function is safe as long as the result type is
        /// a Copy type and doesn't have any invalid values.
        /// In particular, the result type shouldn't contain any
        /// bool or enum value. The Plain trait encodes this requirement.
    
        let byte_len = self.len() * size_of::<S>();
        let new_len = byte_len / size_of::<T>();
        unsafe { from_raw_parts(self.as_ptr() as *const T, new_len) }
    }

    unsafe fn reinterpret_with_len<T>(&self, len: usize) -> &[T]
        where T: Copy {

        let byte_len = self.len() * size_of::<S>();
        let new_len = byte_len / size_of::<T>();
        assert!(len <= new_len);	
        from_raw_parts(self.as_ptr() as *const T, len)
    }

    fn as_bytes(&self) -> &[u8] {
        // SAFE: So long as the slice is immutable, and we don't overshoot the length,
        // it is safe to view the memory as bytes.
        unsafe {
            from_raw_parts(self.as_ptr() as *const u8, self.len() * size_of::<S>())
        }
    }
    
    unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        from_raw_parts_mut(self.as_mut_ptr() as *mut u8, self.len() * size_of::<S>())
    }
}
*/
