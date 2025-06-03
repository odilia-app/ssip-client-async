Rust SSIP Client
================

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/odilia-app/ssip-client-async/blob/main/LICENSE-MIT)
[![Crates.io Version](https://img.shields.io/crates/v/ssip-client-async.svg)](https://crates.io/crates/ssip-client-async)

Speech Dispatcher [SSIP client library](http://htmlpreview.github.io/?https://github.com/brailcom/speechd/blob/master/doc/ssip.html) in pure rust.

- [x] Unix socket.
- [x] TCP socket.
- [x] Stop, cancel, pause and resume.
- [x] List, set voices.
- [x] Set rate, pitch, volume.
- [x] Notifications.
- [x] Message history.
- [x] `tokio` support.
- [x] `async-io` support.
- [x] separate protocol driving mechanism in the `ssip` crate.

Feature Flags
---------------

- `default`: none.
- `dbus`: add support to send these types over DBus via the `zbus` crate.
- `serde`: add support to serialize/deserialize the types with `serde`.
- `async-io`: add support for the `smol`/`async-io` runtime. This _does not pull in an entire runtime, it only adds generic `async` integration points_.
- `tokio`: add support for the `tokio` runtime. This will pull in the `tokio` runtime along with support for its `tokio::io::Async*` traits.
- `async-mio`: add support for the low-level `mio` polling mechanism.

Example
-------

```rust
use ssip_client_async::{FifoBuilder, ClientName};
let mut client = fifo::Builder::new().build()?;
client
    .set_client_name(ClientName::new("joe", "hello"))?
    .check_client_name_set()?;
let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
client.quit()?;
```

See [other examples](./ssip-client-async/examples) in the repository.

License
-------

This software is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
