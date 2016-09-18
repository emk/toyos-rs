# A toy OS in Rust

This is based on Philipp Oppermann's
[excellent series of blog posts][blog].  It's purely a learning exercise,
to see what Rust feels like on bare metal.

[blog]: http://blog.phil-opp.com/
[rust-barebones-kernel]: https://github.com/thepowersgang/rust-barebones-kernel

## Building

First, we need to check out the source and rebuild the Rust runtime using a
bare-metal target and no hardware floating point support:

```sh
# Get our source code.
git clone https://github.com/emk/toyos-rs.git
cd toyos-rs

# Set up a Rust compiler.
curl https://sh.rustup.rs -sSf | sh
rustup update nightly-2016-09-16
rustup override set nightly-2016-09-16

# Get a copy of the Rust source code so we can rebuild core
# for a bare-metal target.
git submodule update --init
make runtime
```

From here, we should be able to build a kernel and run it using QEMU:

```sh
make run
```

You should be able to type.

## Licensing

Licensed under the [Apache License, Version 2.0][LICENSE-APACHE] or the
[MIT license][LICENSE-MIT], at your option.

[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
