#![feature(const_fn, no_std, lang_items, unique)]
#![no_std]

extern crate spin;

mod arch;

#[no_mangle]
pub extern fn rust_main() {
    use arch::vga::{SCREEN, ColorScheme};
    use arch::vga::Color::*;

    let mut screen = SCREEN.lock();
    screen.clear(DarkGrey);
    screen.set_colors(ColorScheme::new(Yellow, DarkGrey));
    screen.write(b"Hello, world!");
    loop {};
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

