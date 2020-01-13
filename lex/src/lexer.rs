use crate::token:: {
    Token,
    get_keywords,
};
use crate::location::Location;
use std::collections::HashMap;

pub struct Lexer {
    chars: String,
    location: Location,
    lookahead: Option<Token>,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            chars: input,
            location: Location::new(0, 0),
            lookahead: None,
            keywords: get_keywords(),
        }
    }

    // pub fn lex() -> Token {
        
    // }
}