mod typ;
use typ::{PolyType, MonoType, VarType};

mod ctx;
use ctx::TypeContext;

use ast::Expression;
use thiserror::Error;

use std::collections::HashMap;
use std::rc::Rc;

fn main() {

}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("foo error")]
    Foo
}

type Result<T> = std::result::Result<T, TypeError>;

fn instantiate(ctx: &mut TypeContext, t: &PolyType) -> MonoType {
    fn replace(map: &HashMap<u8, MonoType>, t: &MonoType) -> MonoType {
        use MonoType::*;
        use VarType::*;
        match t {
            Bool => Bool,
            Func { l, r } => {
                let l = Rc::new(replace(map, l));
                let r = Rc::new(replace(map, r));
                Func { l, r }
            }
            Var { tvar } => match &**tvar {
                Bound { typ } => replace(map, typ),
                Unbound { tvar_id } => match map.get(&tvar_id) {
                    Some(t_) => t_.to_owned(),
                    None => t.clone(),
                }
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
        use VarType::*;
        match t {
            Bool => vec![],
            Func { l, r } => {
                let mut l_vars = find_var(l);
                let mut r_vars = find_var(r);
                l_vars.append(&mut r_vars);
                l_vars
            }
            Var { tvar } => match &**tvar {
                Bound { typ } => {todo!()}
                Unbound { tvar_id } => {todo!()}
            }
        }
    }
    todo!()
}

fn occurs(id: u8, level: u8, t: MonoType) -> bool {
    todo!()
}

fn unify(ty1: MonoType, ty2: MonoType) -> Result<()> {
    todo!()
}

fn infer(e: Expression) -> Result<MonoType> {
    todo!()
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
