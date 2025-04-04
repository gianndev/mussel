// Imports necessary modules and functions from the 'nom' library. 
// 'nom' is a parsing library in Rust that provides combinators for constructing parsers.
// 'IResult' is used to represent the result of parsing.
// 'bytes::complete::{tag, take_until}' are combinators to match specific patterns in the input.
// 'sequence::delimited' is a combinator that parses content surrounded by specific patterns.
use nom::{bytes::complete::{tag, take_until}, combinator::map, sequence::delimited, IResult};

// Defines an enumeration 'Atom', which has one variant called 'String'.
// 'String(String)' represents that this variant stores a value of the type String.
// Enumerations in Rust are used for grouping related values under one type.
pub enum Atom {
    String(String),
}

// Declares a function 'parse_string' that takes a string slice '&str' as input.
// The function returns an 'IResult', which includes the remaining unparsed input and the parsed value.
// The parsed value is an 'Atom' in this case.
pub fn parse_string(input: &str) -> IResult<&str, Atom> {

    // Defines the parser using the 'delimited' combinator from 'nom'.
    // The parser expects an initial double quote ('tag("\"")'), then matches everything 
    // until the next double quote ('take_until("\"")'), and finally expects a closing double quote ('tag("\"")').
    let parser = delimited(tag("\""), take_until("\""), tag("\""));

    // Applies the 'map' combinator to the parser. The 'map' combinator transforms the parsed result.
    // The closure '|string: &str| Atom::String(string.to_string())' takes the matched string slice and 
    // converts it into a String, then wraps it in the 'Atom::String' variant.
    // Finally, the function runs the parser on the provided input.
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}
