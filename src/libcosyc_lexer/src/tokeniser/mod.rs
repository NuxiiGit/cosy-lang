mod reader;

use reader::{ CharKind, StringReader };

/// A struct which takes individual characters of a lex and converts them into larger `CharClump`s.
pub struct Tokeniser<'a> {
    reader : StringReader<'a>
}
impl<'a> Tokeniser<'a> {
    /// Creates a new tokeniser from this string slice.
    pub fn from(src : &'a str) -> Self {
        Self {
            reader : StringReader::from(src)
        }
    }
}

/// An enum which represents larger collections of characters, e.g. identifiers.
#[derive(PartialEq, Debug, Clone)]
pub enum CharClump {

}