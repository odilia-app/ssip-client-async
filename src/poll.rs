// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::VecDeque;
use std::io::{Read, Write};

use crate::{
    client::{Client, Source},
    types::*,
};

// Hack to generate the doc. There must be a better way.
#[cfg(all(not(feature = "async-mio"), doc))]
mod mio {
    /// Polls for readiness events on all registered values.
    ///
    /// See [`mio::Poll`](https://docs.rs/mio/latest/mio/struct.Poll.html#)
    pub struct Poll {}

    /// Source identifier.
    ///
    /// See [`mio::Token`](https://docs.rs/mio/latest/mio/struct.Token.html#).
    pub struct Token(pub usize);
}

const INITIAL_REQUEST_QUEUE_CAPACITY: usize = 4;

/// Client with a queue of requests.
///
/// The client can be used with crates like [popol](https://crates.io/crates/popol) or
/// with [mio](https://crates.io/crates/mio) if feature `async-mio` is enabled.
///
/// When the output is ready, a next event can be sent.
pub struct QueuedClient<S: Read + Write + Source> {
    client: Client<S>,
    requests: VecDeque<Request>,
}

impl<S: Read + Write + Source> QueuedClient<S> {
    /// New asynchronous client build on top of a synchronous client.
    pub fn new(client: Client<S>) -> Self {
        Self {
            client,
            requests: VecDeque::with_capacity(INITIAL_REQUEST_QUEUE_CAPACITY),
        }
    }

    #[cfg(all(not(feature = "async-mio"), unix))]
    /// Input source.
    pub fn input_source(&self) -> &S {
        self.client.input_source()
    }

    #[cfg(all(not(feature = "async-mio"), unix))]
    /// Output source.
    pub fn output_source(&self) -> &S {
        self.client.output_source()
    }

    #[cfg(any(feature = "async-mio", doc))]
    /// Register client
    pub fn register(
        &mut self,
        poll: &mio::Poll,
        input_token: mio::Token,
        output_token: mio::Token,
    ) -> std::io::Result<()> {
        self.client.register(poll, input_token, output_token)
    }

    /// Push a new request in the queue.
    pub fn push(&mut self, request: Request) {
        self.requests.push_back(request);
    }

    /// Pop the last request in the queue.
    pub fn pop(&mut self) -> Option<Request> {
        self.requests.pop_back()
    }

    /// Last request in the queue.
    pub fn last(&self) -> Option<&Request> {
        self.requests.back()
    }

    /// Return true if there is a pending request.
    pub fn has_next(&self) -> bool {
        !self.requests.is_empty()
    }

    /// Write one pending request if any.
    ///
    /// Instance of `mio::Poll` generates a writable event only once until the socket returns `WouldBlock`.
    /// This error is mapped to `ClientError::NotReady`.
    pub fn send_next(&mut self) -> ClientResult<bool> {
        if let Some(request) = self.requests.pop_front() {
            self.client.send(request)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Receive one response.
    ///
    /// Must be called each time a readable event is returned by `mio::Poll`.
    pub fn receive_next(&mut self) -> ClientResult<Response> {
        self.client.receive()
    }
}
