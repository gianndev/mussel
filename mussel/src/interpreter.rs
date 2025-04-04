use crate::parser::*;
use std::{collections::HashMap, env::Args};

// Declares the 'interpreter' function, which takes an expression ('Expr') as input.
pub fn interpreter(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Call(name, args) => {
            if name == "println" {
                for arg in args {
                    let arg = interpreter(arg, context);
                    print!("{arg:?}");
                }
            } else {
                match context.get(&name) {
                    Some(Expr::Closure(parameters, body)) => {
                        let mut scope = context.clone();
                        for (parameter, arg) in parameters.into_iter().zip(args.into_iter()) {
                            let expr = interpreter(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }
                        for expr in body {
                            interpreter(expr.clone(), &mut scope);
                        }
                    }
                    _ => panic!("Function '{name}' doesn't exist"),
                }
            }
            Expr::Void
        },
        Expr::Let(name, value) => {
            context.insert(name, *value);
            Expr::Void // Return a default value after side-effect
        },
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => context.get(name).unwrap().clone(),
            _ => expr,  // Return the original expression
        },
        Expr::Void | Expr::Closure(_, _) => expr,
        Expr::Function(name, args, body) => {
            context.insert(name, Expr::Closure(args, body));
            Expr::Void
        }
    }
}
