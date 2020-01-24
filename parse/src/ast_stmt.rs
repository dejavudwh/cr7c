use crate::ast_expr::ExprNode;
use crate::ast::DefVarNode;
use std::fmt;

pub trait StmtNode:fmt::Debug {}

#[derive(Debug)]
pub struct BlockNode {
    pub defvars: Vec<DefVarNode>,
    pub stmts: Vec<Box<dyn StmtNode>>,
}

impl StmtNode for BlockNode {}

#[derive(Debug)]
pub struct IfStmtNode {
    pub condition: Box<dyn ExprNode>,
    pub if_stmt: Box<dyn StmtNode>,
    pub else_stmt: Option<Box<dyn StmtNode>>,
}

impl StmtNode for IfStmtNode {}

#[derive(Debug)]
pub struct ExprStmtNode {
    pub expr: Box<dyn ExprNode>,
}

impl StmtNode for ExprStmtNode {}

#[derive(Debug)]
pub struct WhileStmtNode {
    pub condition: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for WhileStmtNode {}

#[derive(Debug)]
pub struct DoWhileStmtNode {
    pub condition: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for DoWhileStmtNode {}

#[derive(Debug)]
pub struct ForStmtNode {
    pub initial_expr: Box<dyn ExprNode>,
    pub condition: Box<dyn ExprNode>,
    pub end_expr: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for ForStmtNode {}

