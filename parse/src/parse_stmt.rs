use lex::lexer::Lexer;
use lex::token::Token;
use crate::ast_stmt:: {
    StmtNode,
    IfStmtNode,
    ExprStmtNode,
    NullStmtNode,
};
use crate::parse_expr::expr0;

fn statement(mut lexer: &mut Lexer) -> Box<dyn StmtNode> {
    if lexer.lookahead(1) == Token::LBrace {
        lexer.advance();
    }

    let t = lexer.lookahead(1);
    let stmt: Box<dyn StmtNode>;
    match t {
        Token::If => {
            stmt = if_stmt(&mut lexer);
        },
        _ => {
            stmt = expr(&mut lexer);
        }
    }

    if lexer.lookahead(1) == Token::RBrace {
        lexer.advance();
    }

    return stmt
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_stmt() {
        let mut lxr = Lexer::new(String::from("if(1 == 2) { a = 3 + 5; } else { a = 6; }"));
        let node = statement(&mut lxr);
        println!("{:?}", node);
    }
}