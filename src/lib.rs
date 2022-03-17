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
//! See [`client::Client`] for the synchronous API and [`async_mio::AsyncClient`] for the asynchronous API based on [mio](https://crates.io/crates/mio).
//!
//! Example
//! ```no_run
//! use ssip_client::{fifo, ClientName};
//! let mut client = fifo::Builder::new().build()?;
//! client
//!     .set_client_name(ClientName::new("joe", "hello"))?
//!     .check_client_name_set()?;
//! let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
//! client.quit()?;
//! # Ok::<(), ssip_client::ClientError>(())
//! ```

#[macro_use]
mod protocol;

mod types;

pub mod client;
pub mod constants;
pub mod fifo;

#[cfg(any(not(feature = "async-mio"), doc))]
pub use client::Client;

pub use client::{ClientError, ClientName, ClientResult, ClientStatus};
pub use constants::*;
pub use types::StatusLine;
pub use types::*;

#[cfg(any(feature = "async-mio", doc))]
mod async_mio;

#[cfg(feature = "async-mio")]
pub use async_mio::{AsyncClient, Request, Response};
