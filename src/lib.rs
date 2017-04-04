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
//! If you want your plain types to have methods from this crate, also include `use plain.Methods;`.
//!
//! Then it's just a matter of marking the right types and using them.
//!
//! ```
//!
//! extern crate plain;
//! use plain::Methods;
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
//! unsafe impl plain::Plain for ELF64Header {}
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
//! unsafe impl plain::Plain for ArrayEntry {}
//! 
//! fn array_from_bytes(buf: &[u8]) -> &[ArrayEntry] {
//!     // NOTE: length is not a concern here,
//!     // since from_bytes() can return empty slice.
//! 
//!     match plain::from_bytes(buf) {
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
//! `plain` currently provides only four functions (+method wrappers) and its implementation involves
//! mere six lines of unsafe code. It can be used in `no_std` code. Also, it doesn't
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

/// A trait for plain reinterpretable data.
///
/// A type can be [`Plain`](trait.Plain.html) if it is `#repr(C)` and only contains
/// data with no possible invalid values. Specifically,
/// `bool`, `char`, `enum`s, tuples, pointers and references are not
/// `Plain`. On the other hand, arrays of a `Plain` type, and
/// structures where all members are plain, are usually okay.
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
