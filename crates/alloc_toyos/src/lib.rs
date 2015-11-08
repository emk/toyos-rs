//! Main library file for standalone testing.  See `lib.rs` for the file we
//! use when linked into the kernel.

#![feature(no_std)]
#![cfg_attr(not(test), feature(core_slice_ext))]
#![no_std]

#![cfg_attr(feature = "use-as-rust-allocator", feature(allocator))]
#![cfg_attr(feature = "use-as-rust-allocator", allocator)]

#![cfg(feature = "use-as-rust-allocator")]
extern crate spin;

#[cfg(feature = "use-as-rust-allocator")]
pub use integration::*;

mod math;
pub mod heap;

#[cfg(feature = "use-as-rust-allocator")]
mod integration;
