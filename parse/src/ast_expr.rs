use lex::token::Token;
use std::rc::Rc;
use std::fmt;
use crate::ast::TypeNode;
use std::result::Result;

pub trait ExprNode:fmt::Debug {
    fn is_leftvalue(&self) -> Result<(), String> {
        return Ok(())
    }
    fn check_expr_validity(&self) {}
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
    fn check_expr_validity(&self) {
        let result = self.left_value.is_leftvalue();
        if result.is_err() {
            panic!(result.err().unwrap())
        }
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
}

pub trait UnaryNode:fmt::Debug {
    fn is_leftvalue(&self) -> Result<(), String> {
        return Ok(())
    }
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
}

#[derive(Clone, Debug)]
pub struct PrimaryNode {
    /*
        Integer | Char | String | Identifier | (expr)
    */
    pub name: Option<String>,
    pub value: Const,
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