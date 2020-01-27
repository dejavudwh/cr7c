use std::collections::HashMap;
use parse::ast:: {
    ProgramNode,
    TopDefNode,
};
use lex::lexer::Lexer;
use parse::parser::parse;
use parse::symbol_table::TopLevelScope;

pub fn local_resolver(ast: ProgramNode) {
    
}

fn global_def(node: TopDefNode) -> TopLevelScope {
    let mut scope = TopLevelScope::new();
    for var in node.var_defs {
        var.fill_symbol(&mut scope);
    }

    for func in node.func_defs {
        func.fill_symbol(&mut scope);
    }

    return scope
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_def() {
        let mut lxr = Lexer::new(String::from("
            import a.b.c;
            import z.x.c;

            int abc = 1;
            struct student {
                char[] name;
                int age;
                int sex;
            };

            struct class {
                struct student[] *ss;
            };

            int main(int argc,char **argv) {
                struct class a;
                int a = 1;
                int i;
                for(i = 0; i < 10; i++) {
                    a = 1 * 2 << 3 && 4 + 5 / 6 + calc(a);
                    if (a == 2) {
                        break;
                    } else {
                        continue;
                    }
                }

                return 0;
            }

            int calc(int a) {
                return a;
            }
        "));
        let ast = parse(&mut lxr);
        println!("{:?}", global_def(ast.defs));
    }
}