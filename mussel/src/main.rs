// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use std::path;
use std::path::Path;
// Import the `FromArgs` trait from the `argh` crate for parsing command line arguments.
use argh::FromArgs;

// Import items from the `color_eyre` crate. The nested imports include:
// - `eyre` for creating error reports,
// - `WrapErr` to add context to errors,
// - `Help` for error suggestions, and
// - `Result` as a convenient alias for a Result type.
use color_eyre::{
    eyre::{eyre, WrapErr},
    Help, Result,
};
use crate::error::{FileError, FileIdentifier, FileSet, Reporter};

// Declare the modules that are defined in separate files.
// Rust will look for "interpreter.rs" and "parser.rs" in the same directory.
mod interpreter;
mod parser;
mod stdlib;
mod error;
mod lexer;
mod parser2;

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

    // Load the file specified in the command-line arguments into the `FileSet`.
    // If loading fails, print the error using the `Reporter` and return early.
    let file = match load_file(&mut files, &file) {
        Ok(file_id) => file_id,
        Err(error) => {
            let reporter = Reporter::new(files);
            reporter.report(error);
            return Ok(());
        }
    };

    let tokens = match lexer::lex(&files, file) {
        Ok(tokens) => tokens,
        Err(error) => {
            let reporter = Reporter::new(files);
            reporter.report(error);
            return Ok(())
        }
    };
    tokens.iter().for_each(|r| println!("{:?}", r));

    let expressions= match parser2::parser(file, &tokens) {
        Ok(tokens) => tokens,
        Err(error) => {
            let reporter = Reporter::new(files);
            reporter.report(error);
            return Ok(())
        }
    };
    expressions.iter().for_each(|r| println!("{:?}", r));

    // When the file is loaded successfully, retrieve its content.
    let file_content = files.get_content(file).unwrap_or_else(|| {
        // This should never happen. Every FileIdentifier should be valid.
        panic!("Failed to retrieve content");
    });

    // Call the parser from the `parser` module to turn the input into expressions.
    // If parsing fails, convert the error into an eyre error with detailed debugging information.
    let exprs =
        parser::parser(file_content).map_err(|error| eyre!("Error occurred while parsing: {error:#?}"))?;
    // Pass the parsed expressions to the interpreter to evaluate them.
    interpreter::interpreter(exprs);

    // Return success.
    Ok(())
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