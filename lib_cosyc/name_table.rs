use std::collections::HashMap;

/// A struct which maps string slices to identifier keys.
/// Identifier keys are not hashes of the slice, and are,
/// in fact, enumerations.
pub struct NameTable<'a> {
    current_key : Identifier,
    name_table : HashMap<&'a str, Identifier>
}
impl<'a> NameTable<'a> {
    /// Creates an empty name table.
    pub fn new() -> Self {
        Self {
            current_key : 0,
            name_table : HashMap::new()
        }
    }

    /// Inserts an identifier into the name table and
    /// returns its key.
    pub fn insert(&mut self, ident : &'a str) -> Identifier {
        let key = self.current_key;
        self.current_key += 1;
        self.name_table.insert(ident, key);
        key
    }

    /// Attempts to find the identifier key for this name.
    /// # Errors
    /// Returns `None` if the identifier does not exist.
    pub fn get(&mut self, ident : &'a str) -> Option<Identifier> {
        Some(self.name_table.get(ident)?.to_owned())
    }
}

/// A type alias which matches an identifier name.
pub type Identifier = usize;