// Imports necessary modules and functions from the 'nom' library.
// 'nom' is a parsing library that helps construct parsers using combinators.
use nom::{
    bytes::complete::{tag, take_until}, // For matching specific bytes or patterns.
    character::complete::{alpha1, multispace0}, // For matching alphabetic characters and whitespace.
    combinator::map, // For transforming parsed values.
    error::ParseError, // For error handling during parsing.
    multi::many0, // For parsing zero or more repetitions of a pattern.
    sequence::{delimited, preceded, tuple}, // For combining multiple patterns in specific orders.
    branch::alt, // For trying multiple alternative parsers.
    IResult, // Represents the result of parsing, including unparsed input and parsed value.
};

// Defines a helper function for handling optional surrounding whitespace.
// Wraps a parser ('inner') so that it matches input with leading or trailing whitespace.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// Defines an enumeration 'Atom' to represent fundamental values.
// The #[derive(Debug)] attribute allows Atom to be printed for debugging purposes.
#[derive(Debug)]
pub enum Atom {
    String(String), // Represents an owned string within the 'Atom::String' variant.
}

// Implements the 'Display' trait for the 'Atom' enum.
// Enables formatted printing of 'Atom' instances with macros like println.
impl std::fmt::Display for Atom {
    // Defines how 'Atom' is displayed as a formatted string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"), 
            // Formats the Atom::String variant by directly printing its inner string value.
        }
    }
}

// Defines a parser for quoted strings.
pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    // Matches a string surrounded by double quotes and extracts the content between them.
    
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
    // Transforms the parsed string into an Atom::String variant.
}

// Defines an enumeration 'Expr' to represent expressions like variable declarations or function calls.
// The #[derive(Debug)] attribute allows Expr to be printed for debugging purposes.
#[derive(Debug)]
pub enum Expr {
    Let(String, Atom), // Represents a variable declaration with a name and a value.
    Call(String, Atom), // Represents a function call with a name and an argument.
}

// Defines a parser for function calls with arguments.
pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    // Matches the function name consisting of alphabetic characters.
    let parse_name = alpha1; 

    // Matches the argument enclosed in parentheses (e.g., (argument)).
    let parse_arg = delimited(tag("("), parse_string, tag(")"));

    // Combines the function name and argument into a tuple.
    let parser = tuple((parse_name, parse_arg));

    // Transforms the parsed tuple into an Expr::Call variant with owned values.
    map(parser, |(name, arg)| Expr::Call(name.to_string(), arg))(input)
}

// Defines a parser for variable declarations using the 'let' keyword.
pub fn parse_let(input: &str) -> IResult<&str, Expr> {
    // Matches a variable name preceded by the 'let' keyword, allowing surrounding whitespace.
    let parse_name = preceded(tag("let"), ws(alpha1));

    // Matches the equals sign and parses the value after it, allowing whitespace.
    let parse_equals = preceded(tag("="), ws(parse_string));

    // Combines the variable name and value into a tuple.
    let parser = tuple((parse_name, parse_equals));

    // Transforms the parsed tuple into an Expr::Let variant with owned values.
    map(parser, |(name, value)| Expr::Let(name.to_string(), value))(input)
}

// Defines a parser for multiple expressions, which can include both 'let' declarations and function calls.
pub fn parse_expr(input: &str) -> IResult<&str, Vec<Expr>> {
    // Parses zero or more expressions (both 'let' and function calls), allowing whitespace between them.
    many0(ws(alt((parse_let, parse_call))))(input)
}
