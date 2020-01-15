use lex::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramNode {
    /*
        import_stmts top_defs EOF
    */
    pub import_stmts: Vec<ImportStmtNode>,
    pub defs: Vec<TopDefNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportStmtNode {
    /*
        IMPORT name (. name) * ;
    */
    pub paths: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopDefNode {

}

#[derive(Clone, Debug, PartialEq)]
pub struct DefVarsNode {
    /*
        type name [ = expr] (, name [ = expr]) * ;
     */
    pub typeref: TypeNode,
    pub name: Vec<String>,
    pub expr: Vec<String>,
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
    pub base: Token,
    pub name: Option<String>,
}