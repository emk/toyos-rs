// Export our platform-specific modules.
pub use self::platform::*;

// Implementations for x86_64.
#[cfg(target_arch="x86_64")]
#[path="x86_64"]
mod platform {
    pub mod vga;
    pub mod interrupts;
}
