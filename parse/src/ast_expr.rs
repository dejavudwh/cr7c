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

pub trait ExprNode:fmt::Debug {
    fn is_leftvalue(&self) -> Result<(), String> {
        return Ok(())
    }
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {}
}

#[derive(Clone, Debug)]
pub struct AssginmentNode {
    /*
        expr = expr
    */
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AssginmentNode {
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        let result = self.left_value.is_leftvalue();
        if result.is_err() {
            panic!(result.err().unwrap())
        }

        self.left_value.check_expr_validity(scope);
        self.right_value.check_expr_validity(scope);
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
}

pub trait UnaryNode:fmt::Debug {
    fn is_leftvalue(&self) -> Result<(), String> {
        return Ok(())
    }
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {}
    fn get_postfix(&self) -> &Option<Rc<Box<dyn UnaryNode>>> {
        return &None
    }
    fn get_operator(&self) -> Option<Token> {
        return None
    }
    fn get_name(&self) -> String;
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
        self.primary.check_expr_validity(&mut scope);
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
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
        let literal = self.primary.get_type();
        let name = self.primary.get_name();
        let var_type = scope.get_type(&name);
        if var_type.nested_def.len() <= 0 {
            panic!(format!("The identifier \"{}\" is not an array or a pointer", name));
        }
        if literal != String::from("Identifier") {
            panic!(format!("\"{}\" Type! Cannot be referenced as an array", literal));
        }
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
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
    fn typename_from_typeinfo(&self, name: &String, type_info: &TypeInfo) -> Option<String> {
        let struct_node = type_info.origin_struct.as_ref().unwrap();
        for var in &struct_node.member_list {
            let n = var.typeref.type_base.name.as_ref().unwrap();
            println!("{:?} {:?}", n, name);
            if n.as_str() == name.as_str() {
                return Some(name.clone())
            }
        }

        return None
    }

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

    fn get_name(&self) -> String {
        return self.primary.get_name();
    }

    fn get_postfix(&self) -> &Option<Rc<Box<dyn UnaryNode>>> {
        return &self.postfix
    }

    fn get_operator(&self) -> Option<Token> {
        return Some(self.operator.clone());
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
    
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.primary.check_expr_validity(&mut scope);
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
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
    
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.primary.check_expr_validity(&mut scope);
    }

    fn get_name(&self) -> String {
        return self.primary.get_name();
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
    fn get_type(&self) -> String {
        match &self.value {
            Const::Integer(value) => return String::from("Integer"),
            Const::Char(value) => return String::from("Char"),
            Const::String(value) => return String::from("String"),
            Const::Identifier => return String::from("Identifier"),
            Const::ParenthesesExpr(value) => return String::from("Expr"),
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
            panic!("Type error! {:?}", self.value);
        }
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