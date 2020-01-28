use lex::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast_expr::ExprNode;
use crate::ast_stmt::StmtNode;
use std::fmt;
use crate::symbol_table:: {
    TopLevelScope,
    LocalScope,
};

#[derive(Debug)]
pub struct ProgramNode {
    /*
        import_stmts top_defs EOF
    */
    pub import_stmts: Vec<ImportStmtNode>,
    pub defs: TopDefNode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportStmtNode {
    /*
        IMPORT name (. name) * ;
    */
    pub paths: Vec<String>,
}

pub trait DefNode:fmt::Debug {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {}
    fn check_expr_validity(&self) {}
}

#[derive(Clone, Debug)]
pub struct TopDefNode {
    pub var_defs: Vec<Rc<Box<dyn DefNode>>>,
    pub func_defs: Vec<Rc<Box<dyn DefNode>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefStructNode {
    /*
        STRUCT name member_list ;
        member_list ::= { (slot ;) *} 
    */
    pub name: String,
    pub member_list: Vec<SlotNode>,
}

impl DefNode for DefStructNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        scope.global_define_map.insert(self.name.clone(), self.clone());
    }   
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlotNode {
    /*
        type name
    */
    pub typeref: TypeNode,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeNode {
    /*
        TYPE_BASE ( [] | [ INTEGER ] | * | ( param_typeref ) ) *
    */
    pub type_base: TypeBase,
    pub nested_def: Vec<TypeDef>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeDef {
    Array,
    FixedArray(usize),
    Pointer,
    Func, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeBase {
    /*
        int | float | double | struct xxx | char | void 
    */
    pub base: Token,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DefFuncNode {
    /*
        typeref name ( [ param ] ) block
    */
    pub typeref: TypeNode,
    pub name: String,
    pub params: ParamsNode,
    pub block: Rc<Box<dyn StmtNode>>,
}

impl DefNode for DefFuncNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        scope.func_map.insert(self.name.clone(), self.clone());
        let local = Rc::new(RefCell::new(LocalScope::new()));
        scope.scopes.insert(self.name.clone(), Rc::clone(&local));
        for param in &self.params.params {
            let mut name_map: HashMap<String, Option<Rc<Box<dyn ExprNode>>>> = HashMap::new();
            let typeref = param.typeref.clone();
            name_map.insert(param.name.clone(), None);
            local.borrow_mut().var_map.insert(param.name.clone(), DefVarNode {
                typeref,
                name_map,
            });
        }
        scope.scope_stack.push(local);

        self.block.fill_symbol(scope);

        scope.scope_stack.pop();
    }   

    fn check_expr_validity(&self) {
        self.block.check_expr_validity();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParamsNode {
    /*
        ( [ slot ( , slot) * ])
    */
    pub params: Vec<SlotNode>, 
}

#[derive(Clone, Debug)]
pub struct DefVarNode {
    /*
        type name [ = expr] (, name [ = expr]) * ;
     */
    pub typeref: TypeNode,
    pub name_map: HashMap<String, Option<Rc<Box<dyn ExprNode>>>>,
}

impl DefNode for DefVarNode {
    fn fill_symbol(&self, scope: &mut TopLevelScope) {
        for (name, value) in self.name_map.iter() {
            let last = scope.scope_stack.len() - 1;
            let local = &scope.scope_stack[last];
            local.borrow_mut().var_map.insert(name.clone(), self.clone());
        }
    }   
}
