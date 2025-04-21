// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

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

// Declare the modules that are defined in separate files.
// Rust will look for "interpreter.rs" and "parser.rs" in the same directory.
mod interpreter;
mod parser;
mod stdlib;

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
    // Read the content of the file into a string.
    // If reading fails, add a custom error message and a suggestion using `wrap_err` and `suggestion`.
    let input = std::fs::read_to_string(&file)
        .wrap_err(format!("Failed to read file: \"{file}\""))
        .suggestion("try using a file that exists")?;

    // Call the parser from the `parser` module to turn the input into expressions.
    // If parsing fails, convert the error into an eyre error with detailed debugging information.
    let exprs =
        parser::parser(&input).map_err(|error| eyre!("Error occurred while parsing: {error:#?}"))?;
    // Pass the parsed expressions to the interpreter to evaluate them.
    interpreter::interpreter(exprs);

    // Return success.
    Ok(())
}
