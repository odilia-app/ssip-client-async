#[cfg(not(feature = "metal-io"))]
use ssip_client::{ClientName, ClientResult, EventType, FifoBuilder, NotificationType};

#[cfg(not(feature = "metal-io"))]
fn main() -> ClientResult<()> {
    let mut client = FifoBuilder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "notifications"))?
        .check_client_name_set()?;
    client.enable_notification(NotificationType::All).unwrap();
    let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
    println!("message: {}", msg_id);
    loop {
        match client.receive_event() {
            Ok(event) => {
                println!(
                    "event {}: message {} client {}",
                    event.ntype, event.message, event.client
                );
                if matches!(event.ntype, EventType::End) {
                    break;
                }
            }
            Err(err) => {
                eprintln!("error: {:?}", err);
                break;
            }
        }
    }
    client.quit()?;
    Ok(())
}

#[cfg(feature = "metal-io")]
fn main() {
    println!("asynchronous client not implemented");
}
