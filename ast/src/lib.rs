mod parse;
pub use parse::parse;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // just a variable
    Var { name: Rc<str> },

    // function application: f e
    App { f: Box<Expression>, e: Box<Expression> },

    // lambda abstraction: \x . e
    Abs { name: Rc<str>, e: Box<Expression> },

    // let-in: let x = e0 in e1
    Let { name: Rc<str>, e0: Box<Expression>, e1: Box<Expression> },

    // booleans 
    True,
    False
}
