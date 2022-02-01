//! Rust wrappers around the x86-family I/O instructions.

use core::arch::asm;
/// Read an `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    let out;
    asm!("in al, dx", out("al") out, in("dx") port);
    out
}

/// Write an `u8`-sized `value` to `port`.
pub unsafe fn outb(value: u8, port: u16) {
    asm!("out dx, al", in("dx") port, in("al") value);
}

/// Read an `u16`-sized value from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    let out;
    asm!("in ax, dx", out("ax") out, in("dx") port);
    out
}

/// Write an `u16`-sized `value` to `port`.
pub unsafe fn outw(value: u16, port: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") value);
}

/// Read an `u32`-sized `value` from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    let out;
    asm!("in eax, dx", out("eax") out, in("dx") port);
    out
}

/// Write an `u32`-sized `value` to `port`.
pub unsafe fn outl(value: u32, port: u16) {
    asm!("out dx, eax", in("dx") port, in("eax") value);
}
