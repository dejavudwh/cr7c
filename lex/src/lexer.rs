use crate::token:: {
    Token,
    get_keywords,
};
use crate::location::Location;
use std::collections::HashMap;
use std::process;
use crate::error::LexicalError;

pub struct Lexer {
    chars: Vec<char>,
    location: Location,
    read_pos: usize,
    cur_text: Vec<char>,
    lookahead: Vec<Token>,
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
            lookahead: Vec::new(),
            keywords: get_keywords(),
        }
    }

    fn lex(&mut self) -> Token {
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

    pub fn advance(&mut self) -> Token {
        if self.lookahead.len() <= 0 {
            self.lookahead(1);
        }

        println!("advance {}", self.lookahead(1));
        return self.lookahead.remove(0)
    }

    pub fn lookahead(&mut self, number: usize) -> Token {
        let len = self.lookahead.len();
        if number >= len && self.read_pos < self.chars.len() {
            for i in len..number {
                let t = self.lex();
                self.lookahead.push(t);
            } 
        }
        
        if self.lookahead.len() > 0 {
            return self.lookahead[number - 1].clone()
        } else {
            return Token::Eof
        }
    }

    pub fn matcher(&mut self, token: Token) -> Token {
        self.lookahead(1);
        if self.lookahead[0] != token {
            panic!("unexcept token!: {} in {}", self.lookahead[0], self.location);
        };

        self.advance()
    }

    fn whitespace_char(ch: char) -> bool {
        ch == ' ' || ch == '\n' || ch == '\r'
    }

    fn can_be_identifier(&mut self) -> bool {
        let next_char = self.chars[self.read_pos];
        let not_null = self.cur_text.len() > 0;
        let next_invalid = !(next_char.is_ascii_alphabetic()) && !(next_char.is_ascii_digit());
        return  not_null && next_invalid && next_char != '_' || next_char == '.'
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
            '.' => Some(Token::Dot),
            ',' => Some(Token::Comma),
            '^' => Some(Token::Bitxor),
            '|' => Some(self.or_or_bitor_token()),
            '>' => Some(self.greater_or_shift_token()),
            '<' => Some(self.less_or_shift_token()),
            '!' => Some(self.not_or_equal_token()),
            '+' => Some(self.add_or_inc_token()),
            '-' => Some(self.sub_or_dec_token()),
            '&' => Some(self.and_or_bitand_token()),
            '=' => Some(self.assgin_or_equal_tokean()),
            '"' => Some(self.string_token()),
            '\'' => Some(self.char_token()),
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                println!(" == = = = = === = == = {} {}", ch, self.cur_text.len());
                if self.cur_text.len() == 0 {
                    return Some(self.number_token())
                } else {
                    return None
                }
            },
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
        for index in self.read_pos - 1..self.chars.len() {
            let ch = self.chars[index];
            if ch.is_ascii_digit() {
                v.push(ch);
            } else {
                if index + 1 < self.chars.len() {
                    if self.chars[index + 1].is_ascii_alphabetic() {
                        panic!("Numeric constants unexpect token,");
                    } else {
                        break
                    }
                }
            }
        }
        self.read_pos += v.len() - 1;
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

    fn greater_or_shift_token(&mut self) -> Token {
        let ch = self.chars[self.read_pos]; 
        if ch == '>' {
            self.read_pos += 1;
            return Token::Rightshift
        } else if ch == '=' {
            self.read_pos += 1;
            return Token::Greaterequal
        } else {
            return Token::Greater
        }
    }

    fn less_or_shift_token(&mut self) -> Token {
        let ch = self.chars[self.read_pos]; 
        if ch == '<' {
            self.read_pos += 1;
            return Token::Leftshift
        } else if ch == '=' {
            self.read_pos += 1;
            return Token::Lessequal
        } else {
            return Token::Less
        }
    }

    fn not_or_equal_token(&mut self) -> Token {
        let ch = self.chars[self.read_pos]; 
        if ch == '=' {
            self.read_pos += 1;
            return Token::Notequal
        } else {
            return Token::Not
        }
    }

    fn or_or_bitor_token(&mut self) -> Token {
        let ch = self.chars[self.read_pos]; 
        if ch == '|' {
            self.read_pos += 1;
            return Token::Or
        } else {
            return Token::Bitor
        }
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
        } else if self.chars[self.read_pos] == '>' {
            self.read_pos += 1;
            return Token::PointerRef
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

    #[test]
    fn test_lexer() {
        let content = String::from("int main() {
            struct st {
                int a;
                double b;
                float c;
            }
        
            if (1 == 1) {
                int a.v = 123425435;
                char b = 'a';
            }
        
            [ ] \"asd asd \" () && & -- ++ + -
        }");

        let mut lexer = Lexer::new(content);

        println!("lookahead 4 {}", lexer.lookahead(4));
        println!("lookahead 6 {}", lexer.lookahead(6));
        println!("lookahead 8 {}", lexer.lookahead(8));

        for i in 0..40 {
            println!("{}", lexer.advance());
        }
    }
}