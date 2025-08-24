mod typ;
use typ::{PolyType, MonoType};

mod ctx;
use ctx::TypeContext;

use ast::Expression;
use thiserror::Error;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut ctx = TypeContext::new();
    let t = MonoType::Bound { typ: Rc::new(RefCell::new(MonoType::Bool))};
    print!("{}", t);
    if let MonoType::Bound { ref typ } = t {
        typ.replace(ctx.fresh_variable());
    }
    print!("{}", t);
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Variable '{name}' not found.")]
    VarNotFound { name: Rc<str> },
}

type Result<T> = std::result::Result<T, TypeError>;

fn instantiate(ctx: &mut TypeContext, t: &PolyType) -> MonoType {
    fn replace(map: &HashMap<u8, MonoType>, t: &MonoType) -> MonoType {
        use MonoType::*;
        match t {
            Bool => Bool,
            Func { l, r } => {
                let l = Rc::new(RefCell::new(replace(map, &l.borrow())));
                let r = Rc::new(RefCell::new(replace(map, &r.borrow())));
                Func { l, r }
            }
            Bound { typ } => replace(map, &typ.borrow()),
            Unbound { id } => match map.get(id) {
                Some(t_) => t_.clone(),
                None => t.clone()
            }
        }
    }

    let map = t.tvar_ids.iter()
        .map(|id| (*id, ctx.fresh_variable()))
        .collect();

    replace(&map, &t.typ)
}

fn generalize(ctx: &mut TypeContext, t: &MonoType) -> PolyType {
    fn find_var(t: &MonoType) -> Vec<u8> {
        use MonoType::*;
        match t {
            Bool => vec![],
            Func { l, r } => {
                let mut l_vars = find_var(&l.borrow());
                let mut r_vars = find_var(&r.borrow());
                l_vars.append(&mut r_vars);
                l_vars
            }
            Bound { typ } => { find_var(&typ.borrow()) }
            Unbound { id } => { vec![*id] }
        }
    }
    let tvar_ids = find_var(t).into();
    PolyType { tvar_ids, typ: t.clone() }
}

fn occurs(tvar_id: u8, t: &MonoType) -> bool {
    use MonoType::*;
    match t {
        Bool => false,
        Func { l, r } => occurs(tvar_id, &l.borrow()) || occurs(tvar_id, &r.borrow()),
        Bound { typ } => occurs(tvar_id, &typ.borrow()),
        Unbound { id } => tvar_id == *id,
    }
}

fn find(t: Rc<RefCell<MonoType>>) -> Rc<RefCell<MonoType>> {
    use MonoType::*;
    match &*t.clone().borrow() {
        Bound { typ } => find(typ.clone()),
        _ => t,
    }
}

fn unify(ctx: &mut TypeContext, ty1: &MonoType, ty2: &MonoType) -> Result<()> {
    todo!()
}

fn infer(ctx: &mut TypeContext, e: &Expression) -> Result<MonoType> {
    use Expression::*;
    match e {
        Var { name } => {
            let t = ctx.lookup_sym(name);
            match t {
                Some(t) => Ok(instantiate(ctx, &t)),
                None => Err(TypeError::VarNotFound { name: name.clone() }),
            }
        }

        App { e0, e1 } => {
            let t0 = infer(ctx, e0)?;
            let t1 = infer(ctx, e1)?;
            let t2 = ctx.fresh_variable();
            
            unify(ctx, &t0, &MonoType::Func{ l: Rc::new(RefCell::new(t1)), r: Rc::new(RefCell::new(t2.clone())) })?;
            Ok(t2)
        },

        Abs { name, e } => {
            let t0 = ctx.fresh_variable().as_poly();

            ctx.insert_sym(name.clone(), t0.clone());
            let t1 = infer(ctx, e)?;
            ctx.pop_sym();

            Ok(MonoType::Func { l: Rc::new(RefCell::new(t0.typ)), r: Rc::new(RefCell::new(t1)) })
        },

        Let { name, e0, e1 } => {
            let t0 = infer(ctx, e0)?;
            let t0 = generalize(ctx, &t0);

            ctx.insert_sym(name.clone(), t0);
            let t1 = infer(ctx, e1)?;
            ctx.pop_sym();

            Ok(t1)
        },

        True | False => Ok(MonoType::Bool)
    }
}

/*
 *
 * let id = \x. x in id true
 * t = infer-let:
 *     t = infer-abs:
 *         't0 = newvar
 *         ctx, x: 't0
 *         t = infer-var:
 *             ret 't0
 *          ret 't0 -> t
 *      ctx, id: forall t. t -> t
 *      t' = infer-app:
 *           t0 = infer-var:
 *                ret 't0
 *           t1 = infer-val:
 *                ret bool
 *           t' = newvar
 */
