use arch::x86_64::io;

fn read_scancode() -> u8 {
    unsafe { io::inb(0x60) }
}

pub fn read_char() -> Option<char> {
    let b = read_scancode();
    // Scancode table available at
    // http://wiki.osdev.org/Keyboard#Scan_Code_Set_1
    match b {
        0x01 ... 0x0E => Some(b"\x1B1234567890-=\0x02"[b as usize - 0x01] as char),
        0x0F ... 0x1C => Some(b"\tqwertyuiop[]\r"[b as usize - 0x0F] as char),
        //0x1D => left control,
        0x1E ... 0x28 => Some(b"asdfghjkl;'"[b as usize - 0x1E] as char),
        0x2C ... 0x35 => Some(b"zxcvbnm,./"[b as usize - 0x2C] as char),
        0x39 => Some(' '),
        _ => None,
    }
}
