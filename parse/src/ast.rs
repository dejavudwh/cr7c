use lex::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast_expr::ExprNode;
use crate::ast_stmt::StmtNode;
use std::fmt;

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

pub trait DefNode:fmt::Debug {}

#[derive(Debug)]
pub struct TopDefNode {
    pub defs: Vec<Box<dyn DefNode>>,
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

impl DefNode for DefStructNode {}

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

#[derive(Debug)]
pub struct DefFuncNode {
    /*
        typeref name ( [ param ] ) block
    */
    pub typeref: TypeNode,
    pub name: String,
    pub params: ParamsNode,
    pub block: Box<dyn StmtNode>,
}

impl DefNode for DefFuncNode {}

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

impl DefNode for DefVarNode {}
