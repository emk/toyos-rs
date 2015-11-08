# A toy OS in Rust

This is based on Philipp Oppermann's
[excellent series of blog posts][blog].  It's purely a learning exercise,
to see what Rust feels like on bare metal.

[blog]: http://blog.phil-opp.com/
[rust-barebones-kernel]: https://github.com/thepowersgang/rust-barebones-kernel

## Building `libcore`

This is largely based on the approach taken by [rust-barebones-kernel][],
but with added support for cargo.  The `Makefile` assumes that you're using
a nighly build of run installed by `multirust`, and configured as an
override for the current directory:

```sh
multirust override nightly
```

First, you'll need to get your current version of `rustc`

```
$ rustc --version
rustc 1.6.0-nightly (2e07996a9 2015-10-29)
```

Remember the hexadecimal number in the parentheses, and check out a
matching source tree:

```sh
git clone https://github.com/rust-lang/rust
(cd rust && git checkout 2e07996a9)
```

Now you can try to patch `libcore` and install a set of basic runtime
libraries where `rustc` and `cargo` will find them:

```
make patch
make runtime
```

You may need to manually fix the `libcore` build to hide any new `f32` or
`f63` features behind `#[cfg(not(disable_float))]`.

## Building the kernel

```sh
make run
```
