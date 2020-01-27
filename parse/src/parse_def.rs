use lex::lexer::Lexer;
use lex::token:: {
    Token,
    is_base_type,
};
use crate::ast:: {
    ImportStmtNode,
    TopDefNode,
    DefStructNode,
    SlotNode,
    TypeNode,
    TypeDef,
    TypeBase,
    DefFuncNode,
    ParamsNode,
    DefVarNode,
    DefNode,
};
use crate::parse_expr::expr0;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast_expr::ExprNode;
use crate::parse_stmt::block;

pub fn import_statements(mut lexer: &mut Lexer) -> Vec<ImportStmtNode> {
    // import_stmt *
    let mut nodes = Vec::new();
    while lexer.lookahead(1) == Token::Import {
        lexer.advance();
        nodes.push(import_statement(&mut lexer));
    }

    return nodes
}

fn import_statement(mut lexer: &mut Lexer) -> ImportStmtNode {
    // IMPORT NAME (. NAME)* 
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

    lexer.matcher(Token::Semi);

    return ImportStmtNode {
        paths,
    }
}

pub fn top_defs(mut lexer: &mut Lexer) -> TopDefNode {
    let mut var_defs: Vec<Rc<Box<dyn DefNode>>> = Vec::new();
    let mut func_defs: Vec<Rc<Box<dyn DefNode>>> = Vec::new();

    loop {
        let t = lexer.lookahead(1);
        println!(" ========== loop times {}", t);
        if t == Token::Struct {
            var_defs.push(Rc::new(Box::new(defstruct(&mut lexer))));
        } else if is_base_type(&t) {
            let typeref = typeref(&mut lexer);
            if lexer.lookahead(2) == Token::LParentheses {
                func_defs.push(Rc::new(Box::new(deffunc(&mut lexer, typeref))));
            } else {
                var_defs.push(Rc::new(Box::new(defvar(&mut lexer, typeref))));
            }
        } else {
            break;
        }
    }

    TopDefNode {
        var_defs,
        func_defs
    }
}

pub fn defstruct(mut lexer: &mut Lexer) -> DefStructNode {
    // TYPE_BASE ( [] | [ INTEGER ] | * | ( param_typeref ) ) *
    lexer.advance();
    let mut name = String::from("");
    let mut member_list = Vec::new();
    
    let t = lexer.lookahead(1);
    match t {
        Token::Name(s) => name = s,
        _ => panic!("unexcept token! {}", t)
    }
    lexer.advance();

    lexer.matcher(Token::LBrace);

    while is_base_type(&lexer.lookahead(1)) {
        member_list.push(slot(&mut lexer));
        lexer.matcher(Token::Semi);
    }

    lexer.matcher(Token::RBrace);
    lexer.matcher(Token::Semi);

    DefStructNode {
        name,
        member_list,
    }
}

fn slot(mut lexer: &mut Lexer) -> SlotNode {
    /*
        type name
    */
    let typeref = typeref(&mut lexer);
    let name;

    let t = lexer.lookahead(1);
    match t {
        Token::Name(s) => name = s,
        _ => panic!("unexcept token! {}", t)
    }

    lexer.advance();

    SlotNode {
        typeref,
        name,
    }
}

pub fn typeref(mut lexer: &mut Lexer) -> TypeNode {
    // TYPE_BASE ( [] | [ INTEGER ] | * | ( param_typeref ) ) *
    let type_base = typebase(&mut lexer);
    let mut nested_def = Vec::new();
    loop {
        let t = lexer.lookahead(1);
        match t {
            Token::LBrackets => {
                lexer.advance();
                let t = lexer.lookahead(1);
                match t {
                    Token::RBrackets => {
                        nested_def.push(TypeDef::Array);
                    },
                    Token::Number(n) => {
                        nested_def.push(TypeDef::FixedArray(n as usize));
                    },
                    _ => panic!("unexcept token! {}", t)
                }
            },
            Token::Mul => {
                nested_def.push(TypeDef::Pointer);
            },
            // TODO func
            _ => break,
        }
        lexer.advance();
    }

    TypeNode {
        type_base,
        nested_def,
    }
}

