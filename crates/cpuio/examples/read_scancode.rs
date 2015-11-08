//! An example program showing how to read a single scancode from a PS/2
//! keyboard.

extern crate cpuio;

use cpuio::Port;

fn main() {
    let mut keyboard: Port<u8> = unsafe { Port::new(0x60) };
    // If you run this as an ordinary user in user space it will fail with
    // a SIGSEGV.
    println!("scancode: {}", keyboard.read());
}
