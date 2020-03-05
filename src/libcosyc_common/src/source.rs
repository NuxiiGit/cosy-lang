/// A struct which stores information about some substring of a source file.
#[derive(Debug, Clone)]
pub struct Context<'a, T : Clone> {
    pub src_pos : SourcePos<'a>,
    pub value : T
}

/// Stores the source and position 
#[derive(Debug, Clone)]
pub struct SourcePos<'a> {
    pub src : &'a str,
    pub line : usize,
}