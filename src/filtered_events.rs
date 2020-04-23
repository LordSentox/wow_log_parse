use crate::filter::{Filter, Filterable};
use crate::log::Log;
use bitvec::vec::BitVec;

pub struct FilteredEvents<'a> {
    /// The log this filtered list of events is based upon
    log:     &'a Log,
    /// All the positions that are currently included in the filtered events are
    /// marked with a top bit, the others are marked with a bottom bit.
    include: BitVec
}

impl<'a> FilteredEvents<'a> {
    pub fn new(log: &'a Log, include: BitVec) -> Self { Self { log, include } }
}

impl<'a> Filterable for FilteredEvents<'a> {
    type Into = Self;

    fn and(mut self, by: &dyn Filter) -> Self::Into {
        for (i, event) in self.log.events().iter().enumerate() {
            // If the check of the filter fails, the bit in the including filter must be set
            // to false, regardless of what it was before. include[i] is checked,
            // because it is usually much faster than the filters check.
            if self.include[i] && !by.check(&event) {
                self.include.set(i, false);
            }
        }

        self
    }

    fn or(mut self, by: &dyn Filter) -> Self::Into {
        for (i, event) in self.log.events().iter().enumerate() {
            // If the check of the filter succeeds, the bit in the including filter must be
            // set to true, regardless of what it was before. include[i] is
            // checked, because it is usually much faster than the filters
            // check.
            if !self.include[i] && by.check(&event) {
                self.include.set(i, true);
            }
        }

        self
    }
}
