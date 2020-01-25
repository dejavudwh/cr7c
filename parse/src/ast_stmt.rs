use crate::ast_expr::ExprNode;
use crate::ast::DefVarNode;
use std::fmt;

pub trait StmtNode:fmt::Debug {}

#[derive(Debug)]
pub struct BlockNode {
    /*
        {
            defvar *  stmts *
        }
    */
    pub defvars: Vec<DefVarNode>,
    pub stmts: Vec<Box<dyn StmtNode>>,
}

impl StmtNode for BlockNode {}

#[derive(Debug)]
pub struct IfStmtNode {
    /*
        IF (expr) block [ELSE block]
    */
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
    /*
        WHILE (expr) block
    */
    pub condition: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for WhileStmtNode {}

#[derive(Debug)]
pub struct DoWhileStmtNode {
    /*
        DO block WHILE (expr)
    */
    pub condition: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for DoWhileStmtNode {}

#[derive(Debug)]
pub struct ForStmtNode {
    /*
        for (expr; expr; expr;) block
    */
    pub initial_expr: Box<dyn ExprNode>,
    pub condition: Box<dyn ExprNode>,
    pub end_expr: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for ForStmtNode {}

#[derive(Debug)]
pub struct ReturnStmtNode {
    /*
        RETURN expr
    */
    pub value: Box<dyn ExprNode>,
}

impl StmtNode for ReturnStmtNode {}

#[derive(Debug)]
pub struct BreakStmtNode {
    /*
        BREAK ;
    */
}

impl StmtNode for BreakStmtNode {}

#[derive(Debug)]
pub struct ContinueStmtNode {
    /*
        CONTINUE ;
    */
}

impl StmtNode for ContinueStmtNode {}
