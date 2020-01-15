use crate::location::Location;
use crate::token::Token;
use std::fmt;

pub struct LexicalError {
    pub location: Location,
    pub token: Token,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexcept token! {} in {}", self.token, self.location)
    }
}