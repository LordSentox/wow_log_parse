#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit {
    id: u64,
    name: String
}

impl Unit {
    /// Create a new Unit
    pub fn new(id: u64, name: String) -> Unit {
        Unit {
            id,
            name
        }
    }

    /// Convert the raw Strings as found in a log file to a Unit, or None, in
    /// case they do not point to one
    pub fn from_raw<S: AsRef<str>>(id: S, name: S) -> Option<Unit> {
        // Check for non-hex-coded ids
        if !id.as_ref().starts_with("0x") {
            warn!("Invalid Unit id detected: {}", id.as_ref());
            None
        }
        else {
            let id = match u64::from_str_radix(id.as_ref().split_at(2).1, 16) {
                Ok(id) => id,
                Err(err) => { 
                    error!("Error parsing id {}", err);
                    return None;
                }
            };
            if id == 0 || name.as_ref() == "nil" { return None; }

            let name = name.as_ref().trim_matches('\"').to_string();

            Some(Unit { id, name })
        }
    }

    /// Check, if this Unit represents a Player, or something else. Returns true
    /// if it is a Player
    pub fn is_player(&self) -> bool {
        // TODO: Still don't know if this is correct. It seems, that Players are
        // at least on the lower id spectrum, while other entities are at the
        // higher spectrum
        self.id <= u32::max_value() as u64
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn id(&self) -> u64 { self.id }
}
