//! Rust wrappers around the x86-family I/O instructions.

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
