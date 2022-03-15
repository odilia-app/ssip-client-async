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
//! use ssip_client::{new_default_fifo_client, ClientName, OK_CLIENT_NAME_SET};
//! let mut client = new_default_fifo_client(None)?;
//! client
//!     .open(ClientName::new("joe", "hello"))?
//!     .check_status(OK_CLIENT_NAME_SET)?;
//! let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
//! client.quit()?;
//! # Ok::<(), ssip_client::ClientError>(())
//! ```

#[macro_use]
mod protocol;

mod client;
mod constants;
mod fifo;
mod types;

pub use client::{Client, ClientError, ClientName, ClientResult, ClientStatus};
pub use constants::*;
pub use fifo::{new_default_fifo_client, new_fifo_client};
pub use types::StatusLine;
pub use types::*;
