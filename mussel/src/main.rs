mod parser; // Links the 'parser' module, providing access to its parsing functions and types.
mod interpreter; // Links the 'interpreter' module, enabling the execution of parsed expressions.

fn main() {
    // Reads the contents of the file 'hello.mus' at compile time and includes it as a string
    let input = include_str!("../hello.mus");

    // Parses the file content into a vector of expressions using 'parse_expr'.
    // The 'unwrap()' method is used to extract the parsing result, assuming it succeeds.
    let (_, expr) = parser::parse_expr(input).unwrap();

    // Prints the parsed expressions for debugging purposes.
    dbg!(&expr);
}
