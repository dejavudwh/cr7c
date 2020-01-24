use lex::lexer::Lexer;
use lex::token:: {
    Token,
    is_base_type,
};
use crate::ast_stmt:: {
    StmtNode,
    IfStmtNode,
    ExprStmtNode,
    BlockNode,
    WhileStmtNode,
    DoWhileStmtNode,
    ForStmtNode,
    ReturnStmtNode,
    BreakStmtNode,
    ContinueStmtNode,
};
use crate::parse_expr::expr0;
use crate::parse_def:: {
    defvar,
    typeref,
};

fn statement(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    let t = lexer.lookahead(1);
    let stmt: Box<dyn StmtNode>;
    match t {
        Token::LBrace => {
            stmt = block(&mut lexer);
        }
        Token::If => {
            stmt = if_stmt(&mut lexer);
        },
        Token::While => {
            stmt = while_stmt(&mut lexer);
        },
        Token::Do => {
            stmt = do_while_stmt(&mut lexer);
        },
        Token::For => {
            stmt = for_stmt(&mut lexer);
        },
        Token::Break => {
            stmt = break_stmt(&mut lexer);
        },
        Token::Continue => {
            stmt = continue_stmt(&mut lexer);
        }
        Token::Return => {
            stmt = return_stmt(&mut lexer);
        },
        _ => {
            stmt = expr(&mut lexer);
        }
    }

    return stmt
}

pub fn block(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();

    let mut defvars = Vec::new();
    let mut stmts = Vec::new();
    loop {
        let t = lexer.lookahead(1);
        if t == Token::RBrace {
            lexer.advance();
            break;
        } else if t == Token::Semi {
            lexer.advance();
        } else if is_base_type(&t) {
            let typeref = typeref(&mut lexer);
            defvars.push(defvar(&mut lexer, typeref));
        } else {
            stmts.push(statement(&mut lexer));
        }
    }

    
    Box::new(BlockNode {
        defvars,
        stmts,
    })
}

fn if_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();
    lexer.matcher(Token::LParentheses);
    let condition = expr0(&mut lexer);
    lexer.matcher(Token::RParentheses);
    let if_stmt = statement(&mut lexer);
    let mut else_stmt = None;

    if lexer.lookahead(1) == Token::Else {
        lexer.advance();
        else_stmt = Some(statement(&mut lexer));
    }

    Box::new(IfStmtNode {
        condition,
        if_stmt,
        else_stmt,
    })
}

fn expr(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    let expr = expr0(&mut lexer);
    lexer.matcher(Token::Semi);

    Box::new({
        ExprStmtNode {
            expr,
        }
    })
}

fn while_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();

    lexer.matcher(Token::LParentheses);
    let condition = expr0(&mut lexer);
    lexer.matcher(Token::RParentheses);
    println!("while next token {}", lexer.lookahead(1));
    let stmts = statement(&mut lexer);

    Box::new(WhileStmtNode {
        condition,
        stmts,
    })
}

fn do_while_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();
    let stmts = block(&mut lexer);
    lexer.matcher(Token::While);
    lexer.matcher(Token::LParentheses);
    let condition = expr0(&mut lexer);
    lexer.matcher(Token::RParentheses);

    Box::new(DoWhileStmtNode {
        condition,
        stmts,
    })
}

fn for_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();

    lexer.matcher(Token::LParentheses);
    let initial_expr = expr0(&mut lexer);
    lexer.matcher(Token::Semi);
    let condition = expr0(&mut lexer);
    lexer.matcher(Token::Semi);
    let end_expr = expr0(&mut lexer);
    lexer.matcher(Token::RParentheses);
    let stmts = statement(&mut lexer);

    Box::new(ForStmtNode {
        initial_expr,
        condition,
        end_expr,
        stmts,
    })
}

fn break_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();
    lexer.matcher(Token::Semi);

    Box::new(BreakStmtNode {

    })
}

fn continue_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();
    lexer.matcher(Token::Semi);

    Box::new(ContinueStmtNode {

    })
}

fn return_stmt(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    lexer.advance();
    let value = expr0(&mut lexer);
    lexer.matcher(Token::Semi);

    Box::new(ReturnStmtNode {
        value,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_stmt() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { a = 3 + 5; } else { a = 6; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_block() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { int *[] a = 1; if (3 == 4) { a = 1; } } else { a = 6; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_while_stmt() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { while(3 == 4) { a++; } } else { a = 6; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_do_while_stmt() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { do { a = 1 + 2; } while(a < 3) } else { a = 6; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_for_stmt() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { for(a = 1; a < 3; a++) { b = 10 + 20; } } else { a = 6; return a; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }

    #[test]
    fn test_break_continue_stmt() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { for(a = 1; a < 3; a++) { b = 10 + 20; } } else { a = 6; break; continue; return a; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }
}