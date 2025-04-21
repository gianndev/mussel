// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use crate::parser::{Atom, Expr};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

// Loads OS-related built-ins into the context.
pub fn load(context: &mut HashMap<String, Expr>) {
    context.insert("getcwd".to_string(), Expr::Builtin(os_getcwd));
    context.insert("listdir".to_string(), Expr::Builtin(os_listdir));
    context.insert("exists".to_string(), Expr::Builtin(os_exists));
}

// Returns the current working directory as a string.
// Usage: `getcwd()`
pub fn os_getcwd(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if !args.is_empty() {
        panic!("getcwd expects no arguments");
    }
    let cwd = env::current_dir().expect("Failed to get current directory");
    let cwd_str = cwd.to_str().expect("Invalid directory string").to_string();
    Expr::Constant(Atom::String(cwd_str))
}

// Lists all entries in the given directory.
// Usage: `listdir(path)`
// - Returns an array of strings containing the names of entries.
pub fn os_listdir(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("listdir expects 1 argument");
    }
    let path_str = match &args[0] {
        Expr::Constant(Atom::String(s)) => s,
        _ => panic!("listdir expects a string argument"),
    };
    let entries = fs::read_dir(path_str)
        .unwrap_or_else(|_| panic!("Cannot read directory: {}", path_str));
    let mut file_names = Vec::new();
    for entry in entries {
        let entry = entry.expect("Error reading directory entry");
        let file_name = entry.file_name().into_string().expect("Invalid filename");
        file_names.push(Expr::Constant(Atom::String(file_name)));
    }
    Expr::Array(file_names)
}

// Checks if a given path exists.
// Usage: `exists(path)`
// - Returns a boolean indicating whether the path exists.
pub fn os_exists(args: Vec<Expr>, _context: &mut HashMap<String, Expr>) -> Expr {
    if args.len() != 1 {
        panic!("exists expects 1 argument");
    }
    let path_str = match &args[0] {
        Expr::Constant(Atom::String(s)) => s,
        _ => panic!("exists expects a string argument"),
    };
    let exists = Path::new(path_str).exists();
    Expr::Constant(Atom::Boolean(exists))
}