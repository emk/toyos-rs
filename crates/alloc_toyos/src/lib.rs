//! Main library file for standalone testing.  See `lib.rs` for the file we
//! use when linked into the kernel.

#![feature(no_std)]
#![cfg_attr(not(test), feature(core_slice_ext))]
#![no_std]

mod math;
pub mod heap;
