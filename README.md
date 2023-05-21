# cbqn

A Rust crate for running [BQN](https://mlochbaum.github.io/BQN) code within a Rust program using [CBQN](https://github.com/dzaima/CBQN) interpreter shared object or WASI reactor.

# Building

## Native backend (libcbqn.so)

With the native backend (default), please make sure the libcbqn shared object can be found by rustc. For example, assuming `make shared-o3` build has been run in CBQN directory, running the tests for this crate require:

```
LD_LIBRARY_PATH=/path/to/CBQN RUSTFLAGS="-L /path/to/CBQN" RUSTDOCFLAGS="$RUSTFLAGS" cargo test
```

## WASI backend (BQN.wasm)

To build the WASI backend, point the `BQN_WASM` environment variable to the `BQN.wasm` file built with `make wasi-reactor-o3` CBQN makefile target. Disable default features and use `wasi-backend` feature. For example running the tests for this crate:

```
BQN_WASM=/path/to/CBQN/BQN.wasm cargo test --no-default-features --features=wasi-backend --release
```

# Usage

The [documentation](https://detegr.github.io/cbqn-rs/cbqn) contains multiple examples on how to use the crate.

# License

Licensed under the [GNU General Public License v3.0](https://github.com/Detegr/cbqn-rs/blob/master/LICENSE). [CBQN](https://github.com/dzaima/CBQN) uses the same license.
