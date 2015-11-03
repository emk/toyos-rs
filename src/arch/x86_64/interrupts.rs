/// WIP.  Some bits were sanity-checked against
/// https://github.com/ryanra/RustOS/blob/master/src/arch/x86/idt.rs
///
/// See section 6.10 of
/// http://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-manual-325462.pdf
///
/// See http://jvns.ca/blog/2013/12/04/day-37-how-a-keyboard-works/ for
/// some general advice on setting up interrupts and an entertaining saga
/// of frustration.

use core::mem::size_of;
use spin::Mutex;

use arch::x86_64::{pic, keyboard};

/// Maximum possible number of interrupts; we can shrink this later if we
/// want.
const IDT_ENTRY_COUNT: usize = 256;

#[allow(dead_code)]
extern {
    /// The offset of the main code segment in out GDT.  Exported by our
    /// assembly code.
    static gdt64_code_offset: u16;

    /// A primitive interrupt-reporting function.
    fn report_interrupt();

    /// Interrupt handlers which call back to rust_interrupt_handler.
    static int_handlers: [Option<unsafe extern "C" fn()>; IDT_ENTRY_COUNT];
}

/// Various data available on our stack when handling an interrupt.
///
/// Only `pub` because `rust_interrupt_handler` is.
#[repr(C, packed)]
pub struct InterruptContext {
    rsi: u64,
    rdi: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    int_id: u32,
    _pad_1: u32,
    error_code: u32,
    _pad_2: u32,
}

/// Called from our assembly-language interrupt handlers to dispatch an
/// interrupt.
#[no_mangle]
pub extern "C" fn rust_interrupt_handler(ctx: &InterruptContext) {
    match ctx.int_id {
        0x20 => {
            // Timer.
        }
        0x21 => {
            if let Some(input) = keyboard::read_char() {
                if input == '\r' {
                    println!("");
                } else {
                    print!("{}", input);
                }
            }
        }
        0x80 => println!("Not actually Linux, sorry."),
        _ => {
            println!("UNKNOWN INTERRUPT #{}", ctx.int_id);
            loop {}
        }
    }

    unsafe {
        pic::finish_interrupt_if_pic(ctx.int_id as u8);
    }
}

/// An entry in a 64-bit IDT table.  See the Intel manual mentioned above
/// for details, specifically, the section "6.14.1 64-Bit Mode IDT" and the
/// following data values from "Table 3-2. System-Segment and
/// Gate-Descriptor Types":
///
/// 1100 Call gate
/// 1110 Interrupt gate
/// 1111 Trap Gate
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct IdtEntry {
    offset_low: u16,
    segment: u16,
    flags: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    /// Create a IdtEntry marked as "absent".  Not tested with real
    /// interrupts yet.  This contains only simple values, so we can call
    /// it at compile time to initialize data structures.
    const fn absent() -> IdtEntry {
        IdtEntry {
            offset_low: 0,
            segment: 0,
            flags: 0b000_01110_000_00000,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    /// Create a new IdtEntry pointing at `handler`.
    fn new(handler: unsafe extern "C" fn()) -> IdtEntry {
        IdtEntry {
            offset_low: ((handler as u64) & 0xFFFF) as u16,
            segment: gdt64_code_offset,
            flags: 0b100_01110_000_00000,
            offset_mid: (((handler as u64) & 0xFFFF0000) >> 16) as u16,
            offset_high: (((handler as u64) & 0xFFFFFFFF00000000) >> 32) as u32,
            reserved: 0,
        }
    }
}

/// An Interrupt Descriptor Table which specifies how to respond to each
/// interrupt.
struct Idt {
    table: [IdtEntry; IDT_ENTRY_COUNT],
}

impl Idt {
    /// The base address of our IDT.
    fn base(&self) -> u64 {
        &self.table[0] as *const IdtEntry as u64
    }

    /// The size of our IDT.
    fn limit(&self) -> u16 {
        (size_of::<IdtEntry>() * IDT_ENTRY_COUNT) as u16
    }

    /// An IdtInfo describing our IDT's location and size.
    fn info(&self) -> IdtInfo {
        IdtInfo {
            limit: self.limit(),
            base: self.base(),
        }
    }
}

/// A 6-byte value describing an ID.  This is basically an extended
/// argument for use with the IDT function.
#[repr(C, packed)]
struct IdtInfo {
    limit: u16,
    base: u64,
}

impl IdtInfo {
    /// Load this IDT into our processor.
    pub unsafe fn load(&self) {
        asm!("lidt ($0)" :: "{rax}"(self) :: "volatile");
    }
}

/// Our global IDT.
static IDT: Mutex<Idt> = Mutex::new(Idt {
    table: [IdtEntry::absent(); IDT_ENTRY_COUNT]
});

/// Initialize interrupt handling.
pub fn initialize() {
    let mut idt = IDT.lock();

    // Fill in our IDT with our handlers.
    for (index, &opt_handler) in int_handlers.iter().enumerate() {
        if let Some(handler) = opt_handler {
            idt.table[index] = IdtEntry::new(handler);
        }
    }

    unsafe {
        // Load our IDT.
        idt.info().load();

        // Remap our PIC so I/O interrupts don't get confused with processor
        // interrupts.  (Who designed this stuff?)
        pic::initialize();

        // Enable this to trigger a sample interrupt.
        test_interrupt();

        // Turn on real interrupts.
        asm!("sti" :::: "volatile");
    }
}

/// Use the `int` instruction to manually trigger an interrupt without
/// actually using `sti` to enable interrupts.  This is highly recommended by
/// http://jvns.ca/blog/2013/12/04/day-37-how-a-keyboard-works/
#[allow(dead_code)]
pub unsafe fn test_interrupt() {
    println!("Triggering interrupt.");
    asm!("int $$0x80" :::: "volatile");
    println!("Interrupt returned!");
}
