use crate::token:: {
    Token,
    get_keywords,
};
use crate::location::Location;
use std::collections::HashMap;

pub struct Lexer {
    chars: Vec<char>,
    location: Location,
    read_pos: usize,
    cur_text: Vec<char>,
    lookahead: Option<Token>,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut chars = Vec::new();
        for c in input.chars() {
            chars.push(c)
        }
        
        Lexer {
            chars: chars,
            location: Location::new(0, 0),
            read_pos: 0,
            cur_text: Vec::new(),
            lookahead: None,
            keywords: get_keywords(),
        }
    }

    pub fn lex(&mut self) -> Token {
        let start = self.read_pos;
        for index in start..self.chars.len() {
            let ch = self.chars[index];
            if Lexer::whitespace_char(ch) {
                self.read_pos += 1;
                continue
            } 

            let token = self.handle_valid_char(ch);
            self.read_pos += 1;

            match token {
                Some(t) => return t,
                None => {
                    self.cur_text.push(ch);
                },
            }
            
        }

        Token::Eof
    }

    fn whitespace_char(ch: char) -> bool {
        ch == ' ' || ch == '\n' || ch == '\r'
    }

    fn handle_valid_char(&mut self, ch: char) -> Option<Token> {
        match ch {
            '(' => Some(Token::LParentheses),
            ')' => Some(Token::RParentheses),
            '[' => Some(Token::LBrackets),
            ']' => Some(Token::RBrackets),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            ';' => Some(Token::Semi),
            '"' => Some(self.string_token()),
            _ => None,
        }
    }

    fn string_token(&mut self) -> Token {
        let mut s = Vec::new();
        let mut enclose = false;
        for index in self.read_pos + 1..self.chars.len() {
            let ch = self.chars[index];
            match ch {
                '\n' => panic!("The quotes do not match correctly in {} ", self.location),
                '"' => {
                    enclose = true;
                    break;
                },
                _ => {
                    s.push(ch);
                },
            }
        }

        if enclose {
            let sv: String = s.iter().collect();
            self.read_pos += sv.len() + 1;
            return Token::String(String::from(sv))
        } else {
            panic!("The quotes are not closed");
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_char() {
        assert!(Lexer::whitespace_char(' '));
        assert!(Lexer::whitespace_char('\n'));
        assert!(Lexer::whitespace_char('\r'));
    }
}