use crate::event::Event;
use crate::unit::Unit;

/// Represents an Encounter.
/// An Encounter starts, when no other Encounter is active and an Event with an
/// enemy is detected. It ends when all enemies pulled during the encounter are
/// dead.
pub struct Encounter {
    events: Vec<Event>,
    involved: Vec<Unit>
}

impl Encounter {
    /// Split a given vector into all encounters contained within.
    pub fn all_encounters(events: &mut Iterator<Item=Event>) -> Vec<Encounter> {
        let mut encounters = Vec::new();

        while let Some(e) = Encounter::next_encounter(events) {
            encounters.push(e);
        }

        encounters
    }

    /// Continue reading from the iterator, until the next Encounter has been
    /// read and return it, or None if no Encounter was detected.
    pub fn next_encounter(events: &mut Iterator<Item=Event>) -> Option<Encounter> {
        

        None
    }
}
