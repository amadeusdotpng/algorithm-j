use crate::typ::{PolyType, MonoType};
use std::rc::Rc;


pub struct TypeContext {
    current_id: u8,
    syms: Vec<(Rc<str>, PolyType)>,
}

impl TypeContext {
    pub fn new() -> TypeContext {
        TypeContext {
            current_id: 0,
            syms: vec![],
        }
    }

    pub fn insert_sym(&mut self, sym: Rc<str>, t: PolyType) {
        self.syms.push((sym, t));
    }

    pub fn pop_sym(&mut self) {
        self.syms.pop();
    }

    pub fn lookup_sym(&self, sym: &str) -> Option<PolyType> {
        self.syms.iter()
            .rev()
            .find(|(k, _)| &**k == sym)
            .map(|(_, t)| t.clone())
    }

    /*
    pub fn insert_sub(&mut self, id: u8, t: MonoType) {
        self.subs.push((id, t));
    }

    pub fn lookup_sub(&self, id: u8) -> Option<MonoType> {
        self.subs.iter()
            .rev()
            .find(|(k, _)| *k == id)
            .map(|(_, t)| t.clone())
    }
    */

    pub fn fresh_variable(&mut self) -> MonoType {
        let t = MonoType::Unbound { id: self.current_id };
        self.current_id += 1;
        t
    }

    /* I don't think this is ever needed
    fn in_child_scope<T>(&mut self, f: impl FnOnce(&mut TypeContext) -> T) -> T {
        let syms_len = self.syms.len();
        let current_id = self.current_id;

        let res = f(self);

        self.syms.truncate(syms_len);
        self.current_id = current_id;

        res
    }
    */

}

