/*  Closely follows the inference rules in
 *  https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system#Algorithm_J
 *
 *  Also thank you to jfecher's algorithm-j implementation in ocaml
 *  https://github.com/jfecher/algorithm-j/blob/master/LICENSE
 *
 *  We use similar names from the above link:
 *  - Γ  (gamma) => TypeContext (ctx in comments)
 *  - ⊢ⱼ (turnstile with subscript j) => infer
 *  - Γ‾ (gamma bar) => generalize
 *  - inst   => instantiate
 *  - newvar => TypeContext::fresh_variable (ctx.fresh_variable in comments)
 */

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

/* Turns a polytype into a monotype by replacing the type variables bounded by
 * the forall into new unbound type variables.
 * E.g. forall b c. a -> b -> c => a -> d -> e */
fn instantiate(ctx: &mut TypeContext, t: Rc<PolyType>) -> Rc<MonoType> {
    /* replace each of the unbound type variables in 't' with the new one using
     * the mapping created below */
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

    /* for each type variable in the polytype, create a mapping between it and a
     * new unbound type variable */
    let map = t.tvar_ids.iter()
        .map(|id| (*id, ctx.fresh_variable().into()))
        .collect();

    replace(&map, t.typ.clone())
}

/* Turns a monotype into a polytype by finding all of the unbound type variables
 * in 'typ' and "binding" them to a forall.
 * E.g. a -> b -> c => forall a b c. a -> b -> c */
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

/* Checks whether or not an unbound variable appears in some monotype 't' */
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

/* Unification "solves" for type variables and checks if two monotypes have the
 * same type */
fn unify(t0: Rc<MonoType>, t1: Rc<MonoType>) -> Result<()> {
    match (&*t0, &*t1) {
        /*  The only notable case is when 't0' is an type variable is when it is
         *  unbound. If 't0' is unbound, then we should set it to whatever 't1'
         *  is since we're trying to "unify" them. */
        (MonoType::Var { tvar }, _) => {
            match &*tvar.borrow() {
                VarType::Bound { typ }  => return unify(typ.clone(), t1),

                /* We don't want to set a recursive binding of 't0' to itself */
                VarType::Unbound { id } => if occurs(*id, t1.clone()) { 
                    return Err(TypeError::RecursiveType);
                }
            }
            *tvar.borrow_mut() = VarType::Bound { typ: t1 };
        }

        /* If 't0' isn't a type variable but 't1' is, just swap it around. */
        (_, MonoType::Var { .. }) => unify(t1, t0)?,

        /* The types in two function type should match. */
        (MonoType::Func { l: l_a, r: r_a }, MonoType::Func { l: l_b, r: r_b }) => {
            unify(l_a.clone(), l_b.clone())?;
            unify(r_a.clone(), r_b.clone())?;
        }

        /* By now, both types are concrete types and we just have to check if
         * they're equal. */
        (a, b) => if a != b {
            return Err(TypeError::TypeMismatch(Rc::clone(&t0), Rc::clone(&t1)))
        },
    }
    Ok(())
}

/* This is the main part of Algorithm J. We closely follow the inference rules.
 * Some names in the inference rules are changed to fit the names in the
 * implementation */
pub fn infer(ctx: &mut TypeContext, e: &Expression) -> Result<Rc<MonoType>> {
    use Expression::*;
    match e {

        /*  name : s ∊ ctx
         *  t = instantiate s
         *  -----------------
         *  infer ctx name = t
         */
        Var { name } => {
            let s = ctx.lookup_sym(name);
            match s {
                Some(s) => {
                    let t = Ok(instantiate(ctx, s));
                    t
                }
                None => Err(TypeError::VarNotFound(name.clone())),
            }
        }

        /*  infer ctx f = t0
         *  infer ctx e = t1
         *  t2 = ctx.fresh_variable
         *  unify t0 (t1 -> t2)
         *  -----------------------
         *  infer ctx (f x) = t2
         */
        App { f, e } => {
            let t0 = infer(ctx, f)?;
            let t1 = infer(ctx, e)?;
            let t2 = Rc::new(ctx.fresh_variable());
            
            let typ_func = MonoType::Func { l: t1, r: t2.clone() }.into();
            unify(t0, typ_func)?;
            Ok(t2)
        },


        /*  t0 = ctx.fresh_variable
         *  infer (ctx + name : t0) e = t1
         *  ------------------------------
         *  infer ctx (\name. e) = t0 -> t1
         */
        Abs { name, e } => {
            let t0 = Rc::new(ctx.fresh_variable());
            let t0 = Rc::new(t0.as_poly());

            ctx.insert_sym(name.clone(), t0.clone());
            let t1 = infer(ctx, e)?;
            ctx.pop_sym();

            Ok(MonoType::Func { l: t0.typ.clone(), r: t1 }.into())
        },

        /*  infer ctx e0 = t0
         *  infer (ctx + name : generalize t0) e1 = t1
         *  ------------------------------------------
         *  infer ctx (let name = e0 in e1) = t1
         */
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
