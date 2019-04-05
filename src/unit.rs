#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit {
    id: u64,
    name: String
}

impl Unit {
    pub fn new(id: u64, name: String) -> Unit {
        Unit {
            id,
            name
        }
    }

    /// Convert the raw Strings as found in a log file to a Unit, or None, in
    /// case they do not point to one
    pub fn from_raw<S: AsRef<str>>(id: S, name: S) -> Option<Unit> {
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

    pub fn name(&self) -> &String { &self.name }

    pub fn id(&self) -> u64 { self.id }
}
