mod parser;
mod interpreter;

fn main() {
    // Includes the contents of 'input.mopl' as a string at compile time.
    let input = include_str!("../hello.mus"); // Reads the content of the mopl file.

    // Parses the file content using 'parse_call' and unwraps the result.
    // The 'unwrap()' method extracts the parsed expression, assuming successful parsing.
    let (_, expr) = parser::parse_call(input).unwrap(); 

    // Calls the 'interpreter' function from the 'interpreter' module, passing the parsed expression.
    // This executes the logic defined in the interpreter.
    interpreter::interpreter(expr); 
}
