#![feature(const_fn, no_std, lang_items, unique)]
#![no_std]

extern crate spin;

mod vga;

#[no_mangle]
pub extern fn rust_main() {
    vga::SCREEN.lock().write(b'#');
    loop {};
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

