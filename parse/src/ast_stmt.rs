use crate::ast_expr::ExprNode;
use crate::ast::DefVarNode;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::symbol_table:: {
    TopLevelScope,
    LocalScope,
};
use crate::ast::DefNode;

pub trait StmtNode:fmt::Debug {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {}
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {}
}

#[derive(Debug)]
pub struct BlockNode {
    /*
        {
            defvar *  stmts *
        }
    */
    pub defvars: Vec<Box<dyn DefNode>>,
    pub stmts: Vec<Rc<Box<dyn StmtNode>>>,
}

impl StmtNode for BlockNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        let local = Rc::new(RefCell::new(LocalScope::new()));
        let parent = &scope.scope_stack[scope.scope_stack.len() - 1];
        local.borrow_mut().parent = Some(Rc::clone(parent));
        parent.borrow_mut().scopes.push(Rc::clone(&local));
        scope.scope_stack.push(Rc::clone(&local));
        for var in &self.defvars {
            var.fill_symbol(scope);
        }

        for stmt in self.stmts.clone() {
            stmt.fill_symbol(scope);
        }
        scope.scope_stack.pop();
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        let index = scope.scope_stack.len() - 1;
        let local = &scope.scope_stack[index];
        scope.push_block(Rc::clone(local));
        for stmt in &self.stmts {
            stmt.check_expr_validity(&mut scope);
        }
        scope.pop_block();
    }
}

#[derive(Debug)]
pub struct IfStmtNode {
    /*
        IF (expr) block [ELSE block]
    */
    pub condition: Box<dyn ExprNode>,
    pub if_stmt: Box<dyn StmtNode>,
    pub else_stmt: Option<Box<dyn StmtNode>>,
}

impl StmtNode for IfStmtNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        let local = Rc::new(RefCell::new(LocalScope::new()));
        local.borrow_mut().parent = Some(Rc::clone(&scope.scope_stack[scope.scope_stack.len() - 1]));
        self.if_stmt.fill_symbol(scope);
        if let Some(block) = &self.else_stmt {
            block.fill_symbol(scope);
        }
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.if_stmt.check_expr_validity(&mut scope);
        if let Some(block) = &self.else_stmt {
            block.check_expr_validity(&mut scope);
        }
    }
}

#[derive(Debug)]
pub struct ExprStmtNode {
    pub expr: Box<dyn ExprNode>,
}

impl StmtNode for ExprStmtNode {
    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.expr.check_expr_validity(&mut scope);
    }
}

#[derive(Debug)]
pub struct WhileStmtNode {
    /*
        WHILE (expr) block
    */
    pub condition: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for WhileStmtNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        let local = Rc::new(RefCell::new(LocalScope::new()));
        local.borrow_mut().parent = Some(Rc::clone(&scope.scope_stack[scope.scope_stack.len() - 1]));
        self.stmts.fill_symbol(scope);
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.stmts.check_expr_validity(&mut scope);
    }
}

#[derive(Debug)]
pub struct DoWhileStmtNode {
    /*
        DO block WHILE (expr)
    */
    pub condition: Box<dyn ExprNode>,
    pub stmts: Box<dyn StmtNode>,
}

impl StmtNode for DoWhileStmtNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        let local = Rc::new(RefCell::new(LocalScope::new()));
        local.borrow_mut().parent = Some(Rc::clone(&scope.scope_stack[scope.scope_stack.len() - 1]));
        self.stmts.fill_symbol(scope);
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.stmts.check_expr_validity(&mut scope);
    }
}

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

impl StmtNode for ForStmtNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        let local = Rc::new(RefCell::new(LocalScope::new()));
        local.borrow_mut().parent = Some(Rc::clone(&scope.scope_stack[scope.scope_stack.len() - 1]));
        self.stmts.fill_symbol(scope);
    }

    fn check_expr_validity(&self, mut scope: &mut TopLevelScope) {
        self.stmts.check_expr_validity(&mut scope);
    }
}

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
