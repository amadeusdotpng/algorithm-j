use crate::typ::{PolyType, MonoType, VarType};
use std::rc::Rc;


pub struct TypeContext {
    current_id: u8,
    syms: Vec<(Rc<str>, PolyType)>,
}

impl TypeContext {
    pub fn new() -> TypeContext {
        TypeContext { current_id: 0, syms: vec![] }
    }

    pub fn insert_sym(&mut self, sym: Rc<str>, t: PolyType) {
        self.syms.push((sym, t));
    }

    pub fn lookup_sym(&mut self, sym: Rc<str>) -> Option<PolyType> {
        self.syms.iter()
            .rev()
            .find(|(k, _)| *k == sym)
            .map(|(_, t)| t.to_owned())
    }

    pub fn fresh_variable(&mut self) -> MonoType {
        let tvar = VarType::Unbound { tvar_id: self.current_id };
        let t = MonoType::Var { tvar: Rc::new(tvar) };
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

