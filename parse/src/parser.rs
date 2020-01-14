use lex::lexer::Lexer;
use lex::token::Token;
use std::process;
use crate::ast:: {
    ProgramNode,
    ImportStmtNode,
    TopDefNode,
};

pub fn parse(mut lexer: &mut Lexer) {
    compilation_unit(&mut lexer);
}

fn compilation_unit(mut lexer: &mut Lexer) -> ProgramNode {
        // import stmt + defs + EOF
        let mut node = ProgramNode {
            import_stmts: Vec::new(),
            defs: Vec::new(),
        };
        let token = lexer.lookahead(1);
        match token {
            Token::Import => node.import_stmts = import_stmts(&mut lexer),
            Token::Eof => (),
            _ => node.defs = top_defs(&mut lexer),
        };

        return node
}

fn import_stmts(mut lexer: &mut Lexer) -> Vec<ImportStmtNode> {
    let mut nodes = Vec::new();
    while lexer.lookahead(1) == Token::Import {
        lexer.advance();
        nodes.push(import_stmt(&mut lexer));
    }

    return nodes
}

fn import_stmt(mut lexer: &mut Lexer) -> ImportStmtNode {
    let mut paths = Vec::new();
    loop {
        let t = lexer.lookahead(1);
        match t {
            Token::Name(s) => {
                lexer.advance();
                paths.push(s);
            },
            Token::Dot => { 
                lexer.advance();
            },
            _ => break,
        }
    }

    lexer.matcher(Token::Semi).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    return ImportStmtNode {
        paths,
    }
}

fn top_defs(mut lexer: &mut Lexer) -> Vec<TopDefNode> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_stmt() {
        let mut lxr = Lexer::new(String::from("import a.b.c;
        import z.x.c;"));
        println!("{:?}", compilation_unit(&mut lxr).import_stmts);
    }
}