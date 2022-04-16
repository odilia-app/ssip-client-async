// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(not(feature = "async-mio"))]
pub(crate) enum StreamMode {
    Blocking,
    NonBlocking,
    TimeOut(std::time::Duration),
}

#[cfg(test)]
mod tests {}
