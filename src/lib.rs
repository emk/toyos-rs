#![feature(asm, const_fn, no_std, lang_items, unique, core_slice_ext, core_str_ext)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod arch;

#[no_mangle]
pub extern "C" fn rust_main() {
    use arch::vga::{SCREEN, ColorScheme};
    use arch::vga::Color::*;

    SCREEN.lock()
          .clear(DarkGrey)
          .set_colors(ColorScheme::new(Yellow, DarkGrey));
    println!("Hello, world!");

    arch::interrupts::initialize();

    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}
