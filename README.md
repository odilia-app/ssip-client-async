Rust SSIP Client
================

[![build status](https://gitlab.com/lp-accessibility/ssip-client/badges/main/pipeline.svg)](https://gitlab.com/lp-accessibility/ssip-client/commits/main)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://gitlab.com/lp-accessibility/ssip-client/raw/main/LICENSE-MIT)
[![Crates.io Version](https://img.shields.io/crates/v/ssip-client.svg)](https://crates.io/crates/ssip-client)
[![docs.rs](https://docs.rs/ssip-client/badge.svg)](https://docs.rs/ssip-client/latest/ssip_client/)

Speech Dispatcher [SSIP client library](http://htmlpreview.github.io/?https://github.com/brailcom/speechd/blob/master/doc/ssip.html) in pure rust.

- [x] Unix socket.
- [ ] TCP socket.
- [x] Stop, cancel, pause and resume.
- [x] List, set voices.
- [x] Set rate, pitch, volume.
- [x] Notifications.
- [ ] Message history.

Example
-------

```rust
use ssip_client::{new_default_fifo_client, ClientName, OK_CLIENT_NAME_SET};

let mut client = new_default_fifo_client(None)?;
client
    .open(ClientName::new("joe", "hello"))?
    .check_status(OK_CLIENT_NAME_SET)?;
let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
client.quit()?;
```

See [other examples](https://gitlab.com/lp-accessibility/ssip-client/-/tree/main/examples) in the repository.

License
-------

This software is distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
