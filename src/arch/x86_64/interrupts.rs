/// WIP.  Some bits were sanity-checked against
/// https://github.com/ryanra/RustOS/blob/master/src/arch/x86/idt.rs
///
/// See section 6.10 of
/// http://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-manual-325462.pdf

use core::mem::size_of;
use spin::Mutex;

/// Maximum possible number of interrupts; we can shrink this later if we
/// want.
const IDT_ENTRY_COUNT: usize = 256;

extern "C" {
    static gdt64_code_offset: u16;
    fn report_interrupt();
}

// "6.14.1 64-Bit Mode IDT"
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
    const fn absent() -> IdtEntry {
        IdtEntry{
            offset_low: 0,
            segment: 0,
            flags: 0b000_01110_000_00000,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    // Table 3-2. System-Segment and Gate-Descriptor Types
    // 1 1 0 0 64-bit call gate
    // 1 1 1 0 64-bit Interrupt Gate
    // 1 1 1 1 64-bit Trap Gate

    fn new(handler: unsafe extern "C" fn ()) -> IdtEntry {
        IdtEntry{
            offset_low: ((handler as u64) & 0xFFFF) as u16,
            segment: gdt64_code_offset,
            flags: 0b100_01110_000_00000,
            offset_mid: (((handler as u64) & 0xFFFF0000) >> 16) as u16,
            offset_high: (((handler as u64) & 0xFFFFFFFF00000000) >> 32) as u32,
            reserved: 0,
        }
    }
}

struct Idt {
    table: [IdtEntry; IDT_ENTRY_COUNT],
}

static IDT: Mutex<Idt> = Mutex::new(Idt{
    table: [IdtEntry::absent(); IDT_ENTRY_COUNT],
});

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

pub fn initialize() {
    let mut idt = IDT.lock();
    for entry in idt.table.iter_mut() {
        *entry = IdtEntry::new(report_interrupt);
    }
    let ptr = IdtPointer{
        limit: (size_of::<IdtEntry>() * IDT_ENTRY_COUNT) as u16,
        base: ((&(idt.table[0])) as *const IdtEntry) as u64,
    };

    println!("report_interrupt: {:?}", report_interrupt);
    println!("IdtEntry: {:?}", idt.table[1]);
    println!("Printed IdtEntry");
    unsafe {
        asm!("lidt ($0)" :: "{rax}"(&ptr) :: "volatile");
        // Test it.
        asm!("int $$0x01" :::: "volatile");
    }
}
