// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

pub mod synchronous {
    use std::io::{self, BufReader, BufWriter};
    pub use std::net::TcpStream;
    use std::net::{SocketAddr, ToSocketAddrs};
    use std::time::Duration;
    use std::vec;

    use crate::client::Client;
    use crate::net::StreamMode;

    struct Addresses(Vec<SocketAddr>);

    impl ToSocketAddrs for Addresses {
        type Iter = vec::IntoIter<SocketAddr>;
        fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
            Ok(self.0.clone().into_iter())
        }
    }

    pub struct Builder {
        addrs: Addresses,
        mode: StreamMode,
    }

    impl Builder {
        pub fn new<A: ToSocketAddrs>(addrs: A) -> io::Result<Self> {
            Ok(Self {
                addrs: Addresses(addrs.to_socket_addrs()?.collect::<Vec<SocketAddr>>()),
                mode: StreamMode::Blocking,
            })
        }

        pub fn timeout(&mut self, read_timeout: Duration) -> &mut Self {
            self.mode = StreamMode::TimeOut(read_timeout);
            self
        }

        pub fn nonblocking(&mut self) -> &mut Self {
            self.mode = StreamMode::NonBlocking;
            self
        }

        pub fn build(&self) -> io::Result<Client<TcpStream>> {
            let input = TcpStream::connect(&self.addrs)?;
            match self.mode {
                StreamMode::Blocking => input.set_nonblocking(false)?,
                StreamMode::NonBlocking => input.set_nonblocking(true)?,
                StreamMode::TimeOut(timeout) => input.set_read_timeout(Some(timeout))?,
            }
            let output = input.try_clone()?;
            Ok(Client::new(BufReader::new(input), BufWriter::new(output)))
        }
    }
}

#[cfg(feature = "async-mio")]
pub mod asynchronous_mio {
    pub use mio::net::TcpStream;
    use std::io::{self, BufReader, BufWriter};
    use std::net::SocketAddr;
    use std::net::TcpStream as StdTcpStream;

    use crate::client::MioClient;

    pub struct Builder {
        addr: SocketAddr,
    }

    impl Builder {
        pub fn new(addr: SocketAddr) -> Self {
            Self { addr }
        }

        pub fn build(&self) -> io::Result<MioClient<TcpStream>> {
            let stream = StdTcpStream::connect(self.addr)?;
            Ok(MioClient::new(
                BufReader::new(TcpStream::from_std(stream.try_clone()?)),
                BufWriter::new(TcpStream::from_std(stream)),
            ))
        }
    }
}

#[cfg(test)]
mod tests {}
