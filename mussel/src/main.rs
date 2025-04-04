mod parser;
mod interpreter;

fn main() {
    // Includes the contents of 'input.mopl' as a string at compile time.
    let input = include_str!("../hello.mus"); // Reads the content of the mopl file.

    // Attempts to parse the file content using 'parse_call'.
    // Handles the Result explicitly to manage errors gracefully.
    match parser::parse_call(input) {
        Ok((_, expr)) => {
            // Successfully parses the input and extracts the expression.
            interpreter::interpreter(expr); 
            // Passes the parsed expression to the 'interpreter' function for execution.
        },
        Err(err) => {
            // Prints an error message if parsing fails.
            eprintln!("Failed to parse input: {:?}", err);
        },
    }
}
