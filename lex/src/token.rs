use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // type
    Int,
    Float,
    Double,
    Char,
    Void,
    Long,
    Short,

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
    Import,

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
    Mul,
    Equal,
    Assgin,
    Pointer,
    Dot,

    // literal
    Number(i64),
    String(String),
    Character(char),
    Name(String),

    // separator
    LParentheses,
    RParentheses,
    LBrace,
    RBrace,
    LBrackets,
    RBrackets,
    Quotes,
    Semi,

    Eof,
}

pub fn get_keywords() -> HashMap<String, Token> {
    let mut map = HashMap::new();

    map.insert(String::from("int"), Token::Int);
    map.insert(String::from("float"), Token::Float);
    map.insert(String::from("double"), Token::Double);
    map.insert(String::from("char"), Token::Char);
    map.insert(String::from("void"), Token::Void);
    map.insert(String::from("long"), Token::Long);
    map.insert(String::from("short"), Token::Short);
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
    map.insert(String::from("import"), Token::Import);

    map.insert(String::from("struct"), Token::Struct);

    map
}

pub fn is_base_type(token: &Token) -> bool {
    *token == Token::Char   ||
    *token == Token::Int    ||
    *token == Token::Double ||
    *token == Token::Float  ||
    *token == Token::Long   ||
    *token == Token::Short  ||
    *token == Token::Void   ||
    *token == Token::Struct
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Int => write!(f, "Token: <Int>"),
            Token::Float => write!(f, "Token: <Float>"),
            Token::Double => write!(f, "Token: <Double>"),
            Token::Char => write!(f, "Token: <Char>"),
            Token::Long => write!(f, "Token: <Long>"),
            Token::Short => write!(f, "Token: <Short>"),
            Token::Void => write!(f, "Token: <Void>"),
            Token::While => write!(f, "Token: <While>"),
            Token::For => write!(f, "Token: <For>"),
            Token::If => write!(f, "Token: <If>"),
            Token::Else => write!(f, "Token: <Else>"),
            Token::Do => write!(f, "Token: <Do>"),
            Token::Break => write!(f, "Token: <Break>"),
            Token::Continue => write!(f, "Token: <Continue>"),
            Token::Return => write!(f, "Token: <Return>"),
            Token::Switch => write!(f, "Token: <Switch>"),
            Token::Case => write!(f, "Token: <Case>"),
            Token::Default => write!(f, "Token: <Default>"),
            Token::Import => write!(f, "Token: <Import>"),
            Token::Struct => write!(f, "Token: <Struct>"),
            Token::Add => write!(f, "Token: <Add>"),
            Token::Sub => write!(f, "Token: <Sub>"),
            Token::Div => write!(f, "Token: <Div>"),
            Token::Mod => write!(f, "Token: <Mod>"),
            Token::Inc => write!(f, "Token: <Inc>"),
            Token::Dec => write!(f, "Token: <Dec>"),
            Token::Or => write!(f, "Token: <Or>"),
            Token::Dot => write!(f, "Token: <Dot>"),
            Token::Bitor => write!(f, "Token: <Bitor>"),
            Token::And => write!(f, "Token: <And>"),
            Token::Bitand => write!(f, "Token: <Bitand>"),
            Token::Mul => write!(f, "Token: <Mul>"),
            Token::Equal => write!(f, "Token: <Equal>"),
            Token::Assgin => write!(f, "Token: <Assgin>"),
            Token::Pointer => write!(f, "Token: <Pointer>"),
            Token::Number(i) => write!(f, "Token: <Number : {}>", i),
            Token::String(s) => write!(f, "Token: <String : \"{}\">", s),
            Token::Character(u) => write!(f, "Token: <Character : {}>", u),
            Token::Name(s) => write!(f, "Token: <Name : {}>", s),
            Token::LParentheses => write!(f, "Token: <LParentheses>"),
            Token::RParentheses => write!(f, "Token: <RParentheses>"),
            Token::LBrace => write!(f, "Token: <LBrace>"),
            Token::RBrace => write!(f, "Token: <RBrace>"),
            Token::LBrackets => write!(f, "Token: <LBrackets>"),
            Token::RBrackets => write!(f, "Token: <RBrackets>"),
            Token::Quotes => write!(f, "Token: <Quotes>"),
            Token::Semi => write!(f, "Token: <Semi>"),
            Token::Eof => write!(f, "Token: <Eof>"),
        }
    }
}