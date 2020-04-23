use crate::event::Event;
use crate::filter::{Filter, Filterable};
use crate::FilteredEvents;
use bitvec::prelude::*;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum ParseErrorType {
    IOErr, // TODO: Add io::Error to this
    WrongHeadLength,
    WrongTimeFormat,
    InvalidArg,
    UnknownEventType(String)
}

#[derive(Clone, Debug)]
pub struct ParseError {
    typ: ParseErrorType,
    col: usize
}

pub struct Log {
    events: Vec<Event>
}

impl Log {
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Log, ParseError> {
        let mut file = match File::open(path.as_ref()) {
            Ok(file) => file,
            Err(err) => {
                error!("Could not open log file {:?}: {:?}", path.as_ref(), err);
                return Err(ParseError::new(ParseErrorType::IOErr, 0)); // XXX: This is stupid. This should not take a column
            }
        };

        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Ok(_) => {}
            Err(err) => {
                error!("Could not read log file {:?}: {:?}", path.as_ref(), err);
                return Err(ParseError::new(ParseErrorType::IOErr, 0));
            }
        }

        Log::from_str(&string)
    }

    pub fn events(&self) -> &Vec<Event> { &self.events }
}

/// Parse a warcraft log from a string into the log object
impl FromStr for Log {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Log, Self::Err> {
        let mut events = Vec::new();
        for (e, l) in string.lines().enumerate() {
            // TODO: At the moment, errors are only logged. They should instead be
            // made accessible via the API.
            match Event::from_str(l) {
                Ok(event) => events.push(event),
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
            events.len(),
            mem::size_of::<Event>() * events.len() / 1024
        );
        Ok(Log { events })
    }
}

impl<'a> Filterable for &'a Log {
    type Into = FilteredEvents<'a>;

    fn and(self, by: &dyn Filter) -> Self::Into {
        let filtered = FilteredEvents::new(self, bitvec![1; self.events.len()]);
        filtered.and(by)
    }

    fn or(self, by: &dyn Filter) -> Self::Into {
        let filtered = FilteredEvents::new(self, bitvec![0; self.events.len()]);
        filtered.or(by)
    }
}

impl ParseError {
    pub fn new(typ: ParseErrorType, col: usize) -> ParseError { ParseError { typ, col } }

    pub fn typ(&self) -> ParseErrorType { self.typ.clone() }

    pub fn col(&self) -> usize { self.col }
}
