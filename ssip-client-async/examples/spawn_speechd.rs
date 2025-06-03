#[cfg(all(unix, not(feature = "async-mio")))]
use ssip_client_async::{fifo, ClientResult};

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() -> ClientResult<()> {
    // spawn the speech-dispatcher daemon before creating the client
    // and trying to connect to the speech-dispatcher socket
    let mut client = fifo::synchronous::Builder::new().with_spawn()?.build()?;

    client
        .speak()?
        .check_receiving_data()?
        .send_line("test message for the spawn example")?
        .receive_message_id()?;

    client.quit()?.receive()?;
    Ok(())
}

#[cfg(all(unix, feature = "async-mio"))]
fn main() {
    println!("see async_mio_loop for an example of asynchronous client.");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
