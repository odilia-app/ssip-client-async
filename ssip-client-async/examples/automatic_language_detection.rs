#[cfg(all(unix, not(feature = "async-mio")))]
use ssip_client_async::{fifo, ClientResult};

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() -> ClientResult<()> {
    use ssip::ClientName;

    let languages = vec![lingua::IsoCode639_1::EN, lingua::IsoCode639_1::RU];
    let mut client = fifo::Builder::new()
        // initialize the language detection model with a list of languages to distinguish between
        .with_automatic_detection_languages(&languages)
        // spawn the speech-dispatcher daemon before creating the client
        // and trying to connect to the speech-dispatcher socket
        .with_spawn()?
        .build()?;

    client
        .set_client_name(ClientName::new("sasha", "example_app"))?
        .check_client_name_set()?
        .speak()?
        .check_receiving_data()?
        .send_lines_multilingual(
            &"Hello, my name is Joe. меня зовут джо".to_string(),
        )?;

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
