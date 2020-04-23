#[macro_use]
extern crate log as logger;

pub mod encounter;
pub mod event;
pub mod extract;
pub mod filter;
pub mod filtered_events;
pub mod log;
pub mod math;
pub mod unit;

pub use crate::log::*;
pub use encounter::*;
pub use event::*;
pub use filter::*;
pub use filtered_events::*;
pub use math::*;
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

        let log = Log::read_file("logs/utgarde_keep.txt").expect("Unable to read log");

        info!("Involved units:");
        let mut units: HashSet<Unit> = HashSet::new();
        for e in log.events() {
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

        let log = Log::read_file("logs/halls_of_lightning.txt").expect("Unable to read log");

        let encounters = Encounter::all_encounters(log.events().clone());
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

        let log = Log::read_file("logs/utgarde_keep.txt").expect("unable to read log");

        info!("Involved units:");
        let mut units: HashSet<Unit> = HashSet::new();
        for e in log.events() {
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
                extract::damage_dealt(&u, log.events().iter())
            );
        }

        // Check that the player damage amounts are in order
        assert_eq!(
            772_442,
            extract::damage_dealt(&Unit::new(0x137e20, "Telta".into()), log.events().iter())
        );
        assert_eq!(
            32_885,
            extract::damage_dealt(&Unit::new(0x12dc52, "Erle".into()), log.events().iter())
        );
        assert_eq!(
            693_396,
            extract::damage_dealt(&Unit::new(0x160f5b, "Histera".into()), log.events().iter())
        );
        assert_eq!(
            1_624_123,
            extract::damage_dealt(&Unit::new(0x13b13c, "Nundo".into()), log.events().iter())
        );
        assert_eq!(
            1_323_749,
            extract::damage_dealt(&Unit::new(0x117351, "Ironmate".into()), log.events().iter())
        );
    }

    #[test]
    fn healing_done() {
        env_init();

        let log = Log::read_file("logs/utgarde_keep.txt").expect("Unable to read log");

        info!("Involved units:");
        let mut units: HashSet<Unit> = HashSet::new();
        for e in log.events() {
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
                extract::healing_done(&u, log.events().iter())
            );
        }

        // Check that the player damage amounts are in order
        assert_eq!(
            0,
            extract::healing_done(&Unit::new(0x137e20, "Telta".into()), log.events().iter())
        );
        assert_eq!(
            3_622_665,
            extract::healing_done(&Unit::new(0x12dc52, "Erle".into()), log.events().iter())
        );
        assert_eq!(
            202_990,
            extract::healing_done(&Unit::new(0x160f5b, "Histera".into()), log.events().iter())
        );
        assert_eq!(
            0,
            extract::healing_done(&Unit::new(0x13b13c, "Nundo".into()), log.events().iter())
        );
        assert_eq!(
            14_750,
            extract::healing_done(&Unit::new(0x117351, "Ironmate".into()), log.events().iter())
        );
    }
}
