# `ssip-common`

This common crate contains the core types used by the [`ssip-client-async`](https://github.com/odilia-app/ssip-client-async/) crate, although more crates may use this in the future.

This primary goal is simply to model all requests and responses for communication with the SSIP server.
There is not reason this could not be used to model a server as well, but there is not Rust implementation of this.

This crate is also used by the [Odilia screen reader project](https://github.com/odilia-app/), whose members maintain this crate.

Note that this crate can be compiled to many architectures: any Rust target that has `std` available should be able to compile this crate.
This includes:

* `x86_64-unknown-linux-gnu`
* `x86_64-unknown-linux-musl`
* `wasm32-unknown-unknwon`
* `wasm32-wasi`
* `aarch64-unknown-linux-musl`
* `aarch64-unknwon-linux-gnu`
* `x86_64-unknwon-freebsd`
* `x86_64-unknwon-openbsd`
* `x86_64-pc-windows-msvc`

All these are tested in our CI, and [feel free to open an issue](https://github.com/odilia-app/ssip-client-async/issues/) if you'd like to add another officially supported architecture.
