use argh::FromArgs;
use color_eyre::{
    eyre::{eyre, WrapErr},
    Help, Result,
};

// Modules
mod interpreter;
mod parser;

#[derive(FromArgs)]
/// Interpreter for the salt language
struct Args {
    /// file to run
    #[argh(positional)]
    file: String,
}

fn main() -> Result<()> {
    // Nicer panics / error messages
    color_eyre::install()?;

    // Get file
    let Args { file } = argh::from_env();
    let input = std::fs::read_to_string(&file)
        .wrap_err(format!("Failed to read file: \"{file}\""))
        .suggestion("try using a file that exists")?;

    // Parse file and evaluate it
    let exprs =
        parser::parser(&input).map_err(|error| eyre!("Error occurred while parsing: {error:#?}"))?;
    interpreter::interpreter(exprs);

    // Success!
    Ok(())
}