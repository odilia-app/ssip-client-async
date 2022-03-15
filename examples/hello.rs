use ssip_client::{ClientName, ClientResult, FifoBuilder};

// ==============================
//   Synchronous implementation
// ==============================

#[cfg(not(feature = "metal-io"))]
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
    client.quit()?;
    Ok(())
}

// ==============================
//  Asynchronous implementation
// ==============================

#[cfg(feature = "metal-io")]
use mio::{Events, Poll, Token};

#[cfg(feature = "metal-io")]
use ssip_client::ClientError;

#[cfg(feature = "metal-io")]
fn increment<V>(result: ClientResult<V>) -> ClientResult<u16> {
    match result {
        Ok(_) => Ok(1),
        Err(ClientError::NotReady) => Ok(0),
        Err(err) => Err(err),
    }
}

#[cfg(feature = "metal-io")]
fn get_value<V>(result: ClientResult<V>) -> ClientResult<Option<V>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(ClientError::NotReady) => Ok(None),
        Err(err) => Err(err),
    }
}

#[cfg(feature = "metal-io")]
fn main() -> ClientResult<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut client = FifoBuilder::new().build()?;
    let token = Token(0);
    client.register(&poll, token)?;
    let mut step: u16 = 0;
    while step < 7 {
        poll.poll(&mut events, None)?;
        for event in &events {
            if event.token() == token {
                if event.is_writable() {
                    match step {
                        0 => {
                            step +=
                                increment(client.set_client_name(ClientName::new("test", "test")))?
                        }
                        2 => step += increment(client.speak())?,
                        4 => step += increment(client.send_line("hello"))?,
                        6 => step += increment(client.quit())?,
                        _ => (),
                    }
                } else if event.is_readable() {
                    match step {
                        1 => step += increment(client.check_client_name_set())?,
                        3 => step += increment(client.check_receiving_data())?,
                        5 => {
                            if let Some(msgid) = get_value(client.receive_message_id())? {
                                println!("Message identifier: {}", msgid);
                                step += 1;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    Ok(())
}
