use macro_rules_attribute::apply;
use smol_macros::main;
use ssip_client_async::{
    fifo::asynchronous_async_io::Builder,
    types::{ClientName, ClientResult, ClientScope},
};

#[cfg(all(unix, feature = "async-io"))]
main! {
async fn main() -> ClientResult<()> {
    println!("Example:");
    let mut client = Builder::default().build().await?;
    println!("Client created.");
    client
        .set_client_name(ClientName::new("test", "hello"))
        .await?
        .check_client_name_set()
        .await?;
    println!("Client connected");
    let msg_id = client
        .speak()
        .await?
        .check_receiving_data()
        .await?
        .send_line("hello\r\n.")
        .await?
        .receive_message_id()
        .await?;
    println!("message: {msg_id}");
    let volume = client.get_volume().await?.receive_u8().await?;
    println!("volume: {volume}");
    match client
        .set_volume(ClientScope::Current, 1)
        .await?
        .receive()
        .await
    {
        Ok(id) => println!("Volume change ID: {id:?}"),
        Err(e) => println!("Error: {e:?}"),
    };
    let volume = client.get_volume().await?.receive_u8().await?;
    println!("volume: {volume}");
    let msg_id = client
        .speak()
        .await?
        .check_receiving_data()
        .await?
        .send_line("hello\r\n.")
        .await?
        .receive_message_id()
        .await?;
    println!("id2: {msg_id}");
    client.quit().await?.receive().await?;
    Ok(())
}
}

#[cfg(all(unix, not(feature = "async-io")))]
fn main() {
    println!("see hello.rs for an example of a synchronous client.");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
