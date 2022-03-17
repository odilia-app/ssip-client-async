#[cfg(not(feature = "async-mio"))]
use ssip_client::{
    fifo, ClientName, ClientResult, EventType, NotificationType, OK_NOTIFICATION_SET,
};

#[cfg(not(feature = "async-mio"))]
fn main() -> ClientResult<()> {
    let mut client = fifo::Builder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "notifications"))?
        .check_client_name_set()?;
    // Enabling notifications
    client
        .enable_notification(NotificationType::All)?
        .check_status(OK_NOTIFICATION_SET)?;
    // Sending message
    let msg_id = client
        .speak()?
        .check_receiving_data()?
        .send_line("hello")?
        .receive_message_id()?;
    println!("message identifier: {}", msg_id);
    loop {
        // Waiting for event
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
    println!("exiting...");
    client.quit()?;
    Ok(())
}

#[cfg(feature = "async-mio")]
fn main() {
    println!("asynchronous client not implemented");
}
