use lex::token::Token;
use std::rc::Rc;
use std::fmt;
use crate::ast:: {
    TypeNode,
    DefStructNode,
    TypeDef,
    SlotNode,
};
use std::result::Result;
use crate::symbol_table:: {
    TopLevelScope,
    TypeInfo,
};
use std::collections::HashMap;

fn check_type_compatible(left_type: (Token, Vec<TypeDef>), right_type: (Token, Vec<TypeDef>)) {
    if left_type.0 != right_type.0 {
        panic!("{} and {} types are incompatible", left_type.0, right_type.0);
    }

    let l_len = left_type.1.len();
    let r_len = right_type.1.len();

    if l_len == 0 && r_len == 0 {
        return
    }


    if let Some(l_last_type) = &left_type.1.last() {
        if let Some(r_last_type) = &right_type.1.last() {
            if **l_last_type == TypeDef::Array || **l_last_type == TypeDef::Pointer {
                if **r_last_type != TypeDef::Array && **r_last_type != TypeDef::Pointer {
                    panic!("{} and {} types are incompatible", left_type.0, right_type.0);
                }
            }
        } else {
            panic!("{:?} and {:?} types are incompatible", left_type.1, right_type.1);
        }
    } else {
        panic!("{:?} and {:?} types are incompatible", left_type.1, right_type.1);
    }
}

pub trait ExprNode:fmt::Debug {
    fn is_leftvalue(&self) -> Result<(), String> {
        return Ok(())
    }
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {}
    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        return None
    }
}

#[derive(Clone, Debug)]
pub struct AssginmentNode {
    /*
        expr = expr
    */
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl AssginmentNode {
    /**
     *  赋值的类型检查
     *  首先左值的有效性之前就检查过了，所以这里可以确保左值有效
     *  把初始化的类型检查放到这来，也就是先检查Defvarnode里的
     */
    fn check_type(&self, left_type: Option<TypeInfo>, right_type: Option<TypeInfo>, mut scope: &mut TopLevelScope) {
        // 初始化的类型检查
        self.check_init_type(&left_type, &mut scope);

        println!("==========={:?}", left_type);
        println!("==========={:?}", right_type);

        let left = (left_type.as_ref().unwrap().base_type.clone(), left_type.as_ref().unwrap().nested_def.clone());
        let right = (right_type.as_ref().unwrap().base_type.clone(), right_type.as_ref().unwrap().nested_def.clone());
        check_type_compatible(left, right);
    }

    fn check_init_type(&self, value: &Option<TypeInfo>, mut scope: &mut TopLevelScope) {
        if let Some(info) = value {
            if let Some(var) = &info.origin_base {
                let left = (var.typeref.type_base.base.clone(), var.typeref.nested_def.clone());
                let value = var.name_map.get(&info.name);
                if let Some(expr) = value.unwrap() {
                    let type_info = expr.get_type(&mut scope).unwrap();
                    let right = (type_info.base_type.clone(), type_info.nested_def.clone());
                    check_type_compatible(left, right);
                }
            }
        }
    }
}

impl ExprNode for AssginmentNode {
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        let result = self.left_value.is_leftvalue();
        if result.is_err() {
            panic!(result.err().unwrap())
        }

        self.left_value.check_expr_validity(scope);
        self.right_value.check_expr_validity(scope);

        let left = self.left_value.get_type(&mut scope);
        let right = self.right_value.get_type(&mut scope);
        self.check_type(left, right, &mut scope);
    }
}

#[derive(Clone, Debug)]
pub struct ArithmeticOpNode {
    /*
        expr + | - | * | / | % | ^ | & | && | != | == | <= | < | >= | > | != expr
    */
    pub operator: Token,
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for ArithmeticOpNode {
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.left_value.check_expr_validity(&mut scope);
        self.right_value.check_expr_validity(&mut scope);
    }
}

#[derive(Clone, Debug)]
pub struct TermNode {
    /*
        [ (typeref) ] unary
    */
    pub case_type: Option<TypeNode>,
    pub unary: Rc<Box<dyn UnaryNode>>,
}

impl ExprNode for TermNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.unary.is_leftvalue()
    }

    fn check_expr_validity(&self, scope: &mut TopLevelScope) {
        self.unary.check_expr_validity(scope);
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        return self.unary.get_type(&mut scope);
    }
}

pub trait UnaryNode:fmt::Debug {
    fn is_leftvalue(&self) -> Result<(), String> {
        return Ok(())
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        if let Some(prefix) = self.get_prefix() {
            match prefix {
                Token::Inc |
                Token::Dec |
                Token::Mul |
                Token::Bitand |
                Token::Not => {
                    self.get_name();
                },
                _ => {}
            }
        }
    }

