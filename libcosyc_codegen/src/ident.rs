use std::collections::HashMap;

/// A type alias which matches an identifier name.
pub type Identifier = usize;

/// A struct which maps string slices to identifier ids.
/// Identifier ids are just arbitrary enumerations.
#[derive(Default)]
pub struct NameTable<'a> {
    current_key : Identifier,
    table : HashMap<&'a str, Identifier>
}
impl<'a> NameTable<'a> {
    /// Creates an empty name table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an identifier into the name table and returns its id.
    /// Behaves exactly the same as `get` if the identifier already exists.
    pub fn fetch(&mut self, ident : &'a str) -> Identifier {
        if let Some(value) = self.table.get(ident) {
            *value
        } else {
            let value = self.current_key;
            self.current_key += 1;
            self.table.insert(ident, value);
            value
        }
    }
}
