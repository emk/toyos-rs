//! Basic PS/2 keyboard driver.
//!
//! Scancode table available at http://wiki.osdev.org/Keyboard#Scan_Code_Set_1

use spin::Mutex;
use arch::x86_64::io;

#[derive(Debug)]
struct KeyPair {
    left: bool,
    right: bool,
}

impl KeyPair {
    const fn new() -> Self {
        KeyPair { left: false, right: false }
    }

    fn is_pressed(&self) -> bool {
        self.left || self.right
    }
}

#[derive(Debug)]
struct Modifiers {
    shift: KeyPair,
    control: KeyPair,
    alt: KeyPair,
    caps_lock: bool,
}

impl Modifiers {
    const fn new() -> Self {
        Modifiers {
            shift: KeyPair::new(),
            control: KeyPair::new(),
            alt: KeyPair::new(),
            caps_lock: false,
        }
    }

    fn use_uppercase_letters(&self) -> bool {
        self.shift.is_pressed() ^ self.caps_lock
    }

    fn apply_to(&self, ascii: u8) -> u8 {
        if b'a' <= ascii && ascii <= b'z' {
            if self.use_uppercase_letters() {
                return ascii - b'a' + b'A';
            }
        }
        ascii
    }

    fn update(&mut self, scancode: u8) {
        match scancode {
            0x1D => self.control.left = true,
            0x2A => self.shift.left = true,
            0x36 => self.shift.right = true,
            0x38 => self.alt.left = true,
            0x3A => self.caps_lock = !self.caps_lock,
            0x9D => self.control.left = false,
            0xAA => self.shift.left = false,
            0xB6 => self.shift.right = false,
            0xB8 => self.alt.left = false,

            _ => {},
        }
    }
}

static MODIFIERS: Mutex<Modifiers> = Mutex::new(Modifiers::new());

fn read_scancode() -> u8 {
    unsafe { io::inb(0x60) }
}

fn find_ascii(scancode: u8) -> Option<u8> {
    let idx = scancode as usize;
    match scancode {
        0x01 ... 0x0E => Some(b"\x1B1234567890-=\0x02"[idx-0x01]),
        0x0F ... 0x1C => Some(b"\tqwertyuiop[]\r"[idx-0x0F]),
        0x1E ... 0x28 => Some(b"asdfghjkl;'"[idx-0x1E]),
        0x2C ... 0x35 => Some(b"zxcvbnm,./"[idx-0x2C]),
        0x39 => Some(b' '),
        _ => None,
    }
}

pub fn read_char() -> Option<char> {
    let mut mods = MODIFIERS.lock();
    let scancode = read_scancode();

    // Give our modifiers first crack at this.
    mods.update(scancode);

    // Look up the ASCII keycode.
    if let Some(ascii) = find_ascii(scancode) {
        Some(mods.apply_to(ascii) as char)
    } else {
        None
    }
}
