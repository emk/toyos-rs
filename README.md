# A toy OS in Rust

This is based on Philipp Oppermann's
[excellent series of blog posts][blog].  It's purely a learning exercise,
to see what Rust feels like on bare metal.

[blog]: http://blog.phil-opp.com/
[rust-barebones-kernel]: https://github.com/thepowersgang/rust-barebones-kernel

## Building

First, we need to check out the source and rebuild the Rust runtime using
bare-metal target and no floating point support:

```rust
git clone https://github.com/emk/toyos.rs.git
cd toyos.rs
multirust override nightly-2015-11-08
git submodule update --init
make runtime
```

Our copy of Rust has been patched to incoporate a version of the
`libcore_nofp.patch` from [rust-barebones-kernel][], and the `rust`
submodule points at a source tree that has been tested with the specific
`nightly` build mentioned above.

From here, we should be able to build a kernel and run it using QEMU:

```rust
make run
```

## Licensing

Licensed under the [Apache License, Version 2.0][LICENSE-APACHE] or the
[MIT license][LICENSE-MIT], at your option.

[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
