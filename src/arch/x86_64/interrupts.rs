/// WIP.  Some bits were sanity-checked against
/// https://github.com/ryanra/RustOS/blob/master/src/arch/x86/idt.rs

extern "C" {
    static gdt64_code_offset: u16;
    fn report_interrupt() -> ();
}

#[repr(C)]
struct IdtEntry {
    offset_lo: u16,
    segment: u16,
    flags: u16,
    offset_hi: u16,
}

impl IdtEntry {
    fn new() -> IdtEntry {
        IdtEntry{
            offset_lo: ((report_interrupt as u64) & 0xFFFF) as u16,
            segment: gdt64_code_offset,
            flags: 0,
            offset_hi: (((report_interrupt as u64) & 0xFFFF0000) >> 16) as u16,
        }
    }
}

pub fn initialize() {
    IdtEntry::new();
}
