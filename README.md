# cbqn

A Rust crate for running [BQN](https://mlochbaum.github.io/BQN) code within a Rust program using [CBQN](https://github.com/dzaima/CBQN) interpreter shared object or WASI reactor.

# Building

## Native backend (libcbqn.so)

With the native backend (default), please make sure the libcbqn shared object can be found by rustc. Build `libcbqn.so` with `make shared-o3` in CBQN repository and set up `.cargo/config.toml` as following:

```
[build]
rustflags = "-L/path/to/CBQN"
rustdocflags = "-L/path/to/CBQN"
```

To make the executable aware of libcbqn.so, if no global install is available for you, you can use `LD_LIBRARY_PATH` to tell where to load `libcbqn.so` from:

```
LD_LIBRARY_PATH=/path/to/CBQN cargo test
```

## WASI backend (BQN.wasm)

To build the WASI backend, point the `BQN_WASM` environment variable to the `BQN.wasm` file built with `make wasi-reactor-o3` CBQN makefile target. Disable default features and use `wasi-backend` feature. For example running the tests for this crate:

```
BQN_WASM=/path/to/CBQN/BQN.wasm cargo test --no-default-features --features=wasi-backend --release
```

# Usage

The [documentation](https://detegr.github.io/cbqn-rs/cbqn) contains multiple examples on how to use the crate.

# License

Licensed either under GPLv3, LGPLv3 or MPL 2.0 following the licensing of [CBQN](https://github.com/dzaima/CBQN/).
