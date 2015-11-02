//! Support for the 8259 Programmable Interrupt Controller, which handles
//! basic I/O interrupts.  In multicore mode, we would apparently need to
//! replace this with the APIC.

use arch::x86_64::io::{inb, outb};

const Pic1Command: u16 = 0x20;
const Pic1Data: u16 = 0x21;
const Pic2Command: u16 = 0xA0;
const Pic2Data: u16 = 0xA1;

const CmdInit: u8 = 0x11;
const CmdEndOfInterrupt: u8 = 0x20;

const Pic1Offset: u8 = 0x20;
const Pic2Offset: u8 = 0x28;

const Mode8086: u8 = 0x01;

/// Acknowledge that we have finished processing our interrupt, so that we
/// can get more.
pub unsafe fn end_of_interrupt(int_id: u8) {
    if int_id >= Pic2Offset {
        outb(Pic2Command, CmdEndOfInterrupt);
    }
    outb(Pic1Command, CmdEndOfInterrupt);
}

/// Remap our I/O interrupts from 
pub unsafe fn remap() {
    let saved_mask1 = inb(Pic1Data);
    let saved_mask2 = inb(Pic2Data);

    outb(Pic1Command, CmdInit);
    outb(Pic2Command, CmdInit);
    outb(Pic1Data, Pic1Offset);
    outb(Pic2Data, Pic2Offset);
    outb(Pic1Data, 4);
    outb(Pic2Data, 2);
    outb(Pic1Data, Mode8086);
    outb(Pic2Data, Mode8086);

    outb(Pic1Data, saved_mask1);
    outb(Pic2Data, saved_mask2);

    // Enable only keyboard interrupts for now.
    //outb(Pic1Data, 0xfd);
    //outb(Pic2Data, 0xff);

    //outb(Pic1Command, CmdEndOfInterrupt);
    //outb(Pic2Command, CmdEndOfInterrupt);
}
