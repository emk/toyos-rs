//! CPU-level input/output instructions, including `inb`, `outb`, etc., and
//! a high level Rust wrapper.

#![feature(asm, const_fn, no_std)]
#![no_std]

use core::marker::PhantomData;

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
pub use x86::{inb, outb, inw, outw, inl, outl};

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
mod x86;


/// This trait is defined for any type which can be read or written over a
/// port.  The processor supports I/O with `u8`, `u16` and `u32`.  The
/// functions in this trait are all unsafe because they can write to
/// arbitrary ports.
pub trait InOut {
    /// Read a value from the specified port.
    unsafe fn port_in(port: u16) -> Self;

    /// Write a value to the specified port.
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 { inb(port) }
    unsafe fn port_out(port: u16, value: u8) { outb(port, value); }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 { inw(port) }
    unsafe fn port_out(port: u16, value: u16) { outw(port, value); }
}

impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 { inl(port) }
    unsafe fn port_out(port: u16, value: u32) { outl(port, value); }
}

/// An I/O port over an arbitrary type supporting the `InOut` interface.
#[derive(Debug)]
pub struct Port<T: InOut> {
    // Port address.
    port: u16,

    // Zero-byte placeholder.  This is only here so that we can have a
    // type parameter `T` without a compiler error.
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    /// Create a new I/O port.  This is marked `unsafe` because it's the
    /// responsibility of the caller to make that we're pointed at a valid
    /// port address, and to make sure that returned port is used
    /// correctly.
    ///
    /// This is marked as `const` so that you can define ports with known
    /// addresses at compile time.
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    /// Read data from the port.  This is nominally safe, because you
    /// shouldn't be able to get hold of a port object unless somebody
    /// thinks it's safe to give you one.
    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    /// Write data to the port.
    pub fn write(&mut self, value: T) {
        unsafe { T::port_out(self.port, value); }
    }
}
