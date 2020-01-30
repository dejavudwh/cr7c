use std::collections::HashMap;
use parse::ast:: {
    ProgramNode,
    TopDefNode,
};
use lex::lexer::Lexer;
use parse::parser::parse;
use parse::symbol_table::TopLevelScope;

pub fn local_resolver(ast: &ProgramNode) -> TopLevelScope {
    return get_symboltable(&ast.defs)
}

fn get_symboltable(node: &TopDefNode) -> TopLevelScope {
    let mut scope = TopLevelScope::new();
    for var in &node.var_defs {
        var.fill_symbol(&mut scope);
    }

    for func in &node.func_defs {
        func.fill_symbol(&mut scope);
    }

    return scope
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_resolver() {
        let mut lxr = Lexer::new(String::from("
            import a.b.c;
            import z.x.c;

            int aa = 1;
            int ab = 2;
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
                int ba1 = 1;
                int bb = 2;
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

            int calc(int a) {
                int a = 1;

                while(1 == 2) {
                    int aab = 1;
                }

                return a;
            }
        "));
        let ast = parse(&mut lxr);
        println!("{:?}", local_resolver(&ast));
    }
}