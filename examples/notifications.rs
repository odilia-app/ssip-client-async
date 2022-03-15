use ssip_client::{ClientName, ClientResult, EventType, NotificationType};

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client(None)?;
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
