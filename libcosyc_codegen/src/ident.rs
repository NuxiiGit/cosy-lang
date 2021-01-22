use std::{ fmt, collections::HashMap };

/// Represents a mangled ID, including the number of times this ID has been seen.
#[derive(Debug)]
pub struct MangledId {
    id : String,
    count : usize
}

impl fmt::Display for MangledId {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.id)?;
        if self.count > 0 {
            write!(out, "{}", self.count + 1)?;
        }
        Ok(())
    }
}

/// Keeps track of the number of times an identifier has been seen. Returns a unique mangled ID.
#[derive(Default, Debug)]
pub struct NameTable {
    ids : HashMap<String, MangledId>
}

impl NameTable {
    /// Creates an empty `NameTable`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current `MangledId` for this name.
    /// # Errors
    /// Returns `None` if `name` has never been seen.
    pub fn get(&mut self, name : &str) -> Option<&MangledId> {
        self.ids.get(name)
    }

    /// Increments the count of this name and returns its mangled id.
    pub fn set(&mut self, name : &str) -> &MangledId {
        unimplemented!()
    }

    /// Decrements the count of this name.
    pub fn unset(&mut self, name : &str) {
        unimplemented!()
    }

    /*
    /// Inserts an identifier into the name table and returns its id.
    pub fn add(&mut self, name : &str) -> Id {
        if let Some(id) = self.to.get(name) {
            *id
        } else {
            let id = self.current_key;
            self.current_key += 1;
            self.to.insert(name.to_string(), id);
            self.from.push(name.to_string());
            id
        }
    }

    /// Gets an id from an identifier name.
    pub fn get_id(&self, name : &str) -> Option<Id> {
        Some(*self.to.get(name)?)
    }

    /// Gets an identifier name from an id.
    pub fn get_name(&self, id : Id) -> Option<&str> {
        Some(self.from.get(id)? as &str)
    }*/
}
