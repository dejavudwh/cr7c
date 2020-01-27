use std::fmt;
use std::collections::HashMap;
use crate::ast:: {
    DefStructNode,
    DefVarNode,
    DefFuncNode,
};
use std::rc::Rc;
use std::cell::RefCell;

pub struct TopLevelScope {
    pub global_define_map: HashMap<String, DefStructNode>,
    pub func_map: HashMap<String, DefFuncNode>,
    pub scopes: HashMap<String, Rc<RefCell<LocalScope>>>,
    pub scope_stack: Vec<Rc<RefCell<LocalScope>>>,
}

impl TopLevelScope {
    pub fn new() -> Self {
        let mut scope = TopLevelScope {
            global_define_map: HashMap::new(),
            func_map: HashMap::new(),
            scopes: HashMap::new(),
            scope_stack: Vec::new(),
        };
        let global_scope = Rc::new(RefCell::new(LocalScope::new()));
        scope.scopes.insert(String::from("GLOBAL"), Rc::clone(&global_scope));
        scope.scope_stack.push(Rc::clone(&global_scope));

        return scope
    }
}

impl fmt::Debug for TopLevelScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLOBAL DEFINE: {:?} \nGLOBAL FUNC: {:?} \nSCOPES: {:?}", self.global_define_map.keys(), self.func_map.keys(), self.scopes)
    }
}

pub struct LocalScope {
    pub parent: Option<Rc<RefCell<LocalScope>>>,
    pub var_map: HashMap<String, DefVarNode>,
    pub scopes: Vec<Rc<RefCell<LocalScope>>>,
}

impl LocalScope {
    pub fn new() -> Self {
        LocalScope {
            parent: None,
            var_map: HashMap::new(),
            scopes: Vec::new(),
        }
    }
}

impl fmt::Debug for LocalScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VAR: {:?} NEXT SCOPES: {:?} ", self.var_map.keys(), self.scopes)
    }
}