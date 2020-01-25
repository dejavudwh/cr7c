use std::collections::HashMap;
use parse::ast:: {
    ProgramNode,
    TopDefNode,
    DefNode,
};
use std::fmt;
use std::rc::Rc;
use lex::lexer::Lexer;
use parse::parser::parse;

struct TopLevelScope {
    pub global_var_map: HashMap<String, Rc<Box<dyn DefNode>>>,
    pub func_map: HashMap<String, Rc<Box<dyn DefNode>>>,
    pub scopes: Vec<LocalScope>,
}

impl fmt::Debug for TopLevelScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.global_var_map, self.func_map)
    }
}

struct LocalScope {
    pub var_map: HashMap<String, Box<dyn DefNode>>,
    pub scopes: Vec<LocalScope>,
}

pub fn local_resolver(ast: ProgramNode) {
    
}

fn global_def(node: TopDefNode) -> TopLevelScope {
    let mut global_var_map = HashMap::new();
    let mut func_map = HashMap::new();
    for var in node.var_defs {
        let names = var.get_names();
        for name in names {
            println!("=== {}", name);
            global_var_map.insert(name, var.clone());
        }
    }
    for func in node.func_defs {
        let names = func.get_names();
        for name in names {
            println!("=== {}", name);
            func_map.insert(name, func.clone());
        }
    }

    TopLevelScope {
        global_var_map,
        func_map,
        scopes: Vec::new(),
    }
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