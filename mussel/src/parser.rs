// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

// Import various combinators and types from the `nom` crate which is used for parsing.
use nom::{
    Parser,
    branch::alt, // `alt` tries multiple parsers in order until one succeeds.
    bytes::complete::{is_not, take_until, take_while, tag}, // Parsers for matching parts of a string.
    character::complete::{alpha1, digit1, line_ending, alphanumeric1}, // Parsers for alphabetic characters, digits, and whitespace.
    combinator::{map, opt, recognize}, // `map` transforms parser output; `opt` makes a parser optional; `recognize` returns the matched slice.
    multi::{many0, separated_list0, fold_many0}, // `many0` for zero or more occurrences; `separated_list0` for a list with a separator.
    number::complete::double, // Parser to match a floating point number.
    sequence::{delimited, pair, preceded, separated_pair, tuple, terminated}, // Combinators for parsing sequences.
};
// Import enhanced error reporting and additional parser functionality from the `nom_supreme` crate.
use nom_supreme::{
    error::ErrorTree, // An error type that provides detailed error trees.
    final_parser::final_parser, // Helper to run a parser until the end of input.
    ParserExt, // Extension traits for parsers.
};

// Import standard formatting traits for implementing display functionality.
use std::fmt;

// Define a type alias `Span` for a string slice, which is used as the input type for our parsers.
type Span<'a> = &'a str;
// Define a type alias for parser results that includes our custom error type.
type IResult<'a, O> = nom::IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

// A helper function to wrap another parser with optional whitespace on both sides.
// It uses `delimited` to apply `multispace0` before and after the inner parser.
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<O>
where
    F: FnMut(&'a str) -> IResult<O>,
{
    delimited(skip_ws_comments, inner, skip_ws_comments)
}

fn skip_ws_comments(input: Span) -> IResult<()> {
    let line_comment = preceded(
        tag("//"),
        terminated(
            take_while(|c| c != '\n' && c != '\r'),
            opt(line_ending)
        )
    ).map(|_| ());
    
    // Use multispace1 to ensure at least one whitespace character is consumed.
    let whitespace = nom::character::complete::multispace1.map(|_| ());
    
    // Try to consume a comment before falling back to whitespace.
    let mut parser = many0(alt((
        line_comment,
        whitespace,
    )));
    
    let (input, _) = parser(input)?;
    Ok((input, ()))
}

// Define the `Atom` enum representing the basic literal values in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(i64),   // Represents an integer.
    Float(f64),    // Represents a floating-point number.
    Boolean(bool), // Represents a boolean value.
    Name(String),  // Represents an identifier.
    String(String),// Represents a string literal.
}

// Implement the Display trait for Atom so that it can be converted to a user-friendly string.
impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Number(number) => write!(f, "{number}"), // Write the number.
            Atom::Float(float) => write!(f, "{float}"), // Write the float.
            Atom::Boolean(boolean) => write!(f, "{boolean}"), // Write the boolean.
            Atom::Name(name) => write!(f, "{name}"), // Write the name.
            Atom::String(string) => write!(f, "{string}"), // Write the string.
        }
    }
}

/// Parse an identifier: starts with [A-Za-z_] then zero or more [A-Za-z0-9_],
fn parse_identifier(input: &str) -> IResult<String> {
    // first char must be a letter or underscore
    let first = alt((alpha1, tag("_")));
    // subsequent chars can be alphanumeric or underscore
    let rest = many0(alt((alphanumeric1, tag("_"))));
    // combine and give a useful error if it fails
    let parser = recognize(pair(first, rest))
        .context("expected identifier starting with letter or underscore");
    // slice to owned
    map(parser, |s: &str| s.to_string())(input)
}

// Parse a name by wrapping the variable parser into an Atom::Name.
fn parse_name(input: &str) -> IResult<Atom> {
    map(parse_identifier, Atom::Name)(input)
}

// Parse a string literal enclosed in double quotes.
fn parse_string(input: &str) -> IResult<Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""))
        .context("String is incomplete"); // Ensure that the string is properly closed.
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

// Parse an integer number, possibly with a leading minus sign.
fn parse_number(input: &str) -> IResult<Atom> {
    let parser = recognize(pair(opt(tag("-")), digit1)); // Recognize an optional "-" followed by digits.
    map(parser, |number: &str| Atom::Number(number.parse().unwrap()))(input)
}

