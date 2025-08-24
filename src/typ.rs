use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct PolyType {
    pub tvar_ids: Rc<[u8]>,
    pub typ: MonoType,
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

#[derive(Debug, Clone)]
pub enum MonoType {
    Bool,
    Func { l: Rc<RefCell<MonoType>>, r: Rc<RefCell<MonoType>> },
    Unbound { id: u8 },
    Bound { typ: Rc<RefCell<MonoType>> },
}

impl MonoType {
    pub fn as_poly(self) -> PolyType {
        let tvar_ids = Rc::new([]);
        PolyType { tvar_ids, typ: self }
    }
}

impl std::fmt::Display for MonoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonoType::Bool           => write!(f, "Bool"),
            MonoType::Func { l, r }  => write!(f, "{} -> {}", l.borrow(), r.borrow()),
            MonoType::Unbound { id } => write!(f, "{}", id),
            MonoType::Bound { typ }  => write!(f, "{}", typ.borrow()),
        }
    }
}
