//! Support for the 8259 Programmable Interrupt Controller, which handles
//! basic I/O interrupts.  In multicore mode, we would apparently need to
//! replace this with an APIC interface.
//!
//! The basic idea here is that we have two PIC chips, PIC1 and PIC2, and
//! that PIC2 is slaved to interrupt 2 on PIC 1.  You can find the whole
//! story at http://wiki.osdev.org/PIC (as usual).  Basically, our
//! immensely sophisticated modern chipset is engaging in early-80s
//! cosplay, and our goal is to do the bare minimum required to get
//! reasonable interrupts.
//!
//! The most important thing we need to do here is set the base "offset"
//! for each of our two PICs, because by default, PIC1 has an offset of
//! 0x8, which means that the I/O interrupts from PIC1 will overlap
//! processor interrupts for things like "General Protection Fault".  Since
//! interrupts 0x00 through 0x1F are reserved by the processor, we move the
//! PIC1 interrupts to 0x20-0x27 and the PIC2 interrupts to 0x28-0x2F.  If
//! we wanted to write a DOS emulator, we'd presumably need to choose
//! different base interrupts, because DOS used interrupt 0x21 for system
//! calls.

use core::ops::Range;
use arch::x86_64::io::{inb, outb};

// Our I/O ports.
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

// Commands we need to send.
const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;

// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

// Interrupts handled by PIC1.
const PIC1_INTERRUPTS: Range<u8> = Range { start: 0x20, end: 0x28 };

// Interrupts handled by PIC2.
const PIC2_INTERRUPTS: Range<u8> = Range { start: 0x28, end: 0x30 };

/// All interrupts handled by either PIC.
const PIC_INTERRUPTS: Range<u8> = Range {
    start: PIC1_INTERRUPTS.start,
    end: PIC2_INTERRUPTS.end,
};

/// Mysteriously, Rust does not seem to have a function to check membership
/// in a `Range`, so let's define and implement one.  Maybe I'm just missing
/// something added in recent versions?
trait Contains {
    /// Is `v` contained within this object?
    fn contains(&self, v: u8) -> bool;
}

impl Contains for Range<u8> {
    fn contains(&self, v: u8) -> bool {
        self.start <= v && v < self.end
    }
}

/// Acknowledge that we have finished processing our interrupt, so that we
/// can get more.  All I/O interrupts come through PIC1, either directly or
/// via chaining, so we always need to notify PIC1.  But if our interrupt
/// originated in PIC2, we also need to notify PIC2.
pub unsafe fn finish_interrupt_if_pic(int_id: u8) {
    if PIC_INTERRUPTS.contains(int_id) {
        if PIC2_INTERRUPTS.contains(int_id) {
            outb(PIC2_COMMAND, CMD_END_OF_INTERRUPT);
        }
        outb(PIC1_COMMAND, CMD_END_OF_INTERRUPT);
    }
}

/// Remap our I/O interrupts from 
pub unsafe fn remap() {
    // Save our original interrupt masks, because I'm too lazy to figure
    // out reasonable values.
    let saved_mask1 = inb(PIC1_DATA);
    let saved_mask2 = inb(PIC2_DATA);

    // Tell each PIC that we're going to send it a three-byte
    // initialization sequence on its data port.
    outb(PIC1_COMMAND, CMD_INIT);
    outb(PIC2_COMMAND, CMD_INIT);

    // Set up our base offsets.
    outb(PIC1_DATA, PIC1_INTERRUPTS.start);
    outb(PIC2_DATA, PIC2_INTERRUPTS.start);

    // Configure chaining between PIC1 and PIC2.
    outb(PIC1_DATA, 4);
    outb(PIC2_DATA, 2);

    // Set our mode.
    outb(PIC1_DATA, MODE_8086);
    outb(PIC2_DATA, MODE_8086);

    // OK, now we're done, so restore our saved masks.
    outb(PIC1_DATA, saved_mask1);
    outb(PIC2_DATA, saved_mask2);
}
