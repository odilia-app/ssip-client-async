use ssip_client::{ClientName, ClientResult, FifoBuilder};

// ==============================
//   Synchronous implementation
// ==============================

#[cfg(not(feature = "async-mio"))]
fn main() -> ClientResult<()> {
    let mut client = FifoBuilder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "hello"))?
        .check_client_name_set()?;
    let msg_id = client
        .speak()?
        .check_receiving_data()?
        .send_line("hello")?
        .receive_message_id()?;
    println!("message: {}", msg_id);
    let volume = client.get_volume()?.receive_u8()?;
    println!("volume: {}", volume);
    client.quit()?;
    Ok(())
}

// ==============================
//  Asynchronous implementation
// ==============================

#[cfg(feature = "async-mio")]
use mio::{Events, Poll, Token};

#[cfg(feature = "async-mio")]
mod control {

    use ssip_client::{ClientError, ClientResult};

    /// Controller to follow the sequence of actions and keep the socket state.
    pub struct Controller {
        step: u16,
        done: bool,
        writable: bool,
        next_is_read: bool,
    }

    impl Controller {
        pub fn new() -> Controller {
            Controller {
                step: 0,
                done: false,
                writable: false,
                next_is_read: false,
            }
        }

        /// Current step to execute.
        pub fn step(&self) -> u16 {
            self.step
        }

        /// Return true when done.
        pub fn done(&self) -> bool {
            self.done
        }

        /// If the next action is a read or the socket is not writable.
        pub fn must_poll(&self) -> bool {
            self.next_is_read || !self.writable
        }

        /// Record that the socket is writable.
        pub fn set_writable(&mut self) {
            self.writable = true;
        }

        /// Stop.
        pub fn stop(&mut self) {
            self.done = true;
        }

        /// Interpret the result of the action and move to the next step if necessary.
        ///
        /// When the socket is set to writable, no other writable event will be generated until
        /// the I/O returns error WouldBlock which is mapped to client error NotReady.
        pub fn next<V>(&mut self, next_is_read: bool, result: ClientResult<V>) -> ClientResult<()> {
            match result {
                Ok(_) => {
                    self.step += 1;
                    self.next_is_read = next_is_read;
                    Ok(())
                }
                Err(ClientError::NotReady) => {
                    if !self.next_is_read {
                        // let's wait for the socket to become writable
                        self.writable = false;
                    }
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }

        /// Return the value returned by last read and move to next step.
        pub fn get_value<V>(&mut self, result: ClientResult<V>) -> ClientResult<Option<V>> {
            match result {
                Ok(value) => {
                    self.step += 1;
                    self.next_is_read = false;
                    Ok(Some(value))
                }
                Err(ClientError::NotReady) => Ok(None),
                Err(err) => Err(err),
            }
        }
    }
}

#[cfg(feature = "async-mio")]
fn main() -> ClientResult<()> {
    enum Action {
        None,
        Read,
        Write,
    }

    // Create the poll object, the client and register the socket.
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(16);
    let mut client = FifoBuilder::new().build()?;
    let input_token = Token(0);
    let output_token = Token(1);
    client.register(&poll, input_token, output_token)?;

    let mut ctl = control::Controller::new();
    while !ctl.done() {
        if ctl.must_poll() {
            // Poll is only necessary to read or if the last write failed.
            poll.poll(&mut events, None)?;
        }
        let mut event_iter = events.iter();
        loop {
            let event = event_iter.next();
            let action = match event {
                Some(event) if event.token() == output_token && event.is_writable() => {
                    ctl.set_writable();
                    Action::Write
                }
                Some(event) if event.token() == input_token && event.is_readable() => Action::Read,
                Some(_) => panic!("unexpected event"),
                None if ctl.must_poll() => Action::None,
                None => Action::Write, // Next action is write and socket is writable
            };
            match action {
                Action::Write => match ctl.step() {
                    0 => ctl.next(
                        true,
                        client.set_client_name(ClientName::new("test", "test")),
                    )?,
                    2 => ctl.next(true, client.speak())?,
                    4 => ctl.next(true, client.send_line("hello"))?,
                    6 => {
                        ctl.next(true, client.quit())?;
                        ctl.stop();
                        break;
                    }
                    _ => (),
                },
                Action::Read => match ctl.step() {
                    1 => ctl.next(false, client.check_client_name_set())?,
                    3 => ctl.next(false, client.check_receiving_data())?,
                    5 => {
                        if let Some(msgid) = ctl.get_value(client.receive_message_id())? {
                            println!("Message identifier: {}", msgid);
                        }
                    }
                    _ => (),
                },
                Action::None => break,
            }
        }
    }
    Ok(())
}
