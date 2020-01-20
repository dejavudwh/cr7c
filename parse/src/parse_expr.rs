use lex::lexer::Lexer;
use lex::token:: {
    Token,
    is_base_type,
    is_prefix_op,
    is_postfix_op,
};
use crate::parse_def::typeref;
use crate::ast:: {
    UnaryNode,
    TermNode,
    PrimaryNode,
    Const,
    RefUnaryNode,
    PointerRefUnaryNode,
    ExprNode,
    AssginmentNode,
    SingeUnaryNode,
    SelfOpUnaryNode,
    MultiplicationNode,
    DivisionNode,
    ModNode,
    AddNode,
    SubNode,
    RightShiftNode,
    LeftShiftNode,
    BitAndNode,
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

fn expr6(mut lexer: &mut Lexer) -> Box<dyn ExprNode> {
    let mut left_value = expr7(&mut lexer);
    loop {
        let t = lexer.lookahead(1);
        match t {
            Token::Bitand => {
                left_value = bit_and_expr(&mut lexer, left_value);
            },
            _ => {
                return left_value
            }
        }
    }
}

fn bit_and_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = expr7(&mut lexer);
    Box::new(BitAndNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}
    
fn expr7(mut lexer: &mut Lexer) -> Box<dyn ExprNode> {
    let mut left_value = expr8(&mut lexer);
    loop {
        let t = lexer.lookahead(1);
        match t {
            Token::Rightshift => {
                left_value = right_shift_expr(&mut lexer, left_value);
            },
            Token::Leftshift => {
                left_value = left_shift_expr(&mut lexer, left_value);
            },
            _ => {
                return left_value
            }
        }
    }
}

fn right_shift_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = expr8(&mut lexer);
    Box::new(RightShiftNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn left_shift_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = expr8(&mut lexer);
    Box::new(LeftShiftNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn expr8(mut lexer: &mut Lexer) -> Box<dyn ExprNode> {
    let mut left_value = expr9(&mut lexer);
    loop {
        let t = lexer.lookahead(1);
        match t {
            Token::Add => {
                left_value = add_expr(&mut lexer, left_value);
            },
            Token::Sub => {
                left_value = sub_expr(&mut lexer, left_value);
            },
            _ => {
                return left_value
            }
        }
    }
}

fn add_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = expr9(&mut lexer);
    Box::new(AddNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn sub_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = expr9(&mut lexer);
    Box::new(SubNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn expr9(mut lexer: &mut Lexer) -> Box<dyn ExprNode> {
    let mut left_value = term(&mut lexer);
    loop {
        let t = lexer.lookahead(1);
        match t {
            Token::Assgin => {
                let right_value = assignment_expr(&mut lexer);
    
                return Box::new(AssginmentNode {
                    left_value: Rc::new(left_value),
                    right_value: Rc::new(right_value),
                })
            },
            Token::Mul => {
                left_value = multiplication_expr(&mut lexer, left_value);
            },
            Token::Div => {
                left_value = division_expr(&mut lexer, left_value);
            },
            Token::Mod => {
                left_value = mod_expr(&mut lexer, left_value);
            }
            _ => {
                return left_value
            },
        } 
    }
}

fn multiplication_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = term(&mut lexer);
    Box::new(MultiplicationNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn division_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = term(&mut lexer);
    Box::new(DivisionNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn mod_expr(mut lexer: &mut Lexer, node: Box<dyn ExprNode>) -> Box<dyn ExprNode> {
    lexer.advance();
    let t = term(&mut lexer);
    Box::new(ModNode {
        left_value: Rc::new(node),
        right_value: Rc::new(t),
    })
}

fn assignment_expr(mut lexer: &mut Lexer) -> Box<dyn ExprNode> {
    lexer.matcher(Token::Assgin);
    let right_value = expr9(&mut lexer);

    return right_value;
}

fn term(mut lexer: &mut Lexer) -> Box<dyn ExprNode> {
    let mut case_type = None;
    let node;
    if is_base_type(&lexer.lookahead(1)) {
        case_type = Some(typeref(&mut lexer));
    }
    node = Rc::new(unary(&mut lexer));

    Box::new(TermNode {
        case_type,
        unary: node,
    })
}

fn unary(mut lexer: &mut Lexer) -> Box<dyn UnaryNode> {
    let mut t = None;
    let la = lexer.lookahead(1);
    if is_prefix_op(&la) {
        lexer.advance();
        t = Some(la);
    }

    let pn = primary(&mut lexer);

    if is_postfix_op(&lexer.lookahead(1)) {
        match lexer.advance() {
            Token::Dot => {
                return Box::new(RefUnaryNode {
                    prefix: t,
                    primary: pn,
                    postfix: Some(Rc::new(unary(&mut lexer))),
                })
            },
            Token::PointerRef => {
                return Box::new(PointerRefUnaryNode {
                    prefix: t,
                    primary: pn,
                    postfix: Some(Rc::new(unary(&mut lexer))),
                })
            },
            Token::Inc => {
                return Box::new(SelfOpUnaryNode {
                    prefix: t,
                    primary: pn,
                    postfix: lexer.advance(),
                })
            },
            Token::Dec => {
                return Box::new(SelfOpUnaryNode {
                    prefix: t,
                    primary: pn,
                    postfix: lexer.advance(),
                })
            }
            _ => panic!("unexcept token!")
            // TODO other type
        }
    }

    Box::new(SingeUnaryNode {
        prefix: t,
        primary: pn,
    })
}


fn primary(mut lexer: &mut Lexer) -> PrimaryNode {
    let t = lexer.advance();
    println!("primary {}", t);
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
        let mut lxr = Lexer::new(String::from("a->a = 1"));
        let node = assignment_expr(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_expr9() {
        // let mut lxr = Lexer::new(String::from("+a->b.c->d = 1"));
        let mut lxr = Lexer::new(String::from("6 * 5 % 4 * 3 * 2 / 1"));
        let node = expr9(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_expr8() {
        let mut lxr = Lexer::new(String::from("7 / 6 % 5 + 4 * 3 + 2 / 1"));
        let node = expr8(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_expr7() {
        let mut lxr = Lexer::new(String::from("8 / 7 >> 6 % 5 + 4 * 3 + 2 / 1"));
        let node = expr7(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_expr6() {
        let mut lxr = Lexer::new(String::from("8 / 7 >> 6 & 4 * 3 + 2 / 1"));
        let node = expr6(&mut lxr);
        println!("{:?}", node);
    }
}