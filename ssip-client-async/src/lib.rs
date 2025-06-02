// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021-2022 Laurent Pelecq
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
//! See [`client::Client`] for the synchronous API and [`poll::QueuedClient`] for the asynchronous API.
//!
//! Example
//! ```no_run
//! use ssip_client_async::{fifo, ClientName};
//! let mut client = fifo::Builder::new().build()?;
//! client
//!     .set_client_name(ClientName::new("joe", "hello"))?
//!     .check_client_name_set()?;
//! let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
//! client.quit()?;
//! # Ok::<(), ssip_client_async::ClientError>(())
//! ```

#[macro_use]
mod protocol;

mod poll;
pub use ssip as types;

pub mod client;
pub mod constants;
#[cfg(unix)]
pub mod fifo;
pub mod net;
pub mod tcp;

#[cfg(not(feature = "async-mio"))]
pub use client::Client;

#[cfg(feature = "async-io")]
pub mod async_io;
#[cfg(feature = "tokio")]
pub mod tokio;

pub use constants::*;
pub use poll::QueuedClient;
pub use types::*;
