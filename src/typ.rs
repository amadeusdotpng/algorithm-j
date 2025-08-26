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
        write!(f, "forall {}. {}", tvar_ids, self.typ)
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
            MonoType::Func { l, r } => {
                let mut l = l.clone();
                while let MonoType::Var { tvar } = &*l.clone()
                   && let VarType::Bound { typ } = &*tvar.borrow()
                {
                    l = typ.clone();
                    continue;
                }
                match &*l {
                    MonoType::Func { l: l_l, r: l_r } => write!(f, "({l_l} -> {l_r}) -> {r}"),
                    l => write!(f, "{l} -> {r}")
                }
            }

            MonoType::Var { tvar } => write!(f, "{}", tvar.borrow()),
            MonoType::Bool => write!(f, "Bool"),
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
