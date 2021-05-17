// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # SSIP client
//!
//! `ssip-client` implements a Speech Dispatcher SSIP client library in
//! pure rust.
//!
//! See [`Client`] API for details.
//!
//! Example
//! ```no_run
//! let mut client = ssip_client::new_unix_client("joe", "hello", "main")?;
//! let msg_id = client.speak1("hello")?;
//! client.quit()?;
//! # Ok::<(), ssip_client::ClientError>(())
//! ```

#[macro_use]
mod protocol;

mod client;
mod constants;
mod unix;

pub use client::{Client, ClientError, ClientResult, ClientStatus, StatusLine};
pub use constants::*;
pub use unix::new_client as new_unix_client;
