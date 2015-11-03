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

use spin::Mutex;
use arch::x86_64::io;

// Commands we need to send.
const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;

// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

/// This interface is implemented by each of our individual `Pic` chips,
/// and by the subsystem as a whole.
trait HandlesInterrupt {
    /// Are we in change of handling the specified interrupt?
    fn handles_interrupt(&self, interrupt_id: u8) -> bool;

    /// Notify us that an interrupt has been handled and that we're ready
    /// for more.
    fn end_of_interrupt(&mut self, interrupt_id: u8);
}

struct Pic {
    /// The base offset to which our interrupts are mapped.
    offset: u8,

    /// The processor I/O port on which we send commands.
    command: io::Port<u8>,

    /// The processor I/O port on which we send and receive data.
    data: io::Port<u8>,
}

impl HandlesInterrupt for Pic {
    /// Each PIC handles 8 interrupts.
    fn handles_interrupt(&self, interupt_id: u8) -> bool {
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    fn end_of_interrupt(&mut self, _interrupt_id: u8) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}

/// A pair of chained PIC controllers.  This is the standard setup on x86.
struct Pics {
    pics: [Pic; 2],
}

impl Pics {
    /// Initialize both our PICs.  We initialize them together, at the same
    /// time, because it's traditional to do so, and because I/O operations
    /// might not be instantaneous on older processors.
    unsafe fn initialize(&mut self) {
        // Save our original interrupt masks, because I'm too lazy to
        // figure out reasonable values.  We'll restore these when we're
        // done.
        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        // Tell each PIC that we're going to send it a three-byte
        // initialization sequence on its data port.
        self.pics[0].command.write(CMD_INIT);
        self.pics[1].command.write(CMD_INIT);

        // Byte 1: Set up our base offsets.
        self.pics[0].data.write(self.pics[0].offset);
        self.pics[1].data.write(self.pics[1].offset);

        // Byte 2: Configure chaining between PIC1 and PIC2.
        self.pics[0].data.write(4);
        self.pics[1].data.write(2);

        // Byte 3: Set our mode.
        self.pics[0].data.write(MODE_8086);
        self.pics[1].data.write(MODE_8086);

        // Restore our saved masks.
        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
    }
}

impl HandlesInterrupt for Pics {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    /// Figure out which PICs in our chain need to know about this
    /// interrupt.  This is tricky, because all interrupts from `pics[1]`
    /// get chained through `pics[0]`.
    fn end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.pics[1].handles_interrupt(interrupt_id) {
            self.pics[1].end_of_interrupt(interrupt_id);
        }
        self.pics[0].end_of_interrupt(interrupt_id);
    }
}

/// The configuration and state of all our PICs.
static PICS: Mutex<Pics> = Mutex::new(Pics {
    pics: [
        Pic {
            offset: 0x20,
            command: unsafe { io::Port::new(0x20) },
            data: unsafe { io::Port::new(0x21) },
        },
        Pic {
            offset: 0x28,
            command: unsafe { io::Port::new(0xA0) },
            data: unsafe { io::Port::new(0xA1) },
        },
    ],
});

/// Initialize our PICs.
pub unsafe fn initialize() {
    let mut pics = PICS.lock();
    pics.initialize();
}

/// Acknowledge that we have finished processing our interrupt, so that we
/// can get more.  It is safe to call this on all interrupts; it's a noop
/// if we don't handle them.
pub unsafe fn finish_interrupt_if_pic(interrupt_id: u8) {
    let mut pics = PICS.lock();
    if pics.handles_interrupt(interrupt_id) {
        pics.end_of_interrupt(interrupt_id);
    }
}
