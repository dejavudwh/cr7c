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

    pub fn get_type(&self, name: String) -> TypeInfo {
        let struct_type = self.global_define_map.get(&name);
        if let Some(node) = struct_type {
            return TypeInfo {
                origin_struct: Some(node.clone()),
                origin_base: None,
                is_base_type: false,
            }
        } else {
            let mut index = self.scope_stack.len() - 1;
            loop {
                if index <= 0 {
                    panic!("Can't find the symbol \"{}\"", name);
                }
                let local = &self.scope_stack[index];
                if let Some(node) = local.borrow_mut().var_map.get(&name) {
                     return TypeInfo {
                         origin_struct: None,
                         origin_base: Some(node.clone()),
                         is_base_type: true,
                     }
                }
                index = index - 1;
            }
        }
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

#[derive(Debug)]
pub struct TypeInfo {
    pub origin_struct: Option<DefStructNode>,
    pub origin_base: Option<DefVarNode>,
    pub is_base_type: bool,
}