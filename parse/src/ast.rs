use lex::token::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use std::io::Write;

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
    /*
        int | float | double | struct xxx | char | void 
    */
    pub base: Token,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefFuncNode {
    /*
        typeref name ( [ param ] ) block
    */
    pub typeref: TypeNode,
    pub name: String,
    pub params: ParamsNode,
    // pub block: FuncBodyNode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParamsNode {
    /*
        ( [ slot ( , slot) * ])
    */
    pub params: Vec<SlotNode>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncBodyNode {

}

#[derive(Clone, Debug, PartialEq)]
pub struct DefVarNode {
    /*
        typeref name [ = expr] [, name = [expr] ] *
    */
    pub typeref: TypeNode,
    pub name_map: HashMap<String, ExprNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprNode {

}

#[derive(Clone, Debug)]
pub struct AssginmentNode {
    pub left_value: TermNode,
    pub right_value: TermNode,
}

#[derive(Clone, Debug)]
pub struct TermNode {
    pub case_type: Option<TypeNode>,
    pub unary: Rc<Box<dyn UnaryNode>>,
}

impl fmt::Debug for dyn UnaryNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

pub trait UnaryNode {
    fn print(&self) -> String; 
}

#[derive(Clone, Debug, PartialEq)]
pub struct SingeUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
}

impl UnaryNode for SingeUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "(SingeUnaryNode {:?} {:?})", self.prefix, self.primary);

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayUnaryNode {
    prefix: Option<Token>,
    primary: PrimaryNode,
    postfix: ExprNode,
}

impl UnaryNode for ArrayUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "(ArrayUnaryNode {:?} {:?} {:?})", self.prefix, self.primary, self.postfix);

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub name: Token,
}

impl UnaryNode for RefUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "(RefUnaryNode {:?} {:?} {:?})", self.prefix, self.primary, self.name);

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PointerRefUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub name: Token,
}

impl UnaryNode for PointerRefUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "(PointerRefUnaryNode {:?} {:?} {:?})", self.prefix, self.primary, self.name);

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncCallNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub param: Vec<ExprNode>,
}

impl UnaryNode for FuncCallNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "(FuncCallNode {:?} {:?} {:?})", self.prefix, self.primary, self.param);

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimaryNode {
    pub name: Option<String>,
    pub value: Const,
}

impl UnaryNode for PrimaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "/(PrimaryNode {:?} {:?} )", self.name, self.value);

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Integer(i64),
    Char(char),
    String(String),
    Identifier,
}