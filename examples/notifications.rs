use ssip_client::{ClientName, ClientResult, EventType, NotificationType, OK_CLIENT_NAME_SET};

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client(None)?;
    client
        .open(ClientName::new("joe", "notifications"))?
        .check_status(OK_CLIENT_NAME_SET)?;
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
