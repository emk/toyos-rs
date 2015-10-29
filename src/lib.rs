#![feature(const_fn, no_std, lang_items, unique, core_str_ext)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use] mod arch;

#[no_mangle]
pub extern fn rust_main() {
    use arch::vga::{SCREEN, ColorScheme};
    use arch::vga::Color::*;

    arch::interrupts::initialize();

    SCREEN.lock()
        .clear(DarkGrey)
        .set_colors(ColorScheme::new(Yellow, DarkGrey));
    println!("Hello, world!");

    loop {};
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

