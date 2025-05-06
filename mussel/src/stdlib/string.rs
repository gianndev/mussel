// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use core::panic;
use std::collections::HashMap;
use crate::expr::{Atom, Expr};

// This function will be called when the user writes "include string"
pub fn load(context: &mut HashMap<String, Expr>) {
    // Insert built-in functions into the context
    context.insert("lowercase".to_string(), Expr::Builtin(string_lowercase));
    context.insert("uppercase".to_string(), Expr::Builtin(string_uppercase));
    context.insert("length".to_string(), Expr::Builtin(string_length));
    context.insert("split".to_string(), Expr::Builtin(string_split));
    context.insert("reverse".to_string(), Expr::Builtin(string_reverse));
    context.insert("trim".to_string(), Expr::Builtin(string_trim));
    context.insert("ltrim".to_string(), Expr::Builtin(string_ltrim));
    context.insert("rtrim".to_string(), Expr::Builtin(string_rtrim));
}

// Convert a string to lowercase
pub fn string_lowercase(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("lowercase expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => Expr::Constant(Atom::String(s.to_lowercase())),
        _ => panic!("lowercase expects a string argument"),
    }
}

// Convert a string to uppercase
pub fn string_uppercase(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("uppercase expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => Expr::Constant(Atom::String(s.to_uppercase())),
        _ => panic!("uppercase expects a string argument"),
    }
}

// Get the length of a string
pub fn string_length(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("length expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => Expr::Constant(Atom::Number(s.len() as i64)),
        _ => panic!("length expects a string argument"),
    }
}

// Split a string by another
pub fn string_split(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 2 {
        panic!("split expects 2 arguments: strings");
    }
    match (&args[0], &args[1]) {
        (Expr::Constant(Atom::String(s1)), Expr::Constant(Atom::String(s2))) => Expr::Array(
            s1.split(s2.as_str())
                .map(|s| Expr::Constant(Atom::String(s.to_string())))
                .collect(),
        ),
        _ => panic!("split expects strings as arguments"),
    }
}

// Reverse a string
pub fn string_reverse(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("reverse expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => {
            Expr::Constant(Atom::String(s.as_str().chars().rev().collect()))
        }
        _ => panic!("reverse expects a string argument"),
    }
}

//Remove whitespace from both ends
pub fn string_trim(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("trim expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => Expr::Constant(Atom::String(s.trim().to_string())),
        _ => panic!("trim expects a string as argument"),
    }
}

//Remove leading whitespace
pub fn string_ltrim(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("ltrim expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => Expr::Constant(Atom::String(s.trim_start().to_string())),
        _ => panic!("ltrim expects a string argument"),
    }
}

//Remove trailing whitespace
pub fn string_rtrim(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("rtrim expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => Expr::Constant(Atom::String(s.trim_end().to_string())),
        _ => panic!("rtrim expects a string argument"),
    }
}
