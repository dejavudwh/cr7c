use std::collections::HashMap;
use parse::ast::ProgramNode;
use lex::token::Token;
use parse::parser::parse;
use lex::lexer::Lexer;

fn check_expr(ast: ProgramNode) {
    let funcs = ast.defs.func_defs;

    for func in funcs {
        func.check_expr_validity();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_expr() {
        let mut lxr = Lexer::new(String::from("
            struct student {
                char[] name;
                int age;
                int sex;
                struct teacher te;
            };

            struct teacher {
                struct headmaster ss;
            };

            struct headmaster {
                struct student sss;
                struct teacher *s;
            };

            int main(int argc,char **argv) {
                struct class a;
                int ba1 = 1;
                int bb = 2;
                bb = 1;
                1 = 1+1;
                for(i = 0; i < 10; i++) {
                    int ca = 1;
                    a = 1 * 2 << 3 && 4 + 5 / 6 + calc(a);
                    if (a == 2) {
                        int da = 12;
                        break;
                    } else {
                        continue;
                    }
                }

                while(1 == 2) {
                    int cb = 1;
                }

                return 0;
            }
        "));
        let ast = parse(&mut lxr);
        check_expr(ast);
        println!("===========")
    }
}
