mod parse;

mod typ;

mod ctx;
use ctx::TypeContext;

mod typck;

use ast::Expression;

fn main() {
    let mut ctx = TypeContext::new();
    // \f. \x. f (f x) : ('a -> 'a) -> 'a -> 'a
    let e = Expression::Abs {
        name: "f".into(),
        e: Expression::Abs {
            name: "x".into(),
            e: Expression::App {
                e0: Expression::Var { name: "f".into() }.into(),
                e1: Expression::App {
                    e0: Expression::Var { name: "f".into() }.into(),
                    e1: Expression::Var { name: "x".into() }.into(),
                }.into(),
            }.into()
        }.into(),
    };
    match typck::infer(&mut ctx, &e) {
        Ok(t)  => println!("{}", t),
        Err(e) => eprintln!("{}", e),
    }
}
