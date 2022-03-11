use ssip_client::{ClientName, ClientResult, EventType, NotificationType};

fn main() -> ClientResult<()> {
    let mut client =
        ssip_client::new_default_fifo_client(&ClientName::new("joe", "notifications"), None)?;
    client.enable_notification(NotificationType::All).unwrap();
    let msg_id = client.say_line("hello")?;
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
