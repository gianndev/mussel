use crate::parser::*;
use std::collections::HashMap;

// Declares the 'interpreter' function, which takes an expression ('Expr') as input.
pub fn interpreter(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Call(name, arg) => {
            if name == "println" {
                println!("{arg:?}");
            }
            Expr::Void  // This branch returns an Expr
        },
        Expr::Let(name, value) => {
            context.insert(name, *value);
            Expr::Void // Return a default value after side-effect
        },
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => context.get(name.as_str()).unwrap().clone(),
            _ => expr,  // Return the original expression
        },
        Expr::Void => Expr::Void,  // Handle the Void case explicitly
    }
}
