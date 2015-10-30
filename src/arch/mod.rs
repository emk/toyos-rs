// Export our platform-specific modules.
#[cfg(target_arch="x86_64")]
pub use self::x86_64::*;

// Implementations for x86_64.
#[cfg(target_arch="x86_64")]
#[macro_use]
pub mod x86_64 {
    #[macro_use]
    pub mod vga;
    pub mod interrupts;
}
