use std::collections::HashMap;
use parse::ast:: {
    ProgramNode,
    TypeDef,
};
use lex::token::Token;
use parse::parser::parse;
use lex::lexer::Lexer;
use parse::symbol_table::TopLevelScope;
use crate::local_resolver::local_resolver;

pub fn check_expr(ast: &ProgramNode, mut symboltable: &mut TopLevelScope) {
    let funcs = &ast.defs.func_defs;

    for func in funcs {
        func.check_expr_validity(&mut symboltable);
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
                int gender;
                struct teacher *t;
            };

            struct teacher {
                struct student[] s;
                struct headmaster *hm;
            };

            struct headmaster {
                struct teacher[] s;
            };

            int main(int argc,char **argv) {
                struct student a;
                int ba1 = 1;
                int[] aa;
                int[10] bb = aa;
                bb[ba1] = 1 * 2 << 3 && 4 + 5 / 6 + calc(2);
                a.age = 2 + calc(a);
                &a;
                for(i = 0; i < 10; i++) {
                    int ca = 1;
                    if (a == 2) {
                        int da = 12;
                        break;
                    } else {
                        continue;
                    }
                }

                while(1 == 2) {
                    int ca = 1;
                    ca = 2;
                }

                return 0;
            }

            int calc(int a) {
                
            }
        "));
        let ast = parse(&mut lxr);
        println!(" === {:?} ", ast);
        let mut symboltable = local_resolver(&ast);
        println!("symboltable : {:?}", symboltable);
        check_expr(&ast, &mut symboltable);
    }
}

