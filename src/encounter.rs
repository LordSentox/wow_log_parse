use crate::event::Event;
use crate::unit::Unit;

use std::collections::HashSet;

/// Represents an Encounter.
/// An Encounter starts, when no other Encounter is active and an Event with an
/// enemy is detected. It ends when all enemies pulled during the encounter are
/// dead.
pub struct Encounter {
    events: Vec<Event>,
    involved: HashSet<Unit>
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
    pub fn next_encounter(all_events: &mut Iterator<Item=Event>) -> Option<Encounter> {
        let mut enc_events = Vec::new();

        // Once an encounter starts, when one of these Vectors is empty, the
        // encounter stops
        let mut friendlies = HashSet::new();
        let mut hostiles = HashSet::new();
        let mut started = false;

        while let Some(event) = all_events.next() {
            if started { enc_events.push(event); }

            let typ = event.typ();
            if typ.is_hostile() {
                // No two friendly units may be involved in a hostile event
                if let (Some(src), Some(tgt)) = (event.source(), event.target()) {
                    assert!(src.is_player() != tgt.is_player());
                }
                if event.source().is_none() || event.target().is_none() { continue; }
            }
        }

        if !started { None }
    }
}
