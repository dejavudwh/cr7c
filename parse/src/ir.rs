use crate::ast:: {
    DefVarNode,
    DefFuncNode,
};
use std::rc::Rc;
use std::fmt;

pub trait IRNode:fmt::Debug {

}

#[derive(Clone, Debug)]
pub struct IR {
    pub variables: Vec<DefVarNode>,
    pub functions: Vec<DefFuncNode>,
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub lhs: ExprStmt,
    pub rhs: ExprStmt,
}

impl IRNode for Assign {}

#[derive(Clone, Debug)]
pub struct CJump {
    pub cond: ExprStmt,
    pub thenLabel: LabelStmt,
    pub elseLabel: LabelStmt,
}

impl IRNode for CJump {}

#[derive(Clone, Debug, PartialEq)]
pub struct Jump {
    pub label: LabelStmt,
}

impl IRNode for Jump {}

#[derive(Clone, Debug, PartialEq)]
pub struct LabelStmt {
    pub label: String,
}

impl IRNode for LabelStmt {}

#[derive(Clone, Debug)]
pub struct ExprStmt {
    pub expr: Rc<Box<dyn IRNode>>,
}

impl IRNode for ExprStmt {}

#[derive(Clone, Debug)]
pub struct Return {
    pub expr: Rc<Box<dyn IRNode>>,
}

impl IRNode for Return {}

#[derive(Clone, Debug)]
pub struct Uni {
    pub op: Op,
    pub expr: ExprStmt,
}

impl IRNode for Uni {}

#[derive(Clone, Debug)]
pub struct Bin {
    pub op: Op,
    pub left: ExprStmt,
    pub right: ExprStmt,
}

impl IRNode for Bin {}

#[derive(Clone, Debug)]
pub struct Call {
    pub expr: ExprStmt,
    pub args: Vec<ExprStmt>,
}

impl IRNode for Call {}

#[derive(Clone, Debug)]
pub struct Addr {
    pub expr: ExprStmt,
}

impl IRNode for Addr {}

#[derive(Clone, Debug)]
pub struct Mem {
    pub expr: ExprStmt,
}

impl IRNode for Mem {}

#[derive(Clone, Debug)]
pub struct Var {
    pub expr: DefVarNode,
}

impl IRNode for Var {}

#[derive(Clone, Debug, PartialEq)]
pub struct Int {
    pub value: i32,
}

impl IRNode for Int {}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    BIT_AND,
    BIT_OR,
    BIT_XOR,
    BIT_LSHIFT,
    BIT_RSHIFT,
    EQ,
    NEQ,
    GT,
    GTEQ,
    LT,
    LTEQ,
    NOT,
    CAST,
}