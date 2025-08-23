use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PolyType {
    pub tvar_ids: Rc<[u8]>,
    pub typ: MonoType,
}

#[derive(Debug, Clone)]
pub enum MonoType {
    Bool,
    Func { l: Rc<MonoType>, r: Rc<MonoType> },
    Var { tvar: Rc<VarType> },
}

impl MonoType {
    pub fn as_poly(self) -> PolyType {
        let tvar_ids = Rc::new([]);
        PolyType { tvar_ids, typ: self }
    }
}

#[derive(Debug)]
pub enum VarType {
    Bound { typ: MonoType },
    Unbound { tvar_id: u8 },
}
