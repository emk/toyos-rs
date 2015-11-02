pub unsafe fn outb(port: u16, value: u8) {
    // The registers for this instruction are always the same.
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

pub unsafe fn inb(port: u16) -> u8 {
    // The registers for this instruction are always the same.
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}
