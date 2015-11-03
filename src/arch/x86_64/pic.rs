//! Support for the 8259 Programmable Interrupt Controller, which handles
//! basic I/O interrupts.  In multicore mode, we would apparently need to
//! replace this with the APIC.

use arch::x86_64::io::{inb, outb};

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;

const PIC1_OFFSET: u8 = 0x20;
const PIC2_OFFSET: u8 = 0x28;

const MODE_8086: u8 = 0x01;

/// Acknowledge that we have finished processing our interrupt, so that we
/// can get more.
pub unsafe fn end_of_interrupt(int_id: u8) {
    if int_id >= PIC2_OFFSET {
        outb(PIC2_COMMAND, CMD_END_OF_INTERRUPT);
    }
    outb(PIC1_COMMAND, CMD_END_OF_INTERRUPT);
}

/// Remap our I/O interrupts from 
pub unsafe fn remap() {
    let saved_mask1 = inb(PIC1_DATA);
    let saved_mask2 = inb(PIC2_DATA);

    outb(PIC1_COMMAND, CMD_INIT);
    outb(PIC2_COMMAND, CMD_INIT);
    outb(PIC1_DATA, PIC1_OFFSET);
    outb(PIC2_DATA, PIC2_OFFSET);
    outb(PIC1_DATA, 4);
    outb(PIC2_DATA, 2);
    outb(PIC1_DATA, MODE_8086);
    outb(PIC2_DATA, MODE_8086);

    outb(PIC1_DATA, saved_mask1);
    outb(PIC2_DATA, saved_mask2);

    // Enable only keyboard interrupts for now.
    // outb(PIC1_DATA, 0xfd);
    // outb(PIC2_DATA, 0xff);
}
