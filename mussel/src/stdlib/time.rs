// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use crate::parser::{Atom, Expr};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// This function will be called when the user writes "include timings"
pub fn load(context: &mut HashMap<String, Expr>) {

    context.insert("time_ms".to_string(), Expr::Builtin(time_ms));
    context.insert("time_sec".to_string(), Expr::Builtin(time_sec), );
}

// Returns the current time in milliseconds since the Unix epoch.
pub fn time_ms(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 0 {
        panic!("time_ms expects 0 arguments");
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;

    Expr::Constant(Atom::Number(now))
}

// Returns the current time in seconds since the Unix epoch.
pub fn time_sec(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 0 {
        panic!("time_sec expects 0 arguments");
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();

    Expr::Constant(Atom::Float(now))
}
