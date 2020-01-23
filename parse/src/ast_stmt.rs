use crate::ast_expr::ExprNode;
use std::fmt;
use std::io::Write;

pub trait StmtNode:fmt::Debug {}

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
pub struct NullStmtNode {

}

impl StmtNode for NullStmtNode {}