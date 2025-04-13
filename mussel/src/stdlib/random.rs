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

use crate::parser::{Atom, Expr};
use std::collections::HashMap;
use rand::Rng;  // Ensure you have added rand = "0.8" (or a recent version) in Cargo.toml

// This function will be called when the user writes "include random"
pub fn load(context: &mut HashMap<String, Expr>) {
    // Insert a built-in function "rand" into the context.
    // Our built-in function takes exactly 2 numeric arguments: min and max.
    context.insert("rand".to_string(), Expr::Builtin(random_rand));
}

// The built-in random function implementation.
// It expects 2 arguments and returns a random integer between them.
pub fn random_rand(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 2 {
        panic!("rand expects 2 arguments: min and max");
    }
    let min = match &args[0] {
        Expr::Constant(Atom::Number(n)) => *n,
        _ => panic!("rand expects numeric arguments for min"),
    };
    let max = match &args[1] {
        Expr::Constant(Atom::Number(n)) => *n,
        _ => panic!("rand expects numeric arguments for max"),
    };
    let mut rng = rand::thread_rng();
    let random_val = rng.gen_range(min..=max);
    Expr::Constant(Atom::Number(random_val))
}
