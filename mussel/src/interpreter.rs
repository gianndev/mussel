use crate::parser::{parse_interpolation, Atom, Expr, Operator};
use std::collections::HashMap;

pub fn interpreter(exprs: Vec<Expr>) {
    // Create new context for variables, then evaluate each expression
    let mut context = HashMap::new();
    for expr in exprs {
        interpreter_expr(expr, &mut context);
    }
}

fn interpreter_expr(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    // Evaluate expression
    match expr {
        Expr::Void | Expr::Closure(_, _) | Expr::Array(_) => expr,
        Expr::Return(expr) => Expr::Return(Box::new(interpreter_expr(*expr, context))),
        Expr::Constant(Atom::String(ref string)) => match parse_interpolation(string) {
            Ok((_, exprs)) => {
                match exprs.len() {
                    0 | 1 => return expr,
                    _ => {
                        let mut output = String::with_capacity(string.len());
                        for mut expr in exprs {
                            // Keep evaluating until it's just a string
                            loop {
                                let new_expr = interpreter_expr(expr.clone(), context);
                                if expr == new_expr {
                                    break;
                                } else {
                                    expr = new_expr;
                                }
                            }
                            output.push_str(&expr.to_string());
                        }
                        return Expr::Constant(Atom::String(output));
                    }
                }
            }
            _ => expr,
        },
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => context
                .get(name)
                .expect(&format!("{name} doesn't exist!"))
                .clone(),
            _ => expr,
        },
        Expr::Let(name, expr) => {
            let expr = interpreter_expr(*expr, context);
            context.insert(name, expr);
            Expr::Void
        }
        Expr::Compare(left, operator, right) => {
            let left = interpreter_expr(*left, context);
            let right = interpreter_expr(*right, context);
            match (&left, operator, &right) {
                (
                    Expr::Constant(Atom::Number(left)),
                    operator,
                    Expr::Constant(Atom::Number(right)),
                ) => match operator {
                    Operator::LessThan => Expr::Constant(Atom::Boolean(left < right)),
                    Operator::LessThanEqual => Expr::Constant(Atom::Boolean(left <= right)),
                    Operator::GreaterThan => Expr::Constant(Atom::Boolean(left > right)),
                    Operator::GreaterThanEqual => Expr::Constant(Atom::Boolean(left >= right)),
                },
                (
                    Expr::Constant(Atom::Float(left)),
                    operator,
                    Expr::Constant(Atom::Float(right)),
                ) => match operator {
                    Operator::LessThan => Expr::Constant(Atom::Boolean(left < right)),
                    Operator::LessThanEqual => Expr::Constant(Atom::Boolean(left <= right)),
                    Operator::GreaterThan => Expr::Constant(Atom::Boolean(left > right)),
                    Operator::GreaterThanEqual => Expr::Constant(Atom::Boolean(left >= right)),
                },
                _ => panic!("Can't compare {left} or {right}"),
            }
        }
        Expr::If(statement, then, otherwise) => {
            if let Expr::Constant(Atom::Boolean(value)) = interpreter_expr(*statement, context) {
                if value {
                    for expr in then {
                        interpreter_expr(expr, context);
                    }
                } else {
                    if let Some(body) = otherwise {
                        for expr in body {
                            interpreter_expr(expr, context);
                        }
                    }
                }
            }
            Expr::Void
        }
        Expr::Call(name, args) => {
            if name == "println" {
                for arg in args {
                    print!("{}", interpreter_expr(arg, context));
                }
                print!("\n");
            } else {
                match context.get(&name) {
                    Some(Expr::Closure(parameters, body)) => {
                        // Create new scope for context and run each line
                        let mut scope = context.clone();

                        for (parameter, arg) in parameters.into_iter().zip(args.into_iter()) {
                            let expr = interpreter_expr(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }

                        for expr in body {
                            if let Expr::Return(expr) = interpreter_expr(expr.clone(), &mut scope) {
                                return *expr;
                            }
                        }
                    }
                    _ => panic!("Function `{name}` doesn't exist."),
                }
            }
            Expr::Void
        }
        Expr::Function(name, args, body) => {
            context.insert(name, Expr::Closure(args, body));
            Expr::Void
        }
        Expr::For(name, collection, body) => {
            let array = interpreter_expr(*collection, context);
            match array {
                Expr::Array(items) => {
                    let mut scope = context.clone();
                    for item in items {
                        scope.insert(name.clone(), item);
                        for expr in &body {
                            interpreter_expr(expr.clone(), &mut scope);
                        }
                    }
                    Expr::Void
                }
                _ => panic!("Can't loop over `{array}`"),
            }
        }
        Expr::Get(name, index) => match context.get(&name) {
            Some(Expr::Array(items)) => {
                let expr = items[index].clone();
                return interpreter_expr(expr, context);
            }
            Some(invalid) => panic!("Expected array, got {invalid}"),
            None => panic!("Couldn't find {name}"),
        },
    }
}