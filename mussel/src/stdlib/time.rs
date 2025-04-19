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
use std::time::{SystemTime, UNIX_EPOCH};

// This function will be called when the user writes "include timings"
pub fn load(context: &mut HashMap<String, Expr>) {

    context.insert("time_ms".to_string(), Expr::Builtin(time_ms));
    context.insert("time_sec".to_string(), Expr::Builtin(time_sec), );
}

// Returns the current time in milliseconds since the Unix epoch.
pub fn time_ms(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 0 {
        panic!("get_time expects 0 arguments");
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
        panic!("get_time_seconds expects 0 arguments");
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64();

    Expr::Constant(Atom::Float(now))
}
