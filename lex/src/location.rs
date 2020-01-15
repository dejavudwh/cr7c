use std::fmt;

#[derive(Clone)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Location {
    pub fn new(row: usize, column: usize) -> Location {
        Location {
            row,
            column,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.row, self.column)
    }
}