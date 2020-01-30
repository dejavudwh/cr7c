use lex::token::Token;
use std::rc::Rc;
use std::fmt;
use crate::ast::TypeNode;
use std::result::Result;
use crate::symbol_table::TopLevelScope;

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
        let var_type = scope.get_type(self.primary.get_name());
        println!("vat type === : {:?}", var_type);
        if var_type.nested_def.len() <= 0 {
            panic!(format!("The identifier \"{}\" is not an array or a pointer", var_type.name));
        }
        if literal != String::from("Identifier") {
            panic!(format!("\"{}\" Type! Cannot be referenced as an array", literal));
        }
    }
}

#[derive(Clone, Debug)]
pub struct RefUnaryNode {
    /*
        [prefix] primary [. unary]*
    */
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl UnaryNode for RefUnaryNode {
    fn is_leftvalue(&self) -> Result<(), String> {
        return self.primary.is_leftvalue()
    }
    
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.primary.check_expr_validity(&mut scope);
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

    fn get_name(&self) -> String {
        if let Some(name) = &self.name {
            return name.clone()
        } else {
            panic!("Type error! {:?}", self.value);
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
}

#[derive(Clone, Debug)]
pub enum Const {
    Integer(i64),
    Char(char),
    String(String),
    Identifier,
    ParenthesesExpr(Rc<Box<dyn ExprNode>>),
}