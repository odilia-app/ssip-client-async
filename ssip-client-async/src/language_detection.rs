use std::io::{Read, Write};
// Trick to have common implementation for std and mio streams..
#[cfg(all(not(feature = "async-mio"), unix))]
pub use std::os::unix::io::AsRawFd as Source;

use crate::{fifo::Builder, Client, OK_LANGUAGE_SET};
use lingua::IsoCode639_1;
use ssip::{ClientError, ClientResult};

impl Builder {
    /// Initialize the language detection model with a list of languages to distinguish between
    /// Use the ISO 639-3 language codes to distinguish between languages
    pub fn with_automatic_detection_languages(
        &mut self,
        languages: &Vec<IsoCode639_1>,
    ) -> &mut Self {
        self.languages_to_detect = Some(languages.clone());
        self
    }
}

impl<S: Read + Write + Source> Client<S> {
    /// A wrapper over the `send_lines` method to send lines in multiple languages
    pub fn send_lines_multilingual(&mut self, lines: &String) -> ClientResult<&mut Self> {
        let detector =
            self.language_detector
                .as_ref()
                .ok_or(ClientError::LanguageDetectionError(
                    "Language detection not initialized".to_string(),
                ))?;

        let detection_results = detector.detect_multiple_languages_of(lines);

        for result in detection_results {
            let language_code = result.language().iso_code_639_1().to_string();
            // the status check stalls for some reason and never returns
            self.set_language(ssip::ClientScope::Current, &language_code)?
                .check_status(OK_LANGUAGE_SET)?;
            let subsection = lines[result.start_index()..result.end_index()].to_string();
            self.send_lines(&[subsection])?.receive()?;
        }

        Ok(self)
    }
}
