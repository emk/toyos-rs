// The spin::Mutex + Uniq trick here is directly based on
// http://blog.phil-opp.com/rust-os/printing-to-screen.html

use core::ptr::Unique;
use spin::Mutex;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

#[repr(C)]
pub struct Char {
    pub code: u8,
    pub color: u8
}

pub struct Buffer {
    chars: [[Char; WIDTH]; HEIGHT]
}

pub struct Screen {
    buffer: Unique<Buffer>
}

impl Screen {
    pub fn write(&mut self, c: u8) {
        unsafe { self.buffer.get_mut() }.chars[0][0] = Char{code: c, color: 14};
    }
}

pub static SCREEN: Mutex<Screen> =
    Mutex::new(Screen{buffer: unsafe { Unique::new(0xb8000 as *mut _) }});
