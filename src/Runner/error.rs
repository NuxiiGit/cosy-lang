/// A struct which holds error information.
pub struct Error<'a> {
    message : &'a str,
    row : usize,
    column : usize
}
impl<'a> Error<'a> {
    /// Construct a new error instance.
    pub fn new(message : &'a str, row : usize, column : usize) -> Error<'a> {
        Error {
            message : message,
            row : row,
            column : column
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &'a str {
        self.message
    }

    /// Returns the row number the error occured on.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column number the error occured on.
    pub fn column(&self) -> usize {
        self.column
    }
}