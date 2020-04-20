use crate::{Event, Unit};

pub fn damage_dealt<E>(src: &Unit, events: E) -> u64
where
    E: Iterator<Item = Event>
{
    let mut damage = 0;
    for e in events {
        if e.source().as_ref() == Some(src) && e.typ().damaging() {
            damage += e.amount().expect("Damaging event does not have amount");
        }
    }

    damage
}
