#![feature(no_std, lang_items)]
#![no_std]

#[repr(C)]
struct ColoredChar {
    c: u8,
    color: u8
}

#[no_mangle]
pub extern fn rust_main() {
    let buffer_ptr = 0xb8000 as *mut ColoredChar;
    let buffer: &mut [ColoredChar; 80 * 25] =
        unsafe { &mut *(buffer_ptr as *mut [_; 80*25]) };

    buffer[0] = ColoredChar { c: b'H', color: 14 };
    loop {};
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

