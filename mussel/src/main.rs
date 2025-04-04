mod parser;

fn main() {

    // Uses the macro 'include_str!' to include the contents of a file ('input.mopl') as a string literal at compile time.
    // The path '../input.mopl' is relative to the root of the project. This is often used for configuration or data files.
    let input = include_str!("../hello.mus"); // To read the content of the mopl file
    
    // Creates a string slice literal that contains a string wrapped in double quotes, matching the format that the parser expects.
    let string = "\"hello, world!\"";
    
    // The 'dbg!' macro prints the value of an expression along with its file and line number for debugging purposes.
    // Here, it calls the 'parse_string' function from the 'parse' module, passing the 'string' variable as input.
    // This helps verify the parsing result during development.
    dbg!(parser::parse_string(string));
}
