use lex::token::Token;
use std::rc::Rc;
use std::fmt;
use std::io::Write;
use crate::ast::TypeNode;

pub trait ExprNode {
    fn print(&self) -> String; 
}

impl fmt::Debug for dyn ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

#[derive(Clone, Debug)]
pub struct AssginmentNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AssginmentNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ AssginmentNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct MultiplicationNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for MultiplicationNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ MultiplicationNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
} 

#[derive(Clone, Debug)]
pub struct DivisionNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for DivisionNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ DivisionNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct ModNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for ModNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ ModNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct AddNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AddNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ AddNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct SubNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for SubNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ SubNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct RightShiftNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for RightShiftNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ RightShiftNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct LeftShiftNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for LeftShiftNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ LeftShiftNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct BitAndNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for BitAndNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ BitAndNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct BitXorNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for BitXorNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ BitXorNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct BitOrNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for BitOrNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ BitOrNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct GreaterNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for GreaterNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ GreaterNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct GreaterEqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for GreaterEqualNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ GreaterEqualNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct LessNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for LessNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ LessNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct LessEqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for LessEqualNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ LessEqualNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct EqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for EqualNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ EqualNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct NotEqualNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for NotEqualNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ NotEqualNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct AndNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for AndNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ AndNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct OrNode {
    pub left_value: Rc<Box<dyn ExprNode>>,
    pub right_value: Rc<Box<dyn ExprNode>>,
}

impl ExprNode for OrNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ OrNode {:?} {:?} }}", self.left_value, self.right_value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct TermNode {
    pub case_type: Option<TypeNode>,
    pub unary: Rc<Box<dyn UnaryNode>>,
}

impl ExprNode for TermNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ TermNode {:?} {:?} }}", self.case_type, self.unary).unwrap();

        String::from_utf8(w).unwrap()
    }
}

pub trait UnaryNode {
    fn print(&self) -> String; 
}

impl fmt::Debug for dyn UnaryNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

#[derive(Clone, Debug)]
pub struct SingeUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
}

impl UnaryNode for SingeUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ SingeUnaryNode {:?} {:?} }}", self.prefix, self.primary).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct SelfOpUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Token,
}

impl UnaryNode for SelfOpUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ SingeUnaryNode {:?} {:?} {:?} }}", self.prefix, self.primary, self.postfix).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct ArrayUnaryNode {
    prefix: Option<Token>,
    primary: PrimaryNode,
    postfix: Rc<Box<dyn ExprNode>>,
}

impl UnaryNode for ArrayUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ ArrayUnaryNode {:?} {:?} {:?} }}", self.prefix, self.primary, self.postfix).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct RefUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl UnaryNode for RefUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ RefUnaryNode {:?} {:?} {:?} }}", self.prefix, self.primary, self.postfix).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct PointerRefUnaryNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub postfix: Option<Rc<Box<dyn UnaryNode>>>,
}

impl UnaryNode for PointerRefUnaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ PointerRefUnaryNode {:?} {:?} {:?} }}", self.prefix, self.primary, self.postfix).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct FuncCallNode {
    pub prefix: Option<Token>,
    pub primary: PrimaryNode,
    pub params: Option<Vec<Rc<Box<dyn ExprNode>>>>,
}

impl UnaryNode for FuncCallNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ FuncCallNode {:?} {:?} {:?} }}", self.prefix, self.primary, self.params).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct PrimaryNode {
    pub name: Option<String>,
    pub value: Const,
}

impl UnaryNode for PrimaryNode {
    fn print(&self) -> String {
        let mut w = Vec::new();
        write!(&mut w, "{{ PrimaryNode {:?} {:?} }}", self.name, self.value).unwrap();

        String::from_utf8(w).unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum Const {
    Integer(i64),
    Char(char),
    String(String),
    Identifier,
    ParenthesesExpr(Rc<Box<dyn ExprNode>>),
}