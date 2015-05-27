# Cargo bootstrap

[Cargo](http://doc.crates.io/) is Rust's package manager.
One of Cargo's build dependencies is Cargo, which poses a bit of a problem
on platforms without readily-available binary bootstrap releases.
This is an attempt to build a Cargo bootstrapper that only requires a working
Rust compiler (bootstrapping rustc is left as an exercise to the reader).

The goal is to enable bootstrapping with any POSIX-compliant `make` utility
(including `bmake` and `gmake`):

```shell
$ ARCH=x86_64 VENDOR=unknown TARGET_OS=freebsd make
rustc main.rs -o bootstrap
./bootstrap freebsd x86_64-unknown-freebsd
==> libc (rust-lang/libc)
Fetching...
Compiling...

==> bitflags (rust-lang/bitflags)
Fetching...
Compiling...

==> glob (rust-lang/glob)
Fetching...
Compiling...

==> log (rust-lang/log)
Fetching...
Compiling...

==> rand (rust-lang/rand)
Fetching...
Compiling...

==> regex (rust-lang/regex)
Fetching...
Compiling...

==> rustc_serialize (rust-lang/rustc-serialize)
Fetching...
Compiling...
[etc.]
```

This script doesn't quite work yet.
Please file issues using the GitHub issue tracker.
