# `alloc_buddy_simple`: A simple "buddy allocator" for bare-metal Rust

Are you using Rust on bare metal with `#[no_std]`?  Do you lack even a
working `malloc` and `free`?  Would you like to have a Rust-compatible
allocator that works with `libcollections`?

*WARNING:* OK, you shouldn't use `libcollections` for anything serious in
kernel space, because it will panic if you ever run out of memory.  But if
you just want to use a `Vec` or two at startup time, on a well-understood
system, it's very convenient, and maybe you're willing to live with the
consequences.

## Using this allocator

You can pull this into a Cargo build using:

```
[dependencies.alloc_buddy_simple]
git = "https://github.com/emk/toyos.rs.git
features = ["use-as-rust-allocator"]
```

Then you'll need to allocate some memory for your heap somewhere.  This
needs to be aligned on a 4096-byte boundary, and it needs to be a power of
2 in size.  You could use the following declarations with `nasm`:

```asm
section .bss
align 4096
HEAP_BOTTOM:
        resb 4*1024*1024
HEAP_TOP:
```

From there, all you need to do is (1) declare an array of free lists with
enough space:

```rust
extern crate alloc_buddy_simple;

use alloc_buddy_simple::{FreeBlock, initialize_allocator};

static mut FREE_LISTS: [*mut FreeBlock; 19] = [0 as *mut _; 19];
```

The tricky bit here is the `19`.  This determines the minimum allocable
block size, which will be `heap_size >> (19 - 1)`.  Your minimum block size
must be at least as large as a `FreeBlock`.

For calling `initialize_allocator`, see [the toyos `heap.rs` file][heap.rs]
for example code.  Do this before trying to use your heap, or you will get
a Rust panic!

[heap.rs]: https://github.com/emk/toyos.rs/blob/master/src/heap.rs

## Compiling a custom `libcollections`

You will need to manually compile a bunch of libraries from the `rust/src`
directory and copy them into
`~/.multirust/toolchains/nightly/lib/rustlib/$(target)/lib` or the
equivalent directory on your system.  For example code, see
[the toyos `Makefile`][Makefile].

You may also want to apply the [barebones nofp patch][nofp] if your kernel
space does not support

[Makefile]: https://github.com/emk/toyos.rs/blob/master/Makefile
[nofp]: https://github.com/thepowersgang/rust-barebones-kernel/blob/master/libcore_nofp.patch

## Warning

This has only been run in the "low half" of memory, and if you store your
heap in the upper half of your memory range, you may run into some issues
with `isize` versus `usize`.

## Licensing

Licensed under the Apache License, Version 2.0 [LICENSE-APACHE][] or the
MIT license [LICENSE-MIT][], at your option.  This is HIGHLY EXPERIMENTAL
CODE PROVIDED "AS IS", AND IT MAY DO HORRIBLE THINGS TO YOUR COMPUTER OR
DATA.  But if you're using random unsafe, unstable Rust libraries in
implementing a panicking version of `malloc` in kernel space, you probably
knew that already.

[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
