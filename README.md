# cbqn

A Rust crate for running [BQN](https://mlochbaum.github.io/BQN) code within a Rust program using [CBQN](https://github.com/dzaima/CBQN) interpreter compiled as a shared object.

# Building

With the native backend (default), please make sure the libcbqn shared object can be found by rustc. For example, running the tests for this crate require:

```
LD_LIBRARY_PATH=/path/to/CBQN RUSTFLAGS="-L /path/to/CBQN" RUSTDOCFLAGS="$RUSTFLAGS" cargo test
```

# Usage

The [documentation](https://detegr.github.io/cbqn-rs/cbqn) contains multiple examples on how to use the crate.

# License

Licensed under the [GNU General Public License v3.0](https://github.com/Detegr/cbqn-rs/blob/master/LICENSE). [CBQN](https://github.com/dzaima/CBQN) uses the same license.
