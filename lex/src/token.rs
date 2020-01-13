use std::collections::HashMap;

pub enum Token {
    // type
    Int,
    Float,
    Double,
    Char,

    // keywords
    While,
    For,
    If,
    Else,
    Do,
    Break,
    Continue,
    Return,
    Switch,
    Case,
    Default,

    Struct,

    // operator
    Add,
    Sub,
    Div,
    Mod,
    Inc,
    Dec,
    Or,
    Bitor,
    And,
    Bitand,
    Equal,
    assgin,
    Pointer,

    // literal
    Number {
        value: f64
    },
    String {
        value: String,
    },
    Character {
        value: u32,
    },
    Name {
        value: String,
    },

    // separator
    LParentheses,
    RParentheses,
    LBrace,
    RBrace,
    LBrackets,
    RBrackets,
    Quotes,
    Semi,
}

pub fn get_keyword() -> HashMap<String, Token> {
    let mut map = HashMap::new();

    map.insert(String::from("while"), Token::While);
    map.insert(String::from("for"), Token::For);
    map.insert(String::from("if"), Token::If);
    map.insert(String::from("else"), Token::Else);
    map.insert(String::from("do"), Token::Do);
    map.insert(String::from("break"), Token::Break);
    map.insert(String::from("continue"), Token::Continue);
    map.insert(String::from("return"), Token::Return);
    map.insert(String::from("switch"), Token::Switch);
    map.insert(String::from("case"), Token::Case);
    map.insert(String::from("default"), Token::Default);

    map
}
