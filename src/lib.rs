#![feature(asm, const_fn, no_std, lang_items, unique, collections)]
#![no_std]

extern crate collections;

extern crate rlibc;
extern crate spin;
extern crate alloc_toyos;

use core::fmt::Write;

// Needs to be visable to assebmly code.  This might not be the best way.
pub use arch::interrupts::rust_interrupt_handler;

#[macro_use]
mod macros;
mod arch;
mod console;

extern {
    /// The bottom of our heap.  Declared in `boot.asm` so that we can easily
    /// specify alignment constraints.
    static mut HEAP_BOTTOM: u8;

    /// The top of our heap.
    static mut HEAP_TOP: u8;
}

static mut FREE_LISTS: [*mut alloc_toyos::FreeBlock; 19] = [0 as *mut _; 19];

#[no_mangle]
pub extern "C" fn rust_main() {
    use arch::vga::{SCREEN, ColorScheme};
    use arch::vga::Color::*;

    SCREEN.lock()
          .clear(DarkGrey)
          .set_colors(ColorScheme::new(Yellow, DarkGrey));
    println!("Hello, world!");

    arch::interrupts::initialize();

    // Set up our basic system heap.
    unsafe {
        let heap_size =
            &mut HEAP_TOP as *mut _ as usize -
            &mut HEAP_BOTTOM as *mut _ as usize;
        alloc_toyos::initialize_allocator(&mut HEAP_BOTTOM as *mut _,
                                          heap_size,
                                          &mut FREE_LISTS);
    }

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

#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(
    args: ::core::fmt::Arguments, file: &str, line: usize)
    -> !
{
    println!("PANIC: {}:{}: {}", file, line, args);
    loop {}
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn _Unwind_Resume()
{
    println!("UNWIND!");
    loop {}
}

