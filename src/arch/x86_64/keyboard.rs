use arch::x86_64::io;

pub fn read_scancode() -> u8 {
    unsafe { io::inb(0x60) }
}
