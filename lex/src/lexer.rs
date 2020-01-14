use crate::token:: {
    Token,
    get_keywords,
};
use crate::location::Location;
use std::collections::HashMap;
use std::rc::Rc;

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

            match token {
                Some(t) => return t,
                None => {
                    self.cur_text.push(ch);
                },
            }
            
            if self.can_be_identifier() {
                return self.keywords_or_name()
            }
        }

        Token::Eof
    }

    fn whitespace_char(ch: char) -> bool {
        ch == ' ' || ch == '\n' || ch == '\r'
    }

    fn can_be_identifier(&mut self) -> bool {
        let next_char = self.chars[self.read_pos];
        return self.cur_text.len() > 0 && !next_char.is_ascii_alphabetic() && next_char != '_'
    }

    fn handle_valid_char(&mut self, ch: char) -> Option<Token> {
        self.read_pos += 1;
        match ch {
            '(' => Some(Token::LParentheses),
            ')' => Some(Token::RParentheses),
            '[' => Some(Token::LBrackets),
            ']' => Some(Token::RBrackets),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            ';' => Some(Token::Semi),
            '/' => Some(Token::Div),
            '%' => Some(Token::Mod),
            '*' => Some(Token::Mul),
            '+' => Some(self.add_or_inc_token()),
            '-' => Some(self.sub_or_dec_token()),
            '&' => Some(self.and_or_bitand_token()),
            '=' => Some(self.assgin_or_equal_tokean()),
            '"' => Some(self.string_token()),
            '\'' => Some(self.char_token()),
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => Some(self.number_token()),
            _ => None,
        }
    }

    fn string_token(&mut self) -> Token {
        let mut s = Vec::new();
        let mut enclose = false;
        for index in self.read_pos..self.chars.len() {
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

    fn char_token(&mut self) -> Token {
        let next = self.chars[self.read_pos];
        let next2 = self.chars[self.read_pos + 1];
        if next2 != '\'' {
            panic!("Char types can only have a single character");
        }
        self.read_pos += 2;
        return Token::Character(next);
    }

    fn number_token(&mut self) -> Token {
        let mut v = Vec::new();
        for index in self.read_pos..self.chars.len() {
            let ch = self.chars[index];
            println!("di {}", ch);
            if ch.is_ascii_digit() {
                v.push(ch);
            } else {
                if self.chars[index + 1].is_ascii_alphabetic() {
                    panic!("Numeric constants unexpect token,");
                } else {
                    break
                }
            }
        }
        self.read_pos += v.len();
        let num = Lexer::digit_from_vec(v);
        return Token::Number(num as i64)
    }

    fn digit_from_vec(mut vec: Vec<char>) -> u32 {
        let mut num = 0;
        vec.reverse();
        for (i, ch) in vec.iter().enumerate() {
            let index = i as u32;
            let radix: u32 = 10;
            let ci = ch.to_digit(10).unwrap();
            num += ci * radix.pow(index);
        }

        return num
    }

    fn add_or_inc_token(&mut self) -> Token {
        if self.chars[self.read_pos] == '+' {
            self.read_pos += 1;
            return Token::Inc
        } else {
            return Token::Add
        }
    }

    fn sub_or_dec_token(&mut self) -> Token {
        if self.chars[self.read_pos] == '-' {
            self.read_pos += 1;
            return Token::Dec
        } else {
            return Token::Sub
        }
    }

    fn and_or_bitand_token(&mut self) -> Token {
        if self.chars[self.read_pos] == '&' {
            self.read_pos += 1;
            return Token::And
        } else {
            return Token::Bitand
        }
    }

    fn assgin_or_equal_tokean(&mut self) -> Token {
        if self.chars[self.read_pos] == '=' {
            self.read_pos += 1;
            return Token::Equal
        } else {
            return Token::Assgin
        }
    }

    fn keywords_or_name(&mut self) -> Token {
        let s: String = self.cur_text.iter().collect();
        self.cur_text.clear();
        if self.keywords.contains_key(&s) {
            return self.keywords[&s].clone()
        } else {
            return Token::Name(String::from(s))
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