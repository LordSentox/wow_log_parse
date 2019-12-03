use crate::event::*;
use crate::unit::Unit;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

/// Represents an Encounter.
/// An Encounter starts, when no other Encounter is active and an Event with an
/// enemy is detected. It ends when all enemies pulled in the encounter or all
/// players present are dead.
pub struct Encounter {
    events:   Vec<Event>,
    involved: HashSet<Unit>
}

impl Encounter {
    /// Split a given vector into all encounters contained within.
    pub fn all_encounters(events: Vec<Event>) -> Vec<Encounter> {
        // Records the lives of all hostile units as a tuple of the Unit itself,
        // the index of the first event it attacked by or has attacked a player
        // and the index of the last event recorded with it.
        let mut life_windows: HashMap<Unit, (usize, usize)> = HashMap::new();

        // First, run through the Events in positive direction and find all Units
        // starting lifetimes.
        for (i, e) in events.iter().enumerate() {
            if e.is_hostile() {
                if let (Some(src), Some(tgt)) = (e.source(), e.target()) {
                    if src.is_player() && !tgt.is_player() && !life_windows.contains_key(&tgt) {
                        life_windows.insert(tgt, (i, 0));
                    }
                    else if tgt.is_player()
                        && !src.is_player()
                        && !life_windows.contains_key(&src)
                    {
                        life_windows.insert(src, (i, 0));
                    }
                }
            }
        }

        // Run through the Vec backwards and find the last events that each unit
        // has taken part in. This is considered the end of their life, even when
        // there has not been a UnitDied Event.
        for (i, e) in events.iter().enumerate().rev() {
            if let Some(src) = e.source() {
                if let Some(life) = life_windows.get_mut(&src) {
                    life.1 = max(i, life.1);
                }
            }
            if let Some(tgt) = e.target() {
                if let Some(life) = life_windows.get_mut(&tgt) {
                    life.1 = max(i, life.1);
                }
            }
        }

        // Find the encounters by going through the list and connecting up
        // entity lifetimes that overlap with each other.
        let mut life_windows: Vec<&(usize, usize)> = life_windows.values().collect();
        life_windows.sort();
        let mut current_encounter: (usize, usize) = *life_windows[0]; // TODO: Catch index out of bounds
        let mut encounter_indexes: Vec<(usize, usize)> = Vec::new();
        for (life_start, life_end) in life_windows.iter().skip(1) {
            // The next lifetime starts when the encounter is still running, so
            // check if the encounter may be running longer.
            if life_start <= &current_encounter.1 {
                current_encounter.1 = max(*life_end, current_encounter.1);
            }
            // The next lifetime starts after the encounter has ended, create a
            // new encounter.
            else {
                encounter_indexes.push(current_encounter);
                current_encounter = (*life_start, *life_end);
            }
        }

        // Push the last encounter
        encounter_indexes.push(current_encounter);

        encounter_indexes
            .iter()
            .map(|(start, end)| Encounter::from_events(events[*start..*end + 1].to_vec()))
            .collect()
    }

    pub fn from_events(events: Vec<Event>) -> Encounter {
        let mut involved = HashSet::new();
        for e in &events {
            if let Some(src) = e.source() {
                involved.insert(src);
            };
            if let Some(tgt) = e.target() {
                involved.insert(tgt);
            };
        }

        Encounter { events, involved }
    }

    pub fn involved(&self) -> &HashSet<Unit> { &self.involved }
}
