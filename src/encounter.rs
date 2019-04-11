use crate::event::*;
use crate::unit::Unit;

use std::collections::HashSet;

/// Represents an Encounter.
/// An Encounter starts, when no other Encounter is active and an Event with an
/// enemy is detected. It ends when all enemies pulled in the encounter or all
/// players present are dead.
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
        let mut alive_friendlies = HashSet::new();
        let mut alive_hostiles = HashSet::new();
        let mut involved = HashSet::new();
        let mut started = false;

        while let Some(event) = all_events.next() {
            if started { enc_events.push(event.clone()); }

            // Add new entities to the encounter or start the encounter.
            if event.is_hostile() {
                if let Some(src) = event.source() {
                    involved.insert(src.clone());
                    if src.hostile() { alive_hostiles.insert(src); }
                    else { alive_friendlies.insert(src); }
                }
                if let Some(tgt) = event.target() {
                    involved.insert(tgt.clone());
                    if tgt.hostile() { alive_hostiles.insert(tgt); }
                    else { alive_friendlies.insert(tgt); }
                }

                started = true;
            }
            // Remove dead entities from the encounter and end the encounter if
            // either all friendlies or all enemies are dead.
            if event.typ() == EventType::UnitDied {
                alive_hostiles.remove(&event.target().unwrap());
                alive_friendlies.remove(&event.target().unwrap());
            }

            if started {
                // Check if there are no friendlies or no hostiles left
                if alive_friendlies.is_empty() || alive_hostiles.is_empty() {
                    break; // The encounter is over
                }

                info!("Alive hostiles: {:?}", alive_hostiles);
            }
        }

        if started {
            Some(Encounter {
                events: enc_events,
                involved
            })
        }
        else { None }
    }

    pub fn involved(&self) -> &HashSet<Unit> { &self.involved }
}