// Parse a floating point number using the `double` parser.
fn parse_float(input: &str) -> IResult<Atom> {
    map(double, Atom::Float)(input)
}

// Parse a boolean literal, either "true" or "false".
fn parse_boolean(input: &str) -> IResult<Atom> {
    let parser = alt((
        map(tag("true"), |_| true),
        map(tag("false"), |_| false),
    ));
    map(parser, Atom::Boolean)(input)
}

// Attempt to parse any kind of atom by trying each parser in order.
fn parse_atom(input: &str) -> IResult<Atom> {
    alt((
        parse_string,
        parse_number,
        parse_float,
        parse_boolean,
        parse_name,
    ))(input)
}

// Define an enum for comparison operators.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equal,            // Represents "=="
    NotEqual,         // Represents "!="
    LessThan,         // Represents "<"
    LessThanEqual,    // Represents "<="
    GreaterThan,      // Represents ">"
    GreaterThanEqual, // Represents ">="
}

// Parse an operator by trying to match each literal string.
fn parse_operator(input: &str) -> IResult<Operator> {
    alt((
        map(tag("=="), |_| Operator::Equal),
        map(tag("!="), |_| Operator::NotEqual),
        map(tag("<="), |_| Operator::LessThanEqual),
        map(tag("<"), |_| Operator::LessThan),
        map(tag(">="), |_| Operator::GreaterThanEqual),
        map(tag(">"), |_| Operator::GreaterThan),
    ))(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

// Define an enum for expressions in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Void, // Represents a no-value or empty expression.
    Array(Vec<Expr>), // Represents an array of expressions.
    Constant(Atom), // Wraps an Atom literal as an expression.
    Let(String, Box<Expr>), // A let-binding that associates a name with an expression (boxed to allow recursion).
    Call(String, Vec<Expr>), // A function call with a name and arguments.
    Compare(Box<Expr>, Operator, Box<Expr>), // A comparison between two expressions.
    Closure(Vec<String>, Vec<Expr>), // A closure with parameters and a body of expressions.
    Function(String, Vec<String>, Vec<Expr>), // A named function definition.
    If(Box<Expr>, Vec<Expr>, Option<Vec<Expr>>), // An if statement with an optional else branch.
    Return(Box<Expr>), // A return expression.
    For(String, Box<Expr>, Vec<Expr>), // A for loop iterating over a collection.
    Get(String, usize), // Access an element in an array by name and index.
    Until(Box<Expr>, Vec<Expr>), // An until loop: execute the body until the condition becomes true.
    Binary(Box<Expr>, BinOp, Box<Expr>), // Binary arithmetic expression.
    Include(String),
    Builtin(fn(Vec<Expr>, &mut std::collections::HashMap<String, Expr>) -> Expr),
}

// Implement Display for Expr so that it can be printed.
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // For constant expressions, delegate to Atom's display.
            Expr::Constant(atom) => write!(f, "{atom}"),
            // For arrays, manually format each element.
            Expr::Array(items) => {
                write!(f, "[")?;
                for (i, expr) in items.iter().enumerate() {
                    write!(f, "{expr}")?;
                    if i + 1 < items.len() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            _ => Ok(()), // For other expressions, do nothing.
        }
    }
}

// Parse a constant expression from an atom.
fn parse_constant(input: &str) -> IResult<Expr> {
    map(parse_atom, Expr::Constant)(input)
}

// Parse an expression that can validly be compared, like a function call or constant.
fn parse_compare_valid(input: &str) -> IResult<Expr> {
    alt((parse_call, parse_constant))(input)
}

// Parse a comparison expression.
fn parse_compare(input: &str) -> IResult<Expr> {
    // Parse a tuple containing left expression, operator (with surrounding whitespace), and right expression.
    let parser = tuple((parse_compare_valid, ws(parse_operator), parse_compare_valid));
    map(parser, |(left, operator, right)| {
        Expr::Compare(Box::new(left), operator, Box::new(right))
    })(input)
}

