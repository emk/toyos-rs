//! Low-level I/O functions.
//!
//! The processors in the x86 family have a low-level serial interface
//! which can be accessed using the `in` and `out` instructions.

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outb(port: u16, value: u8) {
    // The registers for this instruction are always the same.
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

/// Read a `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    // The registers for this instruction are always the same.
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}
