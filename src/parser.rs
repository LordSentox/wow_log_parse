use crate::Event;
use std::mem;

/// Parse a warcraft logs string or file into an event object.
pub fn parse<S: AsRef<str>>(log: S) -> Vec<Event> {
    let mut result = Vec::new();
    for (e, l) in log.as_ref().lines().enumerate() {
        // TODO: At the moment, errors are only logged. They should instead be
        // made accessible via the API.
        match Event::from_str(l) {
            Ok(event) => result.push(event),
            Err(err) => error!(
                "Error parsing, line {}, column {}: {:?}",
                e + 1,
                err.col(),
                err.typ()
            )
        }
    }

    info!(
        "Loaded {} events successfully into memory ({} KBytes)",
        result.len(),
        mem::size_of::<Event>() * result.len() / 1024
    );
    result
}
