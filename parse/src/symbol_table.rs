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
    pub order_block: Vec<usize>,
    pub current_scope: Option<Rc<RefCell<LocalScope>>>,
}

impl TopLevelScope {
    pub fn new() -> Self {
        let mut scope = TopLevelScope {
            global_define_map: HashMap::new(),
            func_map: HashMap::new(),
            scopes: HashMap::new(),
            scope_stack: Vec::new(),
            order_block: Vec::new(),
            current_scope: None,
        };
        let global_scope = Rc::new(RefCell::new(LocalScope::new()));
        scope.scopes.insert(String::from("GLOBAL"), Rc::clone(&global_scope));
        scope.scope_stack.push(Rc::clone(&global_scope));

        return scope
    }

    pub fn push_func(&mut self, name: String) {
        self.order_block = Vec::new();
        let local = &self.scopes.get(&name);
        self.scope_stack.push(Rc::clone(local.unwrap()));
        self.current_scope = Some(Rc::clone(local.unwrap()));
        self.order_block.push(0);
    }

    pub fn push_block(&mut self, local_scope: Rc<RefCell<LocalScope>>) {
        let l = local_scope.borrow_mut().scopes.len();
        // println!("name {:?} len {:?}", local_scope.borrow_mut().var_map, l);
        let mut len = local_scope.borrow_mut().scopes.len();
        loop {
            let local_children_size = self.order_block[self.order_block.len() - 1];
            // println!("order block {:?} {:?} {:?}", self.order_block, len, local_children_size);
            if len <= local_children_size {
                let pop = self.scope_stack.pop();
                // println!("order block in pop {:?}", pop);
                self.order_block.pop();
                let l = self.order_block.len() - 1;
                let origin = self.order_block[l];
                self.order_block[l] = origin + 1;
                let last_local = &self.scope_stack[self.scope_stack.len() - 1];
                self.current_scope = Some(Rc::clone(last_local));
                len = last_local.borrow_mut().scopes.len();
            } else {
                break;
            }
        }
        // println!("last {:?}", self.order_block);
        let last = self.order_block[self.order_block.len() - 1];
        println!("====== push {:?}", local_scope);
        // let local = &self.scope_stack[self.scope_stack.len() - 1];
        let local = &self.current_scope.as_ref().unwrap();
        let scope = &local.borrow_mut().scopes[last];
        self.scope_stack.push(Rc::clone(scope));
        self.order_block.push(0);
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