use lex::lexer::Lexer;
use lex::token::Token;
use crate::ast:: {
    ProgramNode,
    ImportStmtNode,
};
use crate::parse_def:: {
    import_statements,
    top_defs,
};

pub fn parse(mut lexer: &mut Lexer) -> ProgramNode {
    return compilation_unit(&mut lexer)
}

fn compilation_unit(mut lexer: &mut Lexer) -> ProgramNode {
    // import_stmts + defs + EOF
    let mut import_stmts: Vec<ImportStmtNode> = Vec::new();
    let token = lexer.lookahead(1);
    println!("=================== {}", token);
    if token == Token::Import {
        import_stmts = import_statements(&mut lexer);
    }
    let defs = top_defs(&mut lexer);

    ProgramNode {
        import_stmts,
        defs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut lxr = Lexer::new(String::from("
            import a.b.c;
            import z.x.c;

            int abc = 1;
            struct student {
                char[] *name;
                int age;
                int sex;
            };

            struct class {
                struct student[] *ss;
            };

            int main(int argc,char **argv) {
                struct class a;
                int a1 = 1;
                calc();
                for(i = 0; i < 10; i++) {
                    a = 1 * 2 << 3 && 4 + 5 / 6 + calc(a);
                    if (a == 2) {
                        break;
                    } else {
                        continue;
                    }
                }
                int i;

                return 0;
            }

            int calc(int a) {
                return a;
            }
        "));
        println!("{:?}", compilation_unit(&mut lxr));
    }
}