// Imports necessary modules and functions from the 'nom' library.
// 'nom' is a parsing library that helps construct parsers using combinators.
use nom::{
    IResult, // Represents the result of parsing, including unparsed input and parsed value.
    bytes::complete::{tag, take_until}, // Combinators for matching specific patterns in input.
    sequence::{delimited, tuple}, // Combines parsing patterns in specific sequences.
    combinator::map, // Allows transformation of parsed values.
    character::complete::alpha1 // Matches alphabetic characters in input.
};

// Defines an enumeration 'Atom' with one variant 'String'.
// The #[derive(Debug)] allows Atom to be printed for debugging.
#[derive(Debug)]
pub enum Atom {
    String(String), // Stores an owned String inside the Atom::String variant.
}

// Declares a function 'parse_string' that parses quoted strings in input.
pub fn parse_string(input: &str) -> IResult<&str, Atom> {

    // Creates a parser for matching strings surrounded by double quotes.
    let parser = delimited(tag("\""), take_until("\""), tag("\""));

    // Transforms the parsed result into an Atom::String variant.
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

// Defines an enumeration 'Expr' for representing expressions like function calls.
// The #[derive(Debug)] allows Expr to be printed for debugging.
#[derive(Debug)]
pub enum Expr {
    Call(String, Atom), // Represents a function call with a name and argument.
}

// Declares a function 'parse_call' for parsing function calls with arguments.
pub fn parse_call(input: &str) -> IResult<&str, Expr> {

    let parse_name = alpha1; // Matches the name of the function (alphabetic characters).

    // Matches the argument enclosed within parentheses, e.g., (argument).
    let parse_arg = delimited(tag("("), parse_string, tag(")"));

    // Combines function name and argument into a tuple (name, argument).
    let parser = tuple((parse_name, parse_arg));

    // Transforms the parsed tuple into an Expr::Call variant with owned values.
    map(parser, |(name, arg)| Expr::Call(name.to_string(), arg))(input)
}
