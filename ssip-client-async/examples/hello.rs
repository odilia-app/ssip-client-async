#[cfg(all(unix, not(feature = "async-mio")))]
use ssip_client::{fifo, ClientName, ClientResult, ClientScope, OK_LANGUAGE_SET};

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() -> ClientResult<()> {
    let mut client = fifo::Builder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "hello"))?
        .check_client_name_set()?
        .set_language(ClientScope::Current, "zh")?
        .check_status(OK_LANGUAGE_SET)?;
    let msg_id = client
        .speak()?
        .check_receiving_data()?
        .send_line("你好")?
        .receive_message_id()?;
    println!("message: {}", msg_id);
    let volume = client.get_volume()?.receive_u8()?;
    println!("volume: {}", volume);
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
