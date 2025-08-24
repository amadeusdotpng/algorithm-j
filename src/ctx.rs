use crate::typ::{PolyType, MonoType, VarType};
use std::rc::Rc;
use std::cell::RefCell;


pub struct TypeContext {
    current_id: u8,
    syms: Vec<(Rc<str>, Rc<PolyType>)>,
}

impl TypeContext {
    pub fn new() -> TypeContext {
        TypeContext {
            current_id: 0,
            syms: vec![],
        }
    }

    pub fn insert_sym(&mut self, sym: Rc<str>, t: Rc<PolyType>) {
        self.syms.push((sym, t));
    }

    pub fn pop_sym(&mut self) {
        self.syms.pop();
    }

    pub fn lookup_sym(&self, sym: &str) -> Option<Rc<PolyType>> {
        self.syms.iter()
            .rev()
            .find(|(k, _)| &**k == sym)
            .map(|(_, t)| t.clone())
    }

    pub fn fresh_variable(&mut self) -> MonoType {
        let tvar = RefCell::new(VarType::Unbound { id: self.current_id });
        let t = MonoType::Var { tvar };
        self.current_id += 1;
        t
    }
}

