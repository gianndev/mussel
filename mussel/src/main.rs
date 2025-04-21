// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use std::error::Error;
use std::path;
// Import the `FromArgs` trait from the `argh` crate for parsing command line arguments.
use argh::FromArgs;
// Import items from the `color_eyre` crate. The nested imports include:
// - `eyre` for creating error reports,
// - `WrapErr` to add context to errors,
// - `Help` for error suggestions, and
// - `Result` as a convenient alias for a Result type.
use color_eyre::{eyre::{eyre, WrapErr}, Help, Report, Result};
use nom_locate::LocatedSpan;
use nom_supreme::error::{BaseErrorKind, ErrorTree};
use crate::error::{FilePath, FileSet, LError, Reporter, TokenError};
use crate::lexer::{Span, TokenRecord};

// Declare the modules that are defined in separate files.
// Rust will look for "interpreter.rs" and "parser.rs" in the same directory.
mod interpreter;
mod parser;
mod stdlib;
mod lexer;
mod parser2;
mod error;

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


    let absolute_path = path::absolute(&file)
        .map_err(|_| eyre!("Failed to get absolute path"))?;
    let mut files = FileSet::new();
    let file = files.add_file(&absolute_path, input);

    let tokens = match lexer::lex(&files, file) {
        Ok(tokens) => tokens,
        Err(error) => {
            let reporter = Reporter::new(&files);
            reporter.report(error);
            return Ok(())
        }
    };
    tokens.iter().for_each(|r| println!("{:?}", r));

    let expressions= match parser2::parser(file, &tokens) {
        Ok(tokens) => tokens,
        Err(error) => {
            let reporter = Reporter::new(&files);
            reporter.report(error);
            return Ok(())
        }
    };

   //
   //  let file = file.as_str();
   //  let program = input.as_str();
   //  let input = LocatedSpan::new_extra(program, file);
   //
   //  let now = std::time::Instant::now();
   //  let result = lexer::lex(input)?;
   // // result.iter().for_each(|r| println!("{:?}", r));
   //  println!("Lexer took: {:?}", now.elapsed());
   //
   //  println!("Token count: {}", result.len());
   //  let now2 = std::time::Instant::now();
   //  let expressions = parser2::parser(&result).map_err(|e| e.create_report(file, program))?;
   //  // expressions.iter().for_each(|r| println!("{:?}", r));
   //  println!("Parser took: {:?}", now2.elapsed());
   //  println!("exprs: {:?}", expressions.len());
   //
   //  println!("both took: {:?}", now.elapsed());
   //
   //  // Call the parser from the `parser` module to turn the input into expressions.
   //  // If parsing fails, convert the error into an eyre error with detailed debugging information.
   //  let now = std::time::Instant::now();
   //  let exprs =
   //      parser::parser(&input).map_err(|error| format_error(error))?;
   //  println!("old took: {:?}", now.elapsed());
   //  println!("exprs: {:?}", exprs.len());

    // Return success.
    Ok(())
}

fn entry() {
    let files = FileSet::new();
    let reporter = Reporter::new(&files);
    let error = get_error();
    reporter.report(error);
}

fn get_error() -> Box<dyn LError> {
    todo!()
}


fn format_error(error: ErrorTree<&str>) -> Report {
    let base = match error {
        ErrorTree::Base { location, kind } => {
            generate_report(location, kind)
        }
        ErrorTree::Stack { base, contexts: _contexts } => {
            format_error(*base)
        }

        ErrorTree::Alt(other) => {
            for x in other {
                return format_error(x);
            }
            eyre!("Error occurred while parsing")
        }
    };
    base
}

fn generate_report(location: &str, kind: BaseErrorKind<&str, Box<dyn Error+Send+Sync>>) -> Report {
    let kind = kind.to_string();
    let first_20 = &location.chars().take(40).collect::<String>();
    eyre!("Error occurred while parsing '{kind}' {first_20}")
}
