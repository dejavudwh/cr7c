use std::collections::HashMap;
use parse::ast::DefStructNode;
use lex::token::Token;
use parse::parser::parse;
use lex::lexer::Lexer;
use crate::local_resolver::local_resolver;

#[derive(Debug, Clone)]
struct StructTypeNode {
    pub name: String,
    pub next: HashMap<String, usize>,
    pub checked: bool,
}

fn check_circle_def(defines: HashMap<String, DefStructNode>) {
    let mut type_graph = build_type_graph(defines);
    println!("{:?}", type_graph);
    depth_traversal(0, &mut type_graph);
}

fn depth_traversal(index: usize, mut graph: &mut Vec<StructTypeNode>) {
    let mut node = &mut graph[index];
    if node.checked {
        panic!("\"{}\" recursive definition ", node.name);
    }
    node.checked = true;
    println!("set node: {}", node.name);
    for (name, i) in node.next.clone() {
        depth_traversal(i, &mut graph);
    }
}

fn build_type_graph(defines: HashMap<String, DefStructNode>) -> Vec<StructTypeNode> {
    let mut type_graph: Vec<StructTypeNode> = Vec::new();
    for (name, def) in defines {
        let mut next = HashMap::new();
        for member in def.member_list {
            let type_base = member.typeref.type_base;
            let is_struct = type_base.base == Token::Struct;
            let not_pointer = member.typeref.nested_def.len() == 0;
            if is_struct && not_pointer {
                next.insert(type_base.name.unwrap(), 0);
            }
        } 
        type_graph.push(StructTypeNode {
            name,
            next,
            checked: false,
        });
    }

    let mut index_map = HashMap::new();
    for (i, node) in type_graph.iter().enumerate() {
        index_map.insert(node.name.clone(), i);
    }

    for node in &mut type_graph {
        for name in node.next.clone().keys() {
            node.next.insert(name.clone(), index_map[&name.clone()]);
        }
    }

    return type_graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_type_graph() {
        let mut lxr = Lexer::new(String::from("
            struct student {
                char[] name;
                int age;
                int sex;
                struct teacher te;
            };

            struct teacher {
                struct student[] *ss;
            };

            struct headmaster {
                struct student sss;
                struct teacher *s;
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
        "));
        let ast = parse(&mut lxr);
        let symboltable = local_resolver(ast);
        println!("{:?}", build_type_graph(symboltable.global_define_map))
    }

    #[test]
    fn test_check_type_circle() {
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
        let symboltable = local_resolver(ast);
        check_circle_def(symboltable.global_define_map);
    }

    #[test]
    fn test_find_symbol() {
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
        let symboltable = local_resolver(ast);
        println!("{:?}", symboltable.get_type(String::from("bb")));
    }
}