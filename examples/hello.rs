#[cfg(not(feature = "async-mio"))]
use ssip_client::{fifo, ClientName, ClientResult};

#[cfg(not(feature = "async-mio"))]
fn main() -> ClientResult<()> {
    let mut client = fifo::Builder::new().build()?;
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

#[cfg(feature = "async-mio")]
fn main() {
    println!("see async_mio_loop for an example of asynchronous client.");
}
