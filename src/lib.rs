#[macro_use]
extern crate log;

pub mod encounter;
pub mod event;
pub mod extract;
pub mod math;
pub mod parser;
pub mod unit;

pub use encounter::*;
pub use event::*;
pub use math::*;
pub use parser::*;
pub use unit::*;

#[cfg(test)]
extern crate env_logger;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;
    use std::sync::Once;

    static ENV_LOG_INIT: Once = Once::new();
    fn env_init() {
        ENV_LOG_INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn load_event() {
        env_init();

        Event::from_str("3/9 19:05:22.252  SPELL_CAST_SUCCESS,0x000000000014EABC,\"Draleofdeath\",0x512,0x000000000014EABC,\"Draleofdeath\",0x512,25899,\"Greater Blessing of Sanctuary\",0x2").expect("Unable to parse event");
    }

    #[test]
    fn load_file() {
        env_init();

        let mut file = File::open("logs/utgarde_keep.txt").expect("Unable to open file");
        let mut log = String::new();
        file.read_to_string(&mut log).expect("Unable to read file");

        let events = parse(&log);

        info!("Involved units:");
        let mut units: HashSet<Unit> = HashSet::new();
        for e in events {
            if let Some(src) = e.source() {
                units.insert(src);
            }
            if let Some(tar) = e.target() {
                units.insert(tar);
            }
        }

        for u in units {
            info!("[{:x}, \"{}\"]", u.id(), u.name());
        }
    }

    #[test]
    fn load_file_encounters() {
        env_init();

        let mut file = File::open("logs/halls_of_lightning.txt").expect("Unable to open file");
        let mut log = String::new();
        file.read_to_string(&mut log).expect("Unable to read file");

        let events = parse(&log);

        let encounters = Encounter::all_encounters(events);
        info!("Number of encounters: {}", encounters.len());

        for e in &encounters {
            info!("{} units involved in an encounter.", e.involved().len());
            for u in e.involved() {
                info!("[{:x}, \"{}\"]", u.id(), u.name());
            }
        }

        assert_eq!(encounters.len(), 28);
    }

    #[test]
    fn damage_dealt() {
        env_init();

        let mut file = File::open("logs/utgarde_keep.txt").expect("Unable to open file");
        let mut log = String::new();
        file.read_to_string(&mut log).expect("Unable to read file");

        let events = parse(&log);

        info!("Involved units:");
        let mut units: HashSet<Unit> = HashSet::new();
        for e in &events {
            if let Some(src) = e.source() {
                units.insert(src);
            }
            if let Some(tar) = e.target() {
                units.insert(tar);
            }
        }

        for u in units {
            info!(
                "[{:x}, \"{}\"] has dealt {} damage in total",
                u.id(),
                u.name(),
                extract::damage_dealt(&u, events.iter())
            );
        }

        // Check that the player damage amounts are in order
        assert_eq!(
            772_442,
            extract::damage_dealt(&Unit::new(0x137e20, "Telta".into()), events.iter())
        );
        assert_eq!(
            32_885,
            extract::damage_dealt(&Unit::new(0x12dc52, "Erle".into()), events.iter())
        );
        assert_eq!(
            693_396,
            extract::damage_dealt(&Unit::new(0x160f5b, "Histera".into()), events.iter())
        );
        assert_eq!(
            1_624_123,
            extract::damage_dealt(&Unit::new(0x13b13c, "Nundo".into()), events.iter())
        );
        assert_eq!(
            1_323_749,
            extract::damage_dealt(&Unit::new(0x117351, "Ironmate".into()), events.iter())
        );
    }

    #[test]
    fn healing_done() {
        env_init();

        let mut file = File::open("logs/utgarde_keep.txt").expect("Unable to open file");
        let mut log = String::new();
        file.read_to_string(&mut log).expect("Unable to read file");

        let events = parse(&log);

        info!("Involved units:");
        let mut units: HashSet<Unit> = HashSet::new();
        for e in &events {
            if let Some(src) = e.source() {
                units.insert(src);
            }
            if let Some(tar) = e.target() {
                units.insert(tar);
            }
        }

        for u in units {
            info!(
                "[{:x}, \"{}\"] has done {} healing in total",
                u.id(),
                u.name(),
                extract::healing_done(&u, events.iter())
            );
        }

        // Check that the player damage amounts are in order
        assert_eq!(
            0,
            extract::healing_done(&Unit::new(0x137e20, "Telta".into()), events.iter())
        );
        assert_eq!(
            3_622_665,
            extract::healing_done(&Unit::new(0x12dc52, "Erle".into()), events.iter())
        );
        assert_eq!(
            202_990,
            extract::healing_done(&Unit::new(0x160f5b, "Histera".into()), events.iter())
        );
        assert_eq!(
            0,
            extract::healing_done(&Unit::new(0x13b13c, "Nundo".into()), events.iter())
        );
        assert_eq!(
            14_750,
            extract::healing_done(&Unit::new(0x117351, "Ironmate".into()), events.iter())
        );
    }
}
