use crate::{Event, Unit};

pub fn damage_dealt<'a, E>(src: &Unit, events: E) -> u64
where
    E: Iterator<Item = &'a Event>
{
    let mut damage = 0;
    for e in events {
        if e.source().as_ref() == Some(src) && e.typ().damaging() {
            damage += e.amount().expect("Damaging event does not have amount");
        }
    }

    damage
}

pub fn healing_done<'a, E>(src: &Unit, events: E) -> u64
where
    E: Iterator<Item = &'a Event>
{
    let mut healing = 0;
    for e in events {
        if e.source().as_ref() == Some(src) && e.typ().healing() {
            healing += e.amount().expect("Healing event does not have amount");
        }
    }

    healing
}
