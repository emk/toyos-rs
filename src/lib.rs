#![feature(asm, const_fn, no_std, lang_items, unique, collections)]
#![no_std]

extern crate collections;

extern crate rlibc;
extern crate spin;
extern crate alloc_toyos;

use core::fmt::Write;

// These need to be visible to the linker, so we need to export them.
pub use arch::interrupts::rust_interrupt_handler;
pub use runtime_glue::*;

#[macro_use]
mod macros;
mod runtime_glue;
mod heap;
mod arch;
mod console;


#[no_mangle]
pub extern "C" fn rust_main() {
    use arch::vga::{SCREEN, ColorScheme};
    use arch::vga::Color::*;

    SCREEN.lock()
          .clear(DarkGrey)
          .set_colors(ColorScheme::new(Yellow, DarkGrey));
    println!("Hello, world!");

    arch::interrupts::initialize();
    unsafe { heap::initialize(); }

    let mut vec = collections::vec::Vec::<u8>::new();
    vec.push(1);
    println!("{:?}", vec);

    println!("Scanning PCI bus...");
    for function in arch::pci::functions() {
        println!("{}", function);
    }

    println!("Running.");

    loop {}
}
