use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct PolyType {
    pub tvar_ids: Rc<[u8]>,
    pub typ: Rc<MonoType>,
}

impl std::fmt::Display for PolyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tvar_ids = self.tvar_ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "\\{}. {}", tvar_ids, self.typ)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MonoType {
    Bool,
    Func { l: Rc<MonoType>, r: Rc<MonoType> },
    Var { tvar: RefCell<VarType> },
}

impl MonoType {
    pub fn as_poly(self: Rc<Self>) -> PolyType {
        let tvar_ids = Rc::new([]);
        PolyType { tvar_ids, typ: self }
    }
}

impl std::fmt::Display for MonoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonoType::Bool          => write!(f, "Bool"),
            MonoType::Func { l, r } => write!(f, "({} -> {})", l, r),
            MonoType::Var { tvar }  => write!(f, "{}", tvar.borrow()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VarType {
    Bound { typ: Rc<MonoType> },
    Unbound { id: u8 },
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarType::Bound { typ }  => write!(f, "{}", typ),
            VarType::Unbound { id } => write!(f, "'t{}", id),
        }
    }
}
