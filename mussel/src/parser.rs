// Imports necessary modules and functions from the 'nom' library.
// 'nom' is a parsing library that helps construct parsers using combinators.
use nom::{
    branch::alt, bytes::complete::{tag, take_until}, character::complete::{alpha1, multispace0}, combinator::map, error::ParseError, multi::{many0, separated_list0}, sequence::{delimited, preceded, tuple}, IResult // Represents the result of parsing, including unparsed input and parsed value.
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
#[derive(Debug, Clone)]
pub enum Atom {
    String(String), // Represents an owned string within the 'Atom::String' variant.
    Name(String),
}

// Implements the 'Display' trait for the 'Atom' enum.
// Enables formatted printing of 'Atom' instances with macros like println.
impl std::fmt::Display for Atom {
    // Defines how 'Atom' is displayed as a formatted string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(string) => write!(f, "{string}"), // Formats the Atom::String variant by directly printing its inner string value.
            Atom::Name(string) => write!(f, "{string}"),
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

// Defines a parser for quoted strings.
pub fn parse_name(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |string: &str| Atom::Name(string.to_string()))(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_string, parse_name))(input)
}

// Defines an enumeration 'Expr' to represent expressions like variable declarations or function calls.
// The #[derive(Debug)] attribute allows Expr to be printed for debugging purposes.
#[derive(Debug, Clone)]
pub enum Expr {
    Void, // To return nothing
    Call(String, Vec<Expr>),
    Let(String, Box<Expr>),
    Constant(Atom),
    Closure(Vec<String>, Vec<Expr>),
}

// Defines a parser for function calls with arguments.
pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    // Matches the function name consisting of alphabetic characters.
    let parse_name = alpha1; 

    // Matches the argument enclosed in parentheses (e.g., (argument)).
    let parse_arg = delimited(tag("("), separated_list0(tag(","), parse_expr), tag(")"));

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
    let parse_equals = preceded(tag("="), ws(parse_expr));

    // Combines the variable name and value into a tuple.
    let parser = tuple((parse_name, parse_equals));

    // Transforms the parsed tuple into an Expr::Let variant with owned values.
    map(parser, |(name, value)| Expr::Let(name.to_string(), Box::new(value)))(input)
}

pub fn parse_closure(input: &str) -> IResult<&str, Expr> {
    let parse_name = map(alpha1, String::from);
    let parse_args = delimited(tag("|"), separated_list0(tag(","), parse_name), tag("|"));
    let parser = tuple((ws(parse_args), parse_expr));
    map(parser, |(args, expr)| Expr::Closure(args, vec![expr]))(input)
}

pub fn parse_constant(input: &str) -> IResult<&str, Expr> {
    map(parse_atom, Expr::Constant)(input)
}

// Defines a parser for multiple expressions, which can include both 'let' declarations and function calls.
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    // Parses zero or more expressions (both 'let' and function calls), allowing whitespace between them.
    alt((parse_let, parse_call, parse_constant, parse_closure))(input)
}

pub fn parser(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(ws(parse_expr))(input)
}