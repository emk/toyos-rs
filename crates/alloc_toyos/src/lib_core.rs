//! Main library file for when we're linked into the kernel.  For testing,
//! see `test_lib.rs`.
//!
//! This is our heap implementation (aka "malloc/free"), but provided in a
//! way that Rust can find and use it.  The crate `alloc_system` is a bit
//! magic: No other crates actually depend on it, but we need to tag it
//! `#[allocator]` and leave it where the compiler can find it.
//!
//! Also, we actually export C symbols, and not a regular Rust API.

#![feature(no_std, allocator)]
#![no_std]

#![crate_name = "alloc_system"]
#![crate_type = "rlib"]

// "Hey, compiler!  This is your heap implementation."
#![allocator]

extern crate lang_items_toyos;

use core::cmp;
use core::ptr;
use spinlock::Mutex;

mod heap::Heap;

/// The number of bytes to allocate for our kernel heap.
/// Assertion: `HEAP_SIZE == (HEAP_BOTTOM - HEAP_TOP)`.
const HEAP_SIZE: usize = 16*1024*1024;

/// The smallest block of memory we can allocate.
const MIN_BLOCK_SIZE: usize = 256;

/// The number of different block sizes that we support.
///
/// Assertion: `2^(BLOCK_SIZE_COUNT-1) * MIN_BLOCK_SIZE == HEAP_SIZE`
const BLOCK_SIZE_COUNT: usize = 17;

extern {
    /// The bottom of our heap.  Declared in `boot.asm` so that we can easily
    /// specify alignment constraints.
    static HEAP_BOTTOM: *mut u8;

    /// The top of our heap.
    static HEAP_TOP: *mut u8;
}

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    // TODO: Implement.
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    // TODO: Implement.
}

/// Attempt to resize an existing block of memory, preserving as much data
/// as possible.  For now, we always just allocate new memory, copy data,
/// and deallocate the old memory.
#[no_mangle]
pub extern "C" fn __rust_reallocate(
    ptr: *mut u8, old_size: usize, size: usize, align: usize)
    -> *mut u8
{
    let new = __rust_allocate(size, align);
    if new.is_null() {
        return new;
    } else {
        ptr::copy(ptr, new_ptr, cmp::min(size, old_size));
        __rust_deallocate(ptr, old_size, align);
        new_ptr
    }
}

/// We do not support in-place reallocation, so just return `old_size`.
#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(
    ptr: *mut u8, old_size: usize, size: usize, align: usize)
    -> usize
{
    old_size
}

/// I have no idea what this actually does, but we're supposed to have one,
/// and the other backends to implement it as something equivalent to the
/// following.
#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
