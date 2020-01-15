use lex::lexer::Lexer;
use lex::token:: {
    Token,
    is_base_type,
};
use crate::parse_def::typeref;
use crate::ast:: {
    TypeNode,
    UnaryNode,
    TermNode,
    PrimaryNode,
    Const,
    RefUnaryNode,
    PointerRefUnaryNode,
    AssginmentNode,
};
use std::rc::Rc;

fn expr0(mut lexer: &mut Lexer) {
    expr1(&mut lexer);
}

fn expr1(mut lexer: &mut Lexer) {
    expr2(&mut lexer);
}

fn expr2(mut lexer: &mut Lexer) {
    expr3(&mut lexer);
}

fn expr3(mut lexer: &mut Lexer) {
    expr4(&mut lexer);
}

fn expr4(mut lexer: &mut Lexer) {
    expr5(&mut lexer);
}

fn expr5(mut lexer: &mut Lexer) {
    expr6(&mut lexer);
}

fn expr6(mut lexer: &mut Lexer) {
    expr1(&mut lexer);
}

fn expr7(mut lexer: &mut Lexer) {
    expr1(&mut lexer);
}

fn expr8(mut lexer: &mut Lexer) {
    expr9(&mut lexer);
}

fn expr9(mut lexer: &mut Lexer) {
    term(&mut lexer);
    if lexer.lookahead(1) == Token::Assgin {
        assignment_expr(&mut lexer);
    }    
}

fn assignment_expr(mut lexer: &mut Lexer) -> AssginmentNode {
    let left_value = term(&mut lexer);
    lexer.advance();
    let right_value = term(&mut lexer);

    AssginmentNode {
        left_value,
        right_value,
    }
}

fn term(mut lexer: &mut Lexer) -> TermNode {
    let mut case_type = None;
    let node;
    if is_base_type(&lexer.lookahead(1)) {
        case_type = Some(typeref(&mut lexer));
    }
    node = Rc::new(unary(&mut lexer));

    TermNode {
        case_type,
        unary: node,
    }
}

fn unary(mut lexer: &mut Lexer) -> Box<dyn UnaryNode> {
    let t = lexer.advance();

    let pn = primary(&mut lexer);

    match lexer.advance() {
        Token::Dot => {
            Box::new(RefUnaryNode {
                prefix: t,
                primary: pn,
                name: lexer.advance(),
            })
        },
        Token::PointerRef => {
            Box::new(RefUnaryNode {
                prefix: t,
                primary: pn,
                name: lexer.advance(),
            })
        },
        _ => panic!("unexcept token!")
        // TODO other type
    }
}

fn primary(mut lexer: &mut Lexer) -> PrimaryNode {
    let t = lexer.advance();
    let name = None;
    match t {
        Token::Number(i) => PrimaryNode {
            name,
            value: Const::Integer(i),
        },
        Token::Character(c) => PrimaryNode {
            name,
            value: Const::Char(c),
        },
        Token::String(s) => PrimaryNode {
            name,
            value: Const::String(s),
        },
        Token::Name(s) => PrimaryNode {
            name: Some(s),
            value: Const::Identifier,
        },
        _ => panic!("unexcept token! {}", t)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assginment_expr() {
        let mut lxr = Lexer::new(String::from("a = 4 + 5"));
        println!("{:?}", assignment_expr(&mut lxr));
    }
}