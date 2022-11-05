extern crate tokio;
use ssip_client::{fifo, ClientName, ClientResult};

#[cfg(all(unix, feature = "tokio"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> ClientResult<()> {
    let mut client = fifo::Builder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "hello"))?
        .check_client_name_set()?;
    let msg_id = client
        .speak()?
        .check_receiving_data()?
        .send_line("Lorem ipsum dollar mit amit dor BIG CHEESE! Hi 123 hi 123 hi 123 hi 123.")?
        .receive_message_id()?;
    println!("message: {}", msg_id);
    let volume = client.get_volume()?.receive_u8()?;
    println!("volume: {}", volume);
    client.quit()?;
    Ok(())
}

#[cfg(all(unix, not(feature = "tokio")))]
fn main() {
    println!("see hello.rs for an example of a synchronous client.");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