// Parse a let-binding expression.
fn parse_let(input: &str) -> IResult<Expr> {
    // Parse a pair: a variable, an "=" (with surrounding whitespace), and an expression.
    let parse_statement = separated_pair(parse_identifier, ws(tag("=")), parse_expr);
    // Precede the statement with the "let" keyword.
    let parser = preceded(ws(tag("let")), parse_statement)
        .context("Invalid let statement");
    map(parser, |(name, expr)| Expr::Let(name, Box::new(expr)))(input)
}

// Parse a function call.
fn parse_call(input: &str) -> IResult<Expr> {
    // Parse function call arguments: a list of expressions separated by commas inside parentheses.
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_expr)),
        tag(")"),
    );
    // Parse the function name followed by its arguments.
    let parser = pair(parse_identifier, parse_args).context("Invalid function call");
    map(parser, |(name, args)| Expr::Call(name, args))(input)
}

// Parse a function definition.
fn parse_function(input: &str) -> IResult<Expr> {
    // Parse the function parameters enclosed in parentheses.
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_identifier)),
        tag(")"),
    );
    // Parse the function body enclosed in curly braces.
    let parse_body = delimited(tag("{"), ws(many0(parse_expr)), tag("}"));
    // Expect the "fn" keyword, then the function name, parameters, and body.
    let parser = preceded(
        tag("fn"),
        tuple((ws(parse_identifier), parse_args, ws(parse_body))),
    );
    map(parser, |(name, args, body)| {
        Expr::Function(name, args, body)
    })(input)
}

// Parse a closure (anonymous function) expression.
fn parse_closure(input: &str) -> IResult<Expr> {
    // Parse closure parameters between pipes.
    let parse_args = delimited(
        tag("|"),
        separated_list0(tag(","), ws(parse_identifier)),
        tag("|"),
    );
    // Pair the parsed parameters with a following expression (the closure's body).
    let parser = pair(parse_args, ws(parse_expr));
    map(parser, |(args, expr)| Expr::Closure(args, vec![expr]))(input)
}

// Parse 'if', 'else if', 'else' statements.
fn parse_if(input: &str) -> IResult<Expr> {
    // Parse the "if" keyword, condition, and then branch.
    let (input, _) = tag("if")(input)?;
    let (input, _) = skip_ws_comments(input)?;
    let (input, condition) = parse_expr(input)?;
    let (input, _) = skip_ws_comments(input)?;
    let (input, then_block) = delimited(tag("{"), ws(many0(parse_expr)), tag("}"))(input)?;

    // Parse an optional else branch:
    // It can be either an "else if" chain or a plain "else" block.
    let (input, else_branch) = opt(preceded(
        ws(tag("else")),
        alt((
            // "else if": parse another if expression and wrap it in a vector.
            map(parse_if, |if_expr| vec![if_expr]),
            // "else": parse a normal block.
            delimited(tag("{"), ws(many0(parse_expr)), tag("}"))
        ))
    ))(input)?;

    Ok((input, Expr::If(Box::new(condition), then_block, else_branch)))
}

// Parse a for loop.
fn parse_for(input: &str) -> IResult<Expr> {
    // Parse the loop variable following the "for" keyword.
    let parse_name = preceded(tag("for"), ws(parse_identifier));
    // Parse the collection expression following the "in" keyword.
    let parse_collection = preceded(tag("in"), ws(parse_expr));
    // Parse the loop body enclosed in curly braces.
    let parse_body = delimited(tag("{"), ws(many0(parse_expr)), tag("}"));
    let parser = tuple((parse_name, parse_collection, parse_body));
    map(parser, |(name, collection, body)| {
        Expr::For(name, Box::new(collection), body)
    })(input)
}

// Parse a return statement.
fn parse_return(input: &str) -> IResult<Expr> {
    // Expect the "return" keyword followed by an expression.
    let parser = preceded(tag("return"), ws(parse_expr));
    map(parser, |expr| Expr::Return(Box::new(expr)))(input)
}

// Parse an array literal.
fn parse_array(input: &str) -> IResult<Expr> {
    // Parse a list of expressions separated by commas and enclosed in square brackets.
    let parser = delimited(
        tag("["),
        separated_list0(tag(","), ws(parse_expr)),
        tag("]"),
    );
    map(parser, Expr::Array)(input)
}

