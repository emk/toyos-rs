// The spin::Mutex + Uniq trick here is directly based on
// http://blog.phil-opp.com/rust-os/printing-to-screen.html

use core::ptr::Unique;
use spin::Mutex;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

/// Standard VGA colors.
#[derive(Copy, Clone)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

/// VGA foreground and background color set.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorScheme {
    value: u8
}

impl ColorScheme {
    pub const fn new(fore: Color, back: Color) -> Self {
        ColorScheme{value: (back as u8) << 4 | (fore as u8)}
    }
}

/// A colored VGA character.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Char {
    pub code: u8,
    pub colors: ColorScheme,
}

type Buffer = [[Char; WIDTH]; HEIGHT];

/// A VGA screen, in character mode.
pub struct Screen {
    colors: ColorScheme,
    x: usize,
    y: usize,
    buffer: Unique<Buffer>,
}

impl Screen {
    /// Clear the screen to the specified color.
    pub fn clear(&mut self, color: Color) {
        let colors = ColorScheme::new(color, color);
        let c = Char{code: b' ', colors: colors};
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.buffer()[y][x] = c;
            }
        }
    }

    /// Set the current text colors.
    pub fn set_colors(&mut self, colors: ColorScheme) {
        self.colors = colors;
    }

    /// Write a string to the screen.
    pub fn write(&mut self, text: &[u8]) {
        for c in text {
            self.write_byte(*c);
        }
    }

    /// Write a single character to the screen.
    pub fn write_byte(&mut self, code: u8) {
        if code == b'\n' {
            self.y += 1;
        } else {
            let c = Char{code: code, colors: self.colors};
            self.buffer()[self.y][self.x] = c;
            self.x += 1;
            if self.x >= WIDTH {
                self.x = 0;
                self.y += 1;
            }
        }
        if self.y >= HEIGHT {
            self.y = 0;
            // TODO: Scroll.
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }
}

/// The system's VGA screen.
pub static SCREEN: Mutex<Screen> = Mutex::new(Screen{
    colors: ColorScheme::new(Color::White, Color::Black),
    x: 0,
    y: 0,
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});
