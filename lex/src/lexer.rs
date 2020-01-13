use crate::token;
use crate::location::Location;

pub struct Lexer {
    chars: String,
    location: Location,
}