use ssip_client::{ClientName, ClientResult, SynthesisVoice};

fn voice_to_string(voice: &SynthesisVoice) -> String {
    match &voice.language {
        Some(language) => match &voice.dialect {
            Some(dialect) => format!("{} [{}_{}]", voice.name, language, dialect),
            None => format!("{} [{}]", voice.name, language),
        },
        None => format!("{}", voice.name),
    }
}

fn print_list(title: &str, values: &[String]) {
    println!("{}:", title);
    for val in values {
        println!("- {}", val);
    }
}

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client(&ClientName::new("joe", "list"), None)?;
    const OUTPUT_MODULE_TITLE: &str = "output modules";
    match client.list_output_modules() {
        Ok(values) => print_list(OUTPUT_MODULE_TITLE, &values),
        Err(err) => eprintln!("{}: {}", OUTPUT_MODULE_TITLE, err),
    };
    const VOICE_TYPES_TITLE: &str = "voice types";
    match client.list_voice_types() {
        Ok(values) => print_list(VOICE_TYPES_TITLE, &values),
        Err(err) => eprintln!("{}: {}", VOICE_TYPES_TITLE, err),
    };
    const SYNTHESIS_VOICES_TITLE: &str = "synthesis voices";
    match client.list_synthesis_voices() {
        Ok(values) => print_list(
            SYNTHESIS_VOICES_TITLE,
            &values
                .iter()
                .map(|ref v| voice_to_string(v))
                .collect::<Vec<String>>(),
        ),
        Err(err) => eprintln!("{}: {}", VOICE_TYPES_TITLE, err),
    };

    client.quit().unwrap();
    Ok(())
}
