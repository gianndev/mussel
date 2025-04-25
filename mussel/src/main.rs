// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use std::path;
use std::path::Path;
// Import the `FromArgs` trait from the `argh` crate for parsing command line arguments.
use argh::FromArgs;

// Import items from the `color_eyre` crate. The nested imports include:
// - `eyre` for creating error reports,
// - `Result` as a convenient alias for a Result type.
use color_eyre::Result;
use crate::error::{FileError, FileIdentifier, FileSet, LError, Reporter};
use crate::expr::Expr;


mod interpreter;
mod stdlib;
mod error;
mod lexer;
mod parser;
mod expr;

// Derive the `FromArgs` trait automatically so that command-line arguments can be parsed.
// The doc-comment (triple slash) describes the application when running the help command.
#[derive(FromArgs)]
/// Interpreter for the salt language
struct Args {
    /// file to run
    // This attribute indicates that the field is a positional argument.
    #[argh(positional)]
    file: String, // The `file` field will store the path to the file to run.
}

fn main() -> Result<()> {
    // Install `color_eyre` which sets up enhanced error reporting (including colored output).
    // The `?` operator propagates any error that might occur during installation.
    color_eyre::install()?;

    // Parse command-line arguments from the environment and destructure to extract `file`.
    let Args { file } = argh::from_env();

    // Create a new `FileSet` instance to manage files.
    let mut files = FileSet::new();

    let parsed = match parse(&mut files, file) {
        Ok(file_id) => file_id,
        Err(error) => {
            let reporter = Reporter::new(files);
            reporter.report(error);
            return Ok(());
        }
    };

    // Pass the parsed expressions to the interpreter to evaluate them.
    interpreter::interpreter(parsed);

    // Return success.
    Ok(())
}

fn parse<P: AsRef<Path>>(files: &mut FileSet, file: P) -> Result<Vec<Expr>, Box<dyn LError>> {

    // Load the file specified in the command-line arguments into the `FileSet`.
    // If loading fails, print the error using the `Reporter` and return early.
    let file = load_file(files, &file).map_err(|e| error::boxed(e))?;

    let tokens = lexer::lex(files, file).map_err(|e| error::boxed(e))?;

    let expressions= parser::parser(file, &tokens)?;

    Expr::from_parser(&files, file, expressions).map_err(|e| error::boxed(e))
}


fn load_file<P: AsRef<Path>>(files: &mut FileSet, path: P) -> Result<FileIdentifier, FileError> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path.to_path_buf());
    let path_qualified = path::absolute(path.to_path_buf()).unwrap_or(path.to_path_buf());
    match input {
        Ok(content) => {
            let file_id = files.add_file(path_qualified, content);
            Ok(file_id)
        }
        Err(err) => {
            Err(FileError::new(
                path_qualified,
                format!("Failed to read file: {}", err),
            ))
        }
    }

}
