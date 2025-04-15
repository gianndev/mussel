// Copyright (c) 2025 Francesco Giannice
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::parser::{Atom, Expr};
use std::collections::HashMap;

// This function will be called when the user writes "include string"
pub fn load(context: &mut HashMap<String, Expr>) {
    // Insert built-in functions into the context
    context.insert("lowercase".to_string(), Expr::Builtin(string_lowercase));
    context.insert("uppercase".to_string(), Expr::Builtin(string_uppercase));
    context.insert("length".to_string(), Expr::Builtin(string_length));
}

// Convert a string to lowercase
pub fn string_lowercase(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("lowercase expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => {
            Expr::Constant(Atom::String(s.to_lowercase()))
        }
        _ => panic!("lowercase expects a string argument"),
    }
}

// Convert a string to uppercase
pub fn string_uppercase(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("uppercase expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => {
            Expr::Constant(Atom::String(s.to_uppercase()))
        }
        _ => panic!("uppercase expects a string argument"),
    }
}

// Get the length of a string
pub fn string_length(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("length expects 1 argument: a string");
    }
    match &args[0] {
        Expr::Constant(Atom::String(s)) => {
            Expr::Constant(Atom::Number(s.len() as i64))
        }
        _ => panic!("length expects a string argument"),
    }
}