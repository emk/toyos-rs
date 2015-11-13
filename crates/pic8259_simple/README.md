# Kernel-space interface to 8259 and 8259A Programmable Interrupt Controller (PIC)

**Work in progress:** I am _not_ qualified to have written this crate.
This has been verified to work in simple cases in QEMU.  It may break on
real hardware (especially buggy hardware) or in more complicated scenarios.
Your bug reports and PRs are extremely welcome.  **Things we may not handle
very well yet include:**

1. Masking interrupts.
2. Dealing with spurious interrupts.
3. Non-standard configurations.

This code is based on the [OSDev Wiki PIC notes][PIC], but it's not a
complete implementation of everything they discuss.  Also note that if you
want to do more sophisticated interrupt handling, especially on
multiprocessor systems, you'll probably want to read about the newer
[APIC][] and [IOAPIC][] interfaces.

[PIC]: http://wiki.osdev.org/8259_PIC
[APIC]: http://wiki.osdev.org/APIC
[IOAPIC]: http://wiki.osdev.org/IOAPIC

## Using

This is a very basic interface to the 8259 and 8259A interrupt controllers,
which are used on single processor systems to pass hardware interrupts to
the CPU.

To use this crate, add it to your `Cargo.toml` file, along with an
appropriate kernel-space mutex implementation such as `spin`:

```toml
[dependencies]
pic8259_simple = "*"
spin = "*"
```

You can then declare a global, lockable `ChainedPics` object as follows:

```rust
extern crate pic8259_simple;
extern crate spin;

use pic8259_simple::ChainedPics;
use spin::Mutex;

// Map PIC interrupts to 0x20 through 0x2f.
static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });
```

To perform runtime PIC intialization, call `initialize` before enabling
interrupts:

```rust
PICS.lock().initialize();
```

When you've finished handling an interrupt, run:

```rust
PICS.lock().notify_end_of_interrupt(interrupt_id);
```

It's safe to call `notify_end_of_interrupt` after every interrupt; the
`notify_end_of_interrupt` function will try to figure out what it needs to
do.

All public PIC interfaces are `unsafe`, because it's really easy to trigger
undefined behavior by misconfiguring the PIC or using it incorrectly.

## Licensing

Licensed under the [Apache License, Version 2.0][LICENSE-APACHE] or the
[MIT license][LICENSE-MIT], at your option.

[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
