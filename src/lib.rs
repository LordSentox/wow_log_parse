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
}