    fn get_postfix(&self) -> &Option<Rc<Box<dyn UnaryNode>>> {
        return &None
    }

    fn get_operator(&self) -> Option<Token> {
        return None
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        return None
    }

    fn get_name(&self) -> String;
    fn get_prefix(&self) -> Option<Token>;
}

#[derive(Clone, Debug)]
pub struct SingeUnaryNode {
    /*
        [prefix] primary
    */
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
}

impl UnaryNode for SingeUnaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_operator(&self) -> Option<Token> {
        return None
    }

    fn get_prefix(&self) -> Option<Token> {
        return self.prefix.clone()
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        let token = self.primary.get_primary_type();
        if let Token::Name(n) = token {
            let t = scope.get_type(&n);
            return Some(t)
        } else {
            return Some(TypeInfo {
                name: String::from("none"),
                origin_struct: None,
                origin_base: None,
                base_type: token,
                nested_def: Vec::new(),
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelfOpUnaryNode {
    /*
        [prefix] primary ++ | --
    */
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Token,
}

impl UnaryNode for SelfOpUnaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        if let Some(prefix) = self.get_prefix() {
            match prefix {
                Token::Inc |
                Token::Dec |
                Token::Mul |
                Token::Bitand |
                Token::Not => {
                    self.get_name();
                },
                _ => {}
            }
        }

        self.get_name();
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_prefix(&self) -> Option<Token> {
        return self.prefix.clone()
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        let token = self.primary.get_primary_type();
        if let Token::Name(n) = token {
            let t = scope.get_type(&n);
            return Some(t)
        } else {
            return Some(TypeInfo {
                name: String::from("none"),
                origin_struct: None,
                origin_base: None,
                base_type: token,
                nested_def: Vec::new(),
            })
        }
    }
}

#[derive(Debug)]
pub struct ArrayUnaryNode {
    /*
        [prefix] primary [expr]
    */
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Vec<Box<dyn ExprNode>>,
}

impl UnaryNode for ArrayUnaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }

    fn check_expr_validity(&self, scope: &mut TopLevelScope) {
        let literal = self.primary.get_primary_type();
        let mut name = self.primary.get_name();
        let var_type = scope.get_type(&name);
        if var_type.nested_def.len() <= 0 {
            panic!(format!("The identifier \"{}\" is not an array or a pointer", name));
        }
        if let Token::Name(n) = literal {
            name = n;
        } else {
            panic!(format!("\"{}\" Type! Cannot be referenced as an array", name));
        }
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        let name = self.primary.get_name();
        let t = scope.get_type(&name);
        // 这里返回的是每个元素的类型
        return Some(TypeInfo {
            name: String::from("none"),
            origin_struct: None,
            origin_base: None,
            base_type: t.base_type.clone(),
            nested_def: Vec::new(),
        })
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_prefix(&self) -> Option<Token> {
        return self.prefix.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RefUnaryNode {
    /*
        [prefix] primary [. unary]*
    */
    pub prefix: Option<Token>,
    pub operator: Token,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl RefUnaryNode {
    fn check_access_op(&self, option_op: &Option<Token>, name: String, member_list: &Vec<SlotNode>) {
        if let Some(op) = option_op {
            let n = name.clone();
            if *op == Token::PointerRef {
                for mem in member_list {
                    if mem.name == n {
                        let nested_def = &mem.typeref.nested_def;
                        if nested_def.len() == 0 || *nested_def.last().unwrap() != TypeDef::Pointer {
                            panic!("Members of the \"{}\" should probably access through .", n);
                        }
                    }
                }
            } else {
                for mem in member_list {
                    if mem.name == n {
                        let nested_def = &mem.typeref.nested_def;
                        if nested_def.len() != 0 {
                            panic!("Members of the \"{}\" should probably access through ->", n);
                        }
                    }
                }
            }            
        }
    }
}

impl UnaryNode for RefUnaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }
    
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        let mut name = self.primary.get_name().clone();
        let struct_type = scope.get_type(&name);
        let mut base_type = &struct_type.base_type;
        let mut member_list = &struct_type.origin_struct.clone().unwrap().member_list;
        let mut postfix = &self.postfix;
        self.check_access_op(&Some(self.operator.clone()), name.clone(), &member_list);
        loop {
            if let Some(unary) = postfix {
                let mut names_type = HashMap::new();
                let mem_name;
                if *base_type == Token::Struct {
                    let mut names = Vec::new();
                    for var in member_list {
                        if let Some(t) = var.typeref.type_base.name.as_ref() {
                            names_type.insert(var.name.clone(), t.clone());
                        } else {
                            names_type.insert(var.name.clone(), String::from(""));
                        }
                        names.push(var.name.clone());
                    }
                    let suffix = postfix.as_ref().unwrap();
                    mem_name = suffix.get_name();
                    if !names.contains(&mem_name) {
                        panic!("{} has no members of \"{}\"", name, mem_name);
                    }
                } else {
                    panic!("Type error! The identifier \"{}\" is not a struct", name);
                }
                name = unary.get_name();
                self.check_access_op(&unary.get_operator(), name.clone(), &member_list);
                postfix = &unary.get_postfix();
                let mem = names_type.get(&mem_name).unwrap();
                if let Some(_type) = &scope.global_define_map.get(mem) { 
                    member_list = &_type.member_list;
                    base_type = &Token::Struct;
                } else {
                    break;
                }
            } else {
                break;
            }        
        }
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        let name = self.primary.get_name();
        let struct_type = scope.get_type(&name);
        let mut member_list = &struct_type.origin_struct.clone().unwrap().member_list;    
        let mut postfix = self.get_postfix();
        loop {
            if let Some(unary) = postfix {
                let mem_name = unary.get_name();
                for mem in member_list {
                    if mem_name == mem.name {
                        let type_base = &mem.typeref.type_base;
                        if let Some(name) = &type_base.name {
                            return Some(TypeInfo {
                                name: mem_name.clone(),
                                origin_struct: Some(scope.global_define_map.get(&name.clone()).unwrap().clone()),
                                origin_base: None,
                                base_type: type_base.base.clone(),
                                nested_def: mem.typeref.nested_def.clone(),
                            })
                        } else {
                            return Some(TypeInfo {
                                name: mem_name.clone(),
                                origin_struct: None,
                                origin_base: None,
                                base_type: type_base.base.clone(),
                                nested_def: mem.typeref.nested_def.clone(),
                            })
                        }
                    }
                }
                postfix = unary.get_postfix();
            } else {
                break;
            }

        }
        return None
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_postfix(&self) -> &Option<Rc<Box<dyn UnaryNode>>> {
        return &self.postfix
    }

    fn get_operator(&self) -> Option<Token> {
        return Some(self.operator.clone());
    }

    fn get_prefix(&self) -> Option<Token> {
        return self.prefix.clone()
    }
}

#[derive(Clone, Debug)]
pub struct PointerRefUnaryNode {
    /*
        [prefix] primary [-> unary]*
    */
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl UnaryNode for PointerRefUnaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }
    
    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_prefix(&self) -> Option<Token> {
        return self.prefix.clone()
    }
}

#[derive(Clone, Debug)]
pub struct FuncCallNode {
    /*
        [prefix] primary (params)
    */
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub params: Option<Vec<Rc<Box<dyn ExprNode>>>>,
}

impl UnaryNode for FuncCallNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_prefix(&self) -> Option<Token> {
        return self.prefix.clone()
    }

    fn get_type(&self, mut scope: &mut TopLevelScope) -> Option<TypeInfo> {
        let name = self.primary.get_name();
        let func = scope.func_map.get(&name);

        return Some(TypeInfo {
            name: name.clone(),
            origin_struct: None,
            origin_base: None,
            base_type: func.unwrap().typeref.type_base.base.clone(),
            nested_def: func.unwrap().typeref.nested_def.clone(),
        }) 
    }
}

#[derive(Clone, Debug)]
pub struct PrimaryNode {
    /*
        Integer | Char | String | Identifier | (expr)
    */
    pub name: Option<String>,
    pub value: Const,
}

impl PrimaryNode {
    fn get_primary_type(&self) -> Token {
        match &self.value {
            Const::Integer(value) => return Token::Int,
            Const::Char(value) => return Token::Char,
            Const::String(value) => return Token::String(String::from(value)),
            Const::Identifier => return Token::Name(String::from(self.name.as_ref().unwrap().clone())),
            Const::ParenthesesExpr(value) => return Token::LParentheses,
        }
    }
}

impl UnaryNode for PrimaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        match &self.value {
            Const::Identifier => Ok(()),
            Const::Integer(value) => Err(format!("Unexpect an left value: {}", value)),
            Const::Char(value) => Err(format!("Unexpect an left value: {}", value)),
            Const::String(value) => Err(format!("Unexpect an left value: {}", value)),
            Const::ParenthesesExpr(value) => Err(format!("Unexpect an left value: {:?}", value)),
        }
    }

    fn get_name(&self) -> String {
        if let Some(name) = &self.name {
            return name.clone()
        } else {
            panic!("Type error! {:?}, Expect an left value", self.value);
        }
    }

    fn get_prefix(&self) -> Option<Token> {
        return None
    }
}

#[derive(Clone, Debug)]
pub enum Const {
    Integer(i64),
    Char(char),
    String(String),
    Identifier,
    ParenthesesExpr(Rc<Box<dyn ExprNode>>),
}