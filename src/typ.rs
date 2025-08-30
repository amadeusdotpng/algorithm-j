use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

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

#[derive(Debug, PartialEq, Eq)]
pub enum VarType {
    Bound { typ: Rc<MonoType> },
    Unbound { id: u8 },
}

impl MonoType {
    pub fn as_poly(self: Rc<Self>) -> PolyType {
        let tvar_ids = Rc::new([]);
        PolyType { tvar_ids, typ: self }
    }
}

impl std::fmt::Display for MonoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut unbound_vars = HashSet::new();
        find_vars(&mut unbound_vars, self);

        let mut unbound_vars = unbound_vars.into_iter().collect::<Vec<_>>();
        unbound_vars.sort();

        let disp_type = DisplayType::new(self, &unbound_vars);
        write!(f, "{disp_type}")
    }
}

fn find_vars(unbound_vars: &mut HashSet<u8>, typ: &MonoType) {
    match typ {
        MonoType::Bool => {},
        MonoType::Func { l, r } => {
            find_vars(unbound_vars, l);
            find_vars(unbound_vars, r);
        }
        MonoType::Var { tvar } => match &*tvar.borrow() {
            VarType::Bound { typ }  => find_vars(unbound_vars, typ),
            VarType::Unbound { id } => { unbound_vars.insert(*id); },
        }
    }
}

struct DisplayType<'typ> {
    typ: &'typ MonoType,
    // must be sorted + no duplicates
    unbound_vars: &'typ [u8],
}

impl<'typ> DisplayType<'typ> {
    fn new(typ: &'typ MonoType, unbound_vars: &'typ [u8]) -> DisplayType<'typ> {
        DisplayType { typ, unbound_vars }
    }
}

impl std::fmt::Display for DisplayType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.typ {
            MonoType::Func { l, r } => {
                let mut l = l.clone();
                while let MonoType::Var { tvar } = &*l.clone()
                   && let VarType::Bound { typ } = &*tvar.borrow()
                {
                    l = typ.clone();
                    continue;
                }

                let r = DisplayType::new(r, self.unbound_vars);
                match &*l {
                    MonoType::Func { l: l_l, r: l_r } => {
                        let l_l = DisplayType::new(l_l, self.unbound_vars);
                        let l_r = DisplayType::new(l_r, self.unbound_vars);
                        write!(f, "({l_l} -> {l_r}) -> {r}")
                    }
                    l => {
                        let l = DisplayType::new(l, self.unbound_vars);
                        write!(f, "{l} -> {r}")
                    }
                }
            }

            MonoType::Var { tvar } => match &*tvar.borrow() {
                VarType::Bound { typ } => {
                    let typ = DisplayType::new(typ, self.unbound_vars);
                    write!(f, "{}", typ)
                }

                VarType::Unbound { id } => {
                    let offset = self.unbound_vars.iter()
                        .position(|n| n == id)
                        .expect(format!("Unbound variable of id {id} not found in list").as_str());

                    // Hopefully there won't be more than 26 unbound variables
                    let char_id = ((b'a' as u8) + offset as u8) as char;
                    write!(f, "'{char_id}")
                }
            }

            MonoType::Bool => write!(f, "Bool"),
        }
    }
}
