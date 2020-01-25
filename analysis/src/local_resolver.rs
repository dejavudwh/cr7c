use std::collections::HashMap;
use crate::parse::ast:: {
    ProgramNode,
}
use crate::parse::DefNode;

pub struct TopLevelScope {
    pub global_var_map: HashMap<String, DefNode>,
    pub func_map: HashMap<String, DefNode>,
    pub scopes: Vec<LocalScope>,
}

pub struct LocalScope {
    pub var_map: HashMap<String, DefNode>,
    pub scopes: Vec<LocalScope>,
}

pub fn local_resolver(ast: ProgramNode) {
    
}