use std::collections::HashMap;

/// A struct which maps string slices to identifier ids.
/// Identifier ids are just arbitrary enumerations.
pub struct NameTable<'a> {
    current_key : Identifier,
    name_table : HashMap<&'a str, Identifier>,
    key_table : HashMap<Identifier, &'a str>
}
impl<'a> NameTable<'a> {
    /// Creates an empty name table.
    pub fn new() -> Self {
        Self {
            current_key : 0,
            name_table : HashMap::new(),
            key_table : HashMap::new()
        }
    }

    /// Inserts an identifier into the name table and returns its id.
    /// Behaves exactly the same as `get` if the identifier already exists.
    pub fn add_name(&mut self, ident : &'a str) -> Identifier {
        let key = self.current_key;
        self.current_key += 1;
        self.name_table.insert(ident, key);
        self.key_table.insert(key, ident);
        key
    }

    /// Attempts to find the id of the identifier with this name.
    /// # Errors
    /// Returns `None` if the identifier does not exist.
    pub fn get_id(&mut self, ident : &'a str) -> Option<Identifier> {
        Some(*self.name_table.get(ident)?)
    }

    /// Attempts to find the name of an identifier with this id.
    /// # Errors
    /// Returns `None` if an identifier with this id does not exist.
    pub fn get_name(&mut self, id : &Identifier) -> Option<&'a str> {
        Some(*self.key_table.get(id)?)
    }
}

/// A type alias which matches an identifier name.
pub type Identifier = usize;