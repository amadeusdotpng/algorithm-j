use crate::typ::{PolyType, MonoType, VarType};

use crate::TypeContext;

use ast::Expression;
use thiserror::Error;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Recursive types are not allowed.")]
    RecursiveType,
    #[error("Mismatched types {0} and {1}.")]
    TypeMismatch(Rc<MonoType>, Rc<MonoType>),
    #[error("Variable {0} not found.")]
    VarNotFound(Rc<str>),
}

type Result<T> = std::result::Result<T, TypeError>;

fn instantiate(ctx: &mut TypeContext, t: Rc<PolyType>) -> Rc<MonoType> {
    fn replace(map: &HashMap<u8, Rc<MonoType>>, t: Rc<MonoType>) -> Rc<MonoType> {
        use MonoType::*;
        use VarType::*;
        match &*t {
            Func { l, r } => {
                let l = replace(map, l.clone());
                let r = replace(map, r.clone());
                Func { l, r }.into()
            },
            Var { tvar } => match &*tvar.borrow() {
                Bound { typ }  => replace(map, typ.clone()),
                Unbound { id } => match map.get(id) {
                    Some(t_) => t_.clone(),
                    None => t.clone(),
                }
            }
            _ => t,
        }
    }

    let map = t.tvar_ids.iter()
        .map(|id| (*id, ctx.fresh_variable().into()))
        .collect();

    replace(&map, t.typ.clone())
}

fn generalize(typ: Rc<MonoType>) -> PolyType {
    fn find_vars(t: Rc<MonoType>) -> Vec<u8> {
        use MonoType::*;
        use VarType::*;
        match &*t {
            Bool => vec![],
            Func { l, r } => {
                let mut l_vars = find_vars(l.clone());
                let mut r_vars = find_vars(r.clone());
                l_vars.append(&mut r_vars);
                l_vars
            }
            Var { tvar } => match &*tvar.borrow() {
                Bound { typ } => find_vars(typ.clone()),
                Unbound { id } => vec![*id],
            }
        }
    }

    let tvar_ids = find_vars(typ.clone()).into();
    PolyType { tvar_ids, typ }
}

fn occurs(id: u8, t: Rc<MonoType>) -> bool {
    use MonoType::*;
    use VarType::*;
    match &*t {
        Bool => false,
        Func { l, r } => occurs(id, l.clone()) || occurs(id, r.clone()),
        Var { tvar } => match &*tvar.borrow() {
            Bound { typ } => occurs(id, typ.clone()),
            Unbound { id: id_ } => id == *id_,
        }
    }
}

fn unify(t0: Rc<MonoType>, t1: Rc<MonoType>) -> Result<()> {
    match (&*t0, &*t1) {
        (_, MonoType::Var { tvar }) => match *tvar.borrow_mut() {
            VarType::Bound { ref typ } => unify(t0, Rc::clone(&typ))?,
            ref mut t => { 
                if let VarType::Unbound { id } = t
                    && occurs(*id, Rc::clone(&t0))
                { 
                    return Err(TypeError::RecursiveType);
                }
                *t = VarType::Bound { typ: t0 };
            }
        }
        (MonoType::Var { .. }, _) => unify(t1, t0)?,
        (MonoType::Func { l: l_a, r: r_a }, MonoType::Func { l: l_b, r: r_b }) => {
            unify(l_a.clone(), l_b.clone())?;
            unify(r_a.clone(), r_b.clone())?;
        }
        (a, b) => if a != b {
            return Err(TypeError::TypeMismatch(Rc::clone(&t0), Rc::clone(&t1)))
        },
    }
    Ok(())
}

pub fn infer(ctx: &mut TypeContext, e: &Expression) -> Result<Rc<MonoType>> {
    use Expression::*;
    match e {
        Var { name } => {
            let t = ctx.lookup_sym(name);
            match t {
                Some(t) => Ok(instantiate(ctx, t)),
                None => Err(TypeError::VarNotFound(name.clone())),
            }
        }

        App { e0, e1 } => {
            let t0 = infer(ctx, e0)?;
            let t1 = infer(ctx, e1)?;
            let t2 = Rc::new(ctx.fresh_variable());
            
            let typ_func = MonoType::Func { l: t1, r: t2.clone() }.into();
            unify(t0, typ_func)?;
            Ok(t2)
        },

        Abs { name, e } => {
            let t0 = Rc::new(ctx.fresh_variable());
            let t0 = Rc::new(t0.as_poly());

            ctx.insert_sym(name.clone(), t0.clone());
            let t1 = infer(ctx, e)?;
            ctx.pop_sym();

            Ok(MonoType::Func { l: t0.typ.clone(), r: t1 }.into())
        },

        Let { name, e0, e1 } => {
            let t0 = infer(ctx, e0)?;
            let t0 = generalize(t0).into();

            ctx.insert_sym(name.clone(), t0);
            let t1 = infer(ctx, e1)?;
            ctx.pop_sym();

            Ok(t1)
        },

        True | False => Ok(MonoType::Bool.into())
    }
}
