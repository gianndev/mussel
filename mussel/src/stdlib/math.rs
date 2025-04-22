// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use crate::parser::{Atom, Expr};
use std::collections::HashMap;

// Loads math-related built-ins into the context.
pub fn load(context: &mut HashMap<String, Expr>) {
    context.insert("abs".to_string(), Expr::Builtin(math_abs));
    context.insert("sqrt".to_string(), Expr::Builtin(math_sqrt));
    context.insert("pow".to_string(), Expr::Builtin(math_pow));
}

// Returns the absolute value of a number.
//
// Usage: `abs(x)`
pub fn math_abs(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("abs expects 1 argument");
    }
    match &args[0] {
        Expr::Constant(Atom::Number(n)) => Expr::Constant(Atom::Number(n.abs())),
        Expr::Constant(Atom::Float(f)) => Expr::Constant(Atom::Float(f.abs())),
        _ => panic!("abs expects a numeric argument"),
    }
}

// Returns the square root of a number.
//
// Usage: `sqrt(x)`
pub fn math_sqrt(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("sqrt expects 1 argument");
    }
    match &args[0] {
        Expr::Constant(Atom::Number(n)) => {
            let result = (*n as f64).sqrt();
            Expr::Constant(Atom::Float(result))
        },
        Expr::Constant(Atom::Float(f)) => {
            let result = f.sqrt();
            Expr::Constant(Atom::Float(result))
        },
        _ => panic!("sqrt expects a numeric argument"),
    }
}

// Raises a number to a power.
//
// Usage: `pow(base, exponent)`
pub fn math_pow(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 2 {
        panic!("pow expects 2 arguments: base and exponent");
    }
    let base = match &args[0] {
        Expr::Constant(Atom::Number(n)) => *n as f64,
        Expr::Constant(Atom::Float(f)) => *f,
        _ => panic!("pow expects numeric arguments"),
    };
    let exponent = match &args[1] {
        Expr::Constant(Atom::Number(n)) => *n as f64,
        Expr::Constant(Atom::Float(f)) => *f,
        _ => panic!("pow expects numeric arguments"),
    };

    let result = base.powf(exponent);
    Expr::Constant(Atom::Float(result))
}