fn typebase(mut lexer: &mut Lexer) -> TypeBase {
    /*
        int | float | double | struct xxx | char | void 
    */
    let base;
    let mut name = None;
    let t = lexer.lookahead(1);
    println!("base {}", t);
    if is_base_type(&t) {
        base = t;
        lexer.advance();
    } else {
        panic!("unexcept token! {} ", t);
    }

    if base == Token::Struct {
        let t = lexer.lookahead(1); 
        match t {
            Token::Name(s) => name = Some(s.clone()),
            _ => panic!("unexcept token! {}", t),
        }
        lexer.advance();
    }

    TypeBase {
        base,
        name,
    }
}


fn deffunc(mut lexer: &mut Lexer, typeref: TypeNode) -> DefFuncNode {
    /*
        typeref name ( [ param ] ) block
    */
    // let typeref = typeref(&mut lexer);
    let name;

    let t = lexer.lookahead(1);
    match t {
        Token::Name(s) => name = s,
        _ => panic!("unexcept token! {}", t),
    };

    lexer.advance();

    let params = params(&mut lexer);
    let block = Rc::new(block(&mut lexer));

    DefFuncNode {
        typeref,
        name,
        params,
        block,
    }
}

fn params(mut lexer: &mut Lexer) -> ParamsNode {
    /*
        ( [ slot ( , slot) * ])
    */
    lexer.matcher(Token::LParentheses);
    let mut params: Vec<SlotNode> = Vec::new();
    
    loop {
        if lexer.lookahead(1) != Token::RParentheses {
            params.push(slot(&mut lexer));
        }

        let t = lexer.lookahead(1);
        match t {
            Token::RParentheses => break,
            Token::Comma => lexer.advance(),
            _ => panic!("unexcept token! {}", t),
        };
    }

    lexer.matcher(Token::RParentheses);

    ParamsNode {
        params,
    }
}

pub fn defvar(mut lexer: &mut Lexer, typeref: TypeNode) -> DefVarNode {
    /*
        typeref name [ = expr] [, name = [expr] ] *
    */
    // let typeref = typeref(&mut lexer);
    let mut name_map = HashMap::new();
    var_stmt(&mut lexer, &mut name_map);

    DefVarNode {
        typeref,
        name_map: name_map.clone(),
    }
}

fn var_stmt(mut lexer: &mut Lexer, name_map: &mut HashMap<String, Option<Rc<Box<dyn ExprNode>>>>) {
    let mut name = String::from("");
    if let Token::Name(s) = lexer.lookahead(1) {
        lexer.advance();
        name = s;
    } else {
        panic!("unexcept token! {}", lexer.lookahead(1));
    }
    let t = lexer.lookahead(1);
    match t {
        Token::Assgin => {
            lexer.advance();
            name_map.insert(name, Some(Rc::new(expr0(&mut lexer))));

            if lexer.lookahead(1) == Token::Comma {
                lexer.advance();
                var_stmt(&mut lexer, name_map);
            }
        },
        Token::Comma => {
            name_map.insert(name, None);
            lexer.advance();
            var_stmt(&mut lexer, name_map);
        }
        Token::Semi => {
            name_map.insert(name, None);
        },
        _ => {
            panic!("unexcpet token! {}", t);
        }
    }
    lexer.matcher(Token::Semi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_stmt() {
        let mut lxr = Lexer::new(String::from("import a.b.c;
        import z.x.c;"));
        println!("{:?}", import_statements(&mut lxr));
    }

    #[test]
    fn test_struct() {
        let mut lxr = Lexer::new(String::from("struct st {
            int[][] *a;
            struct stu b;
            float* c;
            int[2] d;
        };"));
        println!("{:?}", defstruct(&mut lxr));
    }

    #[test]
    fn test_deffunc() {
        let mut lxr = Lexer::new(String::from("float test(int[] *a, struct na b) { if(1 == 2) { for(a = 1; a < 3; a++) { b = 10 + 20; } } else { a = 6; return a; } }"));
        // println!("{:?}", deffunc(&mut lxr));
    }

    #[test]
    fn test_defvars() {
        let mut lxr = Lexer::new(String::from("struct stu *[] a = a + 32, b = 234, c;"));
        // println!("{:?}", defvar(&mut lxr));
    }

    #[test]
    fn test_defs() {
        let mut lxr = Lexer::new(String::from("
            int abc = 1;
            struct student {
                char[] name;
                int age;
                int sex;
            }

            struct class {
                struct student[] *ss;
            }

            int main(int argc,char **argv) {
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
        println!("{:?}", top_defs(&mut lxr));
    }
}