// Parse array element access, e.g., name[index].
fn parse_get(input: &str) -> IResult<Expr> {
    // Parse the index as a number.
    let parse_number = map(digit1, |digits: &str| digits.parse::<usize>().unwrap());
    // Expect the index to be enclosed in square brackets.
    let parse_index = delimited(tag("["), parse_number, tag("]"));
    // Pair the variable name with the index.
    let parser = pair(parse_identifier, parse_index);
    map(parser, |(name, index)| Expr::Get(name, index))(input)
}

fn parse_until(input: &str) -> IResult<Expr> {
    let (input, _) = tag("until")(input)?;
    let (input, _) = skip_ws_comments(input)?;
    let (input, condition) = parse_expr(input)?;
    let (input, _) = skip_ws_comments(input)?;
    let (input, body) = delimited(tag("{"), ws(many0(parse_expr)), tag("}"))(input)?;
    Ok((input, Expr::Until(Box::new(condition), body)))
}

// parse_factor: a number, a boolean, a string, a name, or a parenthesized expression.
fn parse_factor(input: &str) -> IResult<Expr> {
    alt((
        // Allow parenthesized expressions.
        delimited(tag("("), ws(parse_expr), tag(")")),
        // Otherwise, use an existing parser that produces a constant or a variable.
        parse_constant,
    ))(input)
}

// parse_term: handle multiplication and division.
fn parse_term(input: &str) -> IResult<Expr> {
    let (input, init) = parse_factor(input)?;
    fold_many0(
        pair(ws(alt((tag("*"), tag("/")))), parse_factor),
        || init.clone(),
        |acc, (op, value)| {
            let bin_op = match op {
                "*" => BinOp::Mul,
                "/" => BinOp::Div,
                _   => unreachable!(),
            };
            Expr::Binary(Box::new(acc), bin_op, Box::new(value))
        }
    )(input)
}

// parse_add_sub: handle addition and subtraction.
fn parse_add_sub(input: &str) -> IResult<Expr> {
    let (input, init) = parse_term(input)?;
    fold_many0(
        pair(ws(alt((tag("+"), tag("-")))), parse_term),
        || init.clone(),
        |acc, (op, value)| {
            let bin_op = match op {
                "+" => BinOp::Add,
                "-" => BinOp::Sub,
                _   => unreachable!(),
            };
            Expr::Binary(Box::new(acc), bin_op, Box::new(value))
        }
    )(input)
}


// parse_arithmetic: our top-level arithmetic parser.
fn parse_arithmetic(input: &str) -> IResult<Expr> {
    parse_add_sub(input)
}

fn parse_include(input: &str) -> IResult<Expr> {
    // "include" keyword followed by whitespace and a library name
    let (input, _) = tag("include")(input)?;
    let (input, _) = ws(nom::combinator::success(()))(input)?;
    let (input, lib_name) = parse_identifier(input)?;
    Ok((input, Expr::Include(lib_name)))
}

// Parse any expression by trying each possibility in order.
fn parse_expr(input: &str) -> IResult<Expr> {
    alt((
        parse_include,
        parse_return,
        parse_function,
        parse_for,
        parse_until,
        parse_if,
        parse_let,
        parse_compare,
        parse_array,
        parse_closure,
        parse_get,
        parse_call,
        parse_arithmetic,
        parse_constant,
    ))(input)
}

// Parse interpolation within strings for embedding expressions.
// It returns a list of expressions that are either literal strings or parsed expressions.
pub fn parse_interpolation(input: &str) -> IResult<Vec<Expr>> {
    // Parse expressions within curly braces.
    let parse_braces = delimited(tag("{"), ws(parse_expr), tag("}"));
    // Parse text segments that are not part of an interpolation.
    let parse_string = map(is_not("{"), |string: &str| {
        Expr::Constant(Atom::String(string.to_string()))
    });
    many0(alt((parse_braces, parse_string)))(input)
}

// The final parser function that ties all the sub-parsers together.
// It applies the expression parser repeatedly until the entire input is consumed.
pub fn parser(input: &str) -> Result<Vec<Expr>, ErrorTree<&str>> {
    final_parser(many0(ws(parse_expr)))(input)
}