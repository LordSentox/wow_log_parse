use crate::Event;

pub trait Filter {
    /// Checks whether or not an event passes this filter. `true` if the event
    /// should be included when passing through this filter, `false` if it
    /// should be excluded.
    fn check(&self, event: &Event) -> bool;
}

pub trait Filterable {
    type Into;

    fn and(self, by: &dyn Filter) -> Self::Into;

    fn or(self, by: &dyn Filter) -> Self::Into;
}
