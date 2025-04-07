// Copyright (c) 2025 Francesco Giannice
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Import definitions from the parser module that are needed for evaluation.
use crate::parser::{parse_interpolation, Atom, Expr, Operator};
// Import the HashMap collection to maintain variable bindings.
use std::collections::HashMap;

// The main interpreter function that takes a vector of expressions.
pub fn interpreter(exprs: Vec<Expr>) {
    // Create a mutable context (a HashMap) to store variable bindings.
    let mut context = HashMap::new();
    // Evaluate each expression in order.
    for expr in exprs {
        interpreter_expr(expr, &mut context);
    }
}

// The recursive function that evaluates an expression given the current context.
// It returns a new expression representing the evaluated result.
fn interpreter_expr(expr: Expr, context: &mut HashMap<String, Expr>) -> Expr {
    // Use pattern matching on the expression to determine how to evaluate it.
    match expr {
        // For these variants, no further evaluation is needed so we return the expression as-is.
        Expr::Void | Expr::Closure(_, _) | Expr::Array(_) => expr,
        // For a return expression, evaluate the inner expression and re-wrap it.
        Expr::Return(expr) => Expr::Return(Box::new(interpreter_expr(*expr, context))),
        // If the expression is a string constant, attempt to parse interpolation.
        Expr::Constant(Atom::String(ref string)) => match parse_interpolation(string) {
            Ok((_, exprs)) => {
                // If there is zero or one interpolated expression, leave it unchanged.
                match exprs.len() {
                    0 | 1 => return expr,
                    _ => {
                        // Otherwise, create an output string and evaluate each interpolated expression.
                        let mut output = String::with_capacity(string.len());
                        for mut expr in exprs {
                            // Continue evaluating until the expression no longer changes.
                            loop {
                                let new_expr = interpreter_expr(expr.clone(), context);
                                if expr == new_expr {
                                    break;
                                } else {
                                    expr = new_expr;
                                }
                            }
                            // Append the evaluated expression's string representation.
                            output.push_str(&expr.to_string());
                        }
                        // Return a new constant with the fully interpolated string.
                        return Expr::Constant(Atom::String(output));
                    }
                }
            }
            // If interpolation parsing fails, return the original expression.
            _ => expr,
        },
        // If the constant is a name, look it up in the context.
        Expr::Constant(ref atom) => match atom {
            Atom::Name(name) => context
                .get(name)
                .expect(&format!("{name} doesn't exist!"))
                .clone(),
            _ => expr, // For other atoms, return as is.
        },
        // Evaluate a let-binding by evaluating the right-hand side and storing it in the context.
        Expr::Let(name, expr) => {
            let expr = interpreter_expr(*expr, context);
            context.insert(name, expr);
            // Let statements evaluate to void.
            Expr::Void
        }
        // Evaluate a comparison expression.
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
                    Operator::Equal => Expr::Constant(Atom::Boolean(left == right)),
                    Operator::NotEqual => Expr::Constant(Atom::Boolean(left != right)),
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
                    Operator::Equal => Expr::Constant(Atom::Boolean(left == right)),
                    Operator::NotEqual => Expr::Constant(Atom::Boolean(left != right)),
                },
                _ => panic!("Can't compare {left} or {right}"),
            }
        },
        // Evaluate an if-statement.
        Expr::If(statement, then, otherwise) => {
            // Evaluate the condition expecting a boolean result.
            if let Expr::Constant(Atom::Boolean(value)) = interpreter_expr(*statement, context) {
                if value {
                    // If true, evaluate all expressions in the "then" branch.
                    for expr in then {
                        interpreter_expr(expr, context);
                    }
                } else {
                    // If false, and an "else" branch exists, evaluate it.
                    if let Some(body) = otherwise {
                        for expr in body {
                            interpreter_expr(expr, context);
                        }
                    }
                }
            }
            // If the if-statement doesn't yield a value, return void.
            Expr::Void
        }
        // Evaluate a function call.
        Expr::Call(name, args) => {
            if name == "println" {
                // Special case for the built-in "println" function.
                for arg in args {
                    print!("{}", interpreter_expr(arg, context)); // Print each argument after evaluating.
                }
                print!("\n"); // Print a newline.
            } else {
                // For other function calls, look up the function in the context.
                match context.get(&name) {
                    Some(Expr::Closure(parameters, body)) => {
                        // Create a new scope by cloning the current context.
                        let mut scope = context.clone();

                        // Bind each function parameter to its corresponding argument.
                        for (parameter, arg) in parameters.into_iter().zip(args.into_iter()) {
                            let expr = interpreter_expr(arg, &mut scope);
                            scope.insert(parameter.clone(), expr);
                        }

                        // Evaluate each expression in the function body.
                        for expr in body {
                            // If a return is encountered, unwrap and return its value.
                            if let Expr::Return(expr) = interpreter_expr(expr.clone(), &mut scope) {
                                return *expr;
                            }
                        }
                    }
                    // If the function is not found or not callable, panic.
                    _ => panic!("Function `{name}` doesn't exist."),
                }
            }
            // Function calls return void if no explicit return value is encountered.
            Expr::Void
        }
        // Define a function by storing it as a closure in the context.
        Expr::Function(name, args, body) => {
            context.insert(name, Expr::Closure(args, body));
            Expr::Void
        }
        // Evaluate a for loop.
        Expr::For(name, collection, body) => {
            let array = interpreter_expr(*collection, context);
            match array {
                // Ensure the collection is an array.
                Expr::Array(items) => {
                    // Create a new scope for the loop.
                    let mut scope = context.clone();
                    for item in items {
                        // Bind the loop variable to the current item.
                        scope.insert(name.clone(), item);
                        // Evaluate each expression in the loop body.
                        for expr in &body {
                            interpreter_expr(expr.clone(), &mut scope);
                        }
                    }
                    Expr::Void
                }
                // Panic if the loop variable is not an array.
                _ => panic!("Can't loop over `{array}`"),
            }
        }
        // Evaluate an array element access.
        Expr::Get(name, index) => match context.get(&name) {
            Some(Expr::Array(items)) => {
                // Retrieve the element at the given index and evaluate it.
                let expr = items[index].clone();
                return interpreter_expr(expr, context);
            }
            Some(invalid) => panic!("Expected array, got {invalid}"),
            None => panic!("Couldn't find {name}"),
        },
    }
}
