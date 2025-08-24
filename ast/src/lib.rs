use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // just a variable
    Var { name: Rc<str> },

    // function application: e0 e1
    App { e0: Box<Expression>, e1: Box<Expression> },

    // lambda abstraction: \x . e
    Abs { name: Rc<str>, e: Box<Expression> },

    // let-in: let x = e0 in e1
    Let { name: Rc<str>, e0: Box<Expression>, e1: Box<Expression> },

    // booleans 
    True,
    False
}
