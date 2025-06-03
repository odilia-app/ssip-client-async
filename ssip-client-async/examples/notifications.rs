#[cfg(all(unix, not(feature = "async-mio")))]
use ssip_client_async::{
    fifo, ClientName, ClientResult, EventType, NotificationType, OK_NOTIFICATION_SET,
};

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() -> ClientResult<()> {
    let mut client = fifo::synchronous::Builder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "notifications"))?
        .check_client_name_set()?;
    // Enabling notifications
    client
        .set_notification(NotificationType::All, true)?
        .check_status(OK_NOTIFICATION_SET)?;
    // Sending message
    let msg_id = client
        .speak()?
        .check_receiving_data()?
        .send_line("hello")?
        .receive_message_id()?;
    println!("message identifier: {msg_id}");
    loop {
        // Waiting for event
        match client.receive_event() {
            Ok(event) => {
                println!(
                    "event {}: message {} client {}",
                    event.ntype, event.id.message, event.id.client
                );
                if matches!(event.ntype, EventType::End) {
                    break;
                }
            }
            Err(err) => {
                eprintln!("error: {err:?}");
                break;
            }
        }
    }
    println!("exiting...");
    client.quit()?.receive()?;
    Ok(())
}

#[cfg(all(unix, feature = "async-mio"))]
fn main() {
    println!("asynchronous client not implemented");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
