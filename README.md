Rust SSIP Client
================

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://gitlab.com/lp-accessibility/ssip-client/raw/main/LICENSE-MIT)
[![Crates.io Version](https://img.shields.io/crates/v/ssip-client-async.svg)](https://crates.io/crates/ssip-client-async)

Speech Dispatcher [SSIP client library](http://htmlpreview.github.io/?https://github.com/brailcom/speechd/blob/master/doc/ssip.html) in pure rust.

The API is synchronous by default.

A non-blocking API can be used with a low-level polling mechanism based on `poll`, or
with [mio](https://github.com/tokio-rs/mio).
This fork also offers a working version using the `tokio` flag, or an occasionally working `async-std` flag.

- [x] Unix socket.
- [x] TCP socket.
- [x] Stop, cancel, pause and resume.
- [x] List, set voices.
- [x] Set rate, pitch, volume.
- [x] Notifications.
- [x] Message history.
- [x] `tokio` support.
- [ ] `async-std` support.
	- If you're interested in helping us implement this, please reach out [on Gituhb](https://github.com/odilia-app/ssip-client-async/issues).

Getting Started
---------------

To use the synchronous API or an asynchronous API compatible with low-level crates based on `poll`, use:

```toml
[dependencies]
ssip-client-async = "0.9"
```

For the tokio API, use:

```toml
[dependencies]
ssip-client = { version = "0.9", features = ["tokio"] }
```

For use with the `zbus` DBus API, use the `dbus` feature.

Example
-------

```rust
use ssip_client::{FifoBuilder, ClientName};
let mut client = fifo::Builder::new().build()?;
client
    .set_client_name(ClientName::new("joe", "hello"))?
    .check_client_name_set()?;
let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
client.quit()?;
```

See [other examples](https://github.com/odilia-app/ssip-client-async/tree/main/examples) in the repository.

License
-------

This software is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
