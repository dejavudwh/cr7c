use lex::token::Token;
use std::rc::Rc;
use std::fmt;
use std::io::Write;
use crate::ast::TypeNode;

pub trait ExprNode:fmt::Debug {
}

#[derive(Clone, Debug)]
pub struct AssginmentNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AssginmentNode {
}

#[derive(Clone, Debug)]
pub struct MultiplicationNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for MultiplicationNode {
} 

#[derive(Clone, Debug)]
pub struct DivisionNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for DivisionNode {
}

#[derive(Clone, Debug)]
pub struct ModNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for ModNode {
}

#[derive(Clone, Debug)]
pub struct AddNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AddNode {
}

#[derive(Clone, Debug)]
pub struct SubNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for SubNode {
}

#[derive(Clone, Debug)]
pub struct RightShiftNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for RightShiftNode {
}

#[derive(Clone, Debug)]
pub struct LeftShiftNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for LeftShiftNode {
}

#[derive(Clone, Debug)]
pub struct BitAndNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for BitAndNode {
}

#[derive(Clone, Debug)]
pub struct BitXorNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for BitXorNode {
}

#[derive(Clone, Debug)]
pub struct BitOrNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for BitOrNode {
}

#[derive(Clone, Debug)]
pub struct GreaterNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for GreaterNode {
}

#[derive(Clone, Debug)]
pub struct GreaterEqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for GreaterEqualNode {
}

#[derive(Clone, Debug)]
pub struct LessNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for LessNode {
}

#[derive(Clone, Debug)]
pub struct LessEqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for LessEqualNode {
}

#[derive(Clone, Debug)]
pub struct EqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for EqualNode {
}

#[derive(Clone, Debug)]
pub struct NotEqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for NotEqualNode {
}

#[derive(Clone, Debug)]
pub struct AndNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AndNode {
}

#[derive(Clone, Debug)]
pub struct OrNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for OrNode {
}

#[derive(Clone, Debug)]
pub struct TermNode {
    pub case_type: Option<TypeNode>,
    pub unary: Rc<Box<dyn UnaryNode>>,
}

impl ExprNode for TermNode {
}

pub trait UnaryNode:fmt::Debug {
}

#[derive(Clone, Debug)]
pub struct SingeUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
}

impl UnaryNode for SingeUnaryNode {
}

#[derive(Clone, Debug)]
pub struct SelfOpUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Token,
}

impl UnaryNode for SelfOpUnaryNode {
}

#[derive(Clone, Debug)]
pub struct ArrayUnaryNode {
    prefix: Option<Token>,
    primary: PrimaryNode,
    postfix: Rc<Box<dyn ExprNode>>,
}

impl UnaryNode for ArrayUnaryNode {
}

#[derive(Clone, Debug)]
pub struct RefUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl UnaryNode for RefUnaryNode {
}

#[derive(Clone, Debug)]
pub struct PointerRefUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl UnaryNode for PointerRefUnaryNode {
}

#[derive(Clone, Debug)]
pub struct FuncCallNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub params: Option<Vec<Rc<Box<dyn ExprNode>>>>,
}

impl UnaryNode for FuncCallNode {
}

#[derive(Clone, Debug)]
pub struct PrimaryNode {
    pub name: Option<String>,
    pub value: Const,
}

impl UnaryNode for PrimaryNode {
}

#[derive(Clone, Debug)]
pub enum Const {
    Integer(i64),
    Char(char),
    String(String),
    Identifier,
    ParenthesesExpr(Rc<Box<dyn ExprNode>>),
}