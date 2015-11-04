//! Low-level I/O functions.
//!
//! The processors in the x86 family have a low-level serial interface
//! which can be accessed using the `in` and `out` processor instructions.
//! We want to provide a nice, Rust-like wrapper over the raw assembly, and
//! do so in a way that allows us to generalize over the supported types.

use core::marker::PhantomData;


//=========================================================================
// Low-level API
//
// This is the traditional API for working directly with I/O ports.

/// Read a `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    // The registers for this instruction are always the same.
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outb(port: u16, value: u8) {
    // The registers for this instruction are always the same.
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

/// Read a `u16`-sized value from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    // The registers for this instruction are always the same.
    let result: u16;
    asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outw(port: u16, value: u16) {
    // The registers for this instruction are always the same.
    asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}

/// Read a `u32`-sized value from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    // The registers for this instruction are always the same.
    let result: u32;
    asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}

/// Write a `u32`-sized `value` to `port`.
pub unsafe fn outl(port: u16, value: u32) {
    // The registers for this instruction are always the same.
    asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}



//=========================================================================
// High-level API
//
// For normal use, we want a more Rust-like wrapper around an I/O port.

/// This trait is defined for any type which can be read or written over a
/// port.  The processor supports I/O with `u8`, `u16` and `u32`.  The
/// functions in this API are all unsafe because they can write to
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
    pub const unsafe fn new(port: u16) -> Self {
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
