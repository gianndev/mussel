mod parser; // Links the 'parse' module, enabling access to its functions and types.

fn main() {

    // Includes the content of 'input.mopl' as a string at compile time.
    let input = include_str!("../hello.mus"); // Reads the content of the mopl file.

    // Defines a string slice representing a function call with an argument.
    let string = "println(\"hello, world!\")";

    // Prints the result of parsing the string using the 'dbg!' macro.
    dbg!(parser::parse_call(string)); // Helps debug and verify the parsed expression.
}
