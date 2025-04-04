use nom::{
    branch::alt,
    bytes::complete::{is_not, take_until},
    character::complete::{alpha1, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
};
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag, ParserExt};
use std::fmt;

type Span<'a> = &'a str;
type IResult<'a, O> = nom::IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

// Whitespace helper from nom docs
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<O>
where
    F: FnMut(&'a str) -> IResult<O>,
{
    delimited(multispace0, inner, multispace0)
}

// Atom parsers
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(i64),
    Float(f64),
    Boolean(bool),
    Name(String),
    String(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Number(number) => write!(f, "{number}"),
            Atom::Float(float) => write!(f, "{float}"),
            Atom::Boolean(boolean) => write!(f, "{boolean}"),
            Atom::Name(name) => write!(f, "{name}"),
            Atom::String(string) => write!(f, "{string}"),
        }
    }
}

fn parse_variable(input: &str) -> IResult<String> {
    let parser = alpha1.context("Expected name");
    map(parser, str::to_string)(input)
}

fn parse_name(input: &str) -> IResult<Atom> {
    map(parse_variable, Atom::Name)(input)
}

fn parse_string(input: &str) -> IResult<Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\"")).context("String is incomplete");
    map(parser, |string: &str| Atom::String(string.to_string()))(input)
}

fn parse_number(input: &str) -> IResult<Atom> {
    let parser = recognize(pair(opt(tag("-")), digit1));
    map(parser, |number: &str| Atom::Number(number.parse().unwrap()))(input)
}

fn parse_float(input: &str) -> IResult<Atom> {
    map(double, Atom::Float)(input)
}

fn parse_boolean(input: &str) -> IResult<Atom> {
    let parser = alt((map(tag("true"), |_| true), map(tag("false"), |_| false)));
    map(parser, Atom::Boolean)(input)
}

fn parse_atom(input: &str) -> IResult<Atom> {
    alt((
        parse_string,
        parse_float,
        parse_number,
        parse_boolean,
        parse_name,
    ))(input)
}

// Operator parsers
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

fn parse_operator(input: &str) -> IResult<Operator> {
    alt((
        map(tag("<="), |_| Operator::LessThanEqual),
        map(tag("<"), |_| Operator::LessThan),
        map(tag(">="), |_| Operator::GreaterThanEqual),
        map(tag(">"), |_| Operator::GreaterThan),
    ))(input)
}

// Expression parsers
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Void,
    Array(Vec<Expr>),
    Constant(Atom),
    Let(String, Box<Expr>),
    Call(String, Vec<Expr>),
    Compare(Box<Expr>, Operator, Box<Expr>),
    Closure(Vec<String>, Vec<Expr>),
    Function(String, Vec<String>, Vec<Expr>),
    If(Box<Expr>, Vec<Expr>, Option<Vec<Expr>>),
    Return(Box<Expr>),
    For(String, Box<Expr>, Vec<Expr>),
    Get(String, usize),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Constant(atom) => write!(f, "{atom}"),
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
            _ => Ok(()),
        }
    }
}

fn parse_constant(input: &str) -> IResult<Expr> {
    map(parse_atom, Expr::Constant)(input)
}

fn parse_compare_valid(input: &str) -> IResult<Expr> {
    alt((parse_call, parse_constant))(input)
}

fn parse_compare(input: &str) -> IResult<Expr> {
    let parser = tuple((parse_compare_valid, ws(parse_operator), parse_compare_valid));
    map(parser, |(left, operator, right)| {
        Expr::Compare(Box::new(left), operator, Box::new(right))
    })(input)
}

fn parse_let(input: &str) -> IResult<Expr> {
    let parse_statement = separated_pair(parse_variable, ws(tag("=")), parse_expr);
    let parser = preceded(ws(tag("let")), parse_statement).context("Invalid let statement");
    map(parser, |(name, expr)| Expr::Let(name, Box::new(expr)))(input)
}

fn parse_call(input: &str) -> IResult<Expr> {
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_expr)),
        tag(")"),
    );
    let parser = pair(parse_variable, parse_args).context("Invalid function call");
    map(parser, |(name, args)| Expr::Call(name, args))(input)
}

fn parse_function(input: &str) -> IResult<Expr> {
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_variable)),
        tag(")"),
    );
    let parse_body = delimited(tag("{"), ws(many0(parse_expr)), tag("}"));
    let parser = preceded(
        tag("fn"),
        tuple((ws(parse_variable), parse_args, ws(parse_body))),
    );
    map(parser, |(name, args, body)| {
        Expr::Function(name, args, body)
    })(input)
}

fn parse_closure(input: &str) -> IResult<Expr> {
    let parse_args = delimited(
        tag("|"),
        separated_list0(tag(","), ws(parse_variable)),
        tag("|"),
    );
    let parser = pair(parse_args, ws(parse_expr));
    map(parser, |(args, expr)| Expr::Closure(args, vec![expr]))(input)
}

fn parse_if(input: &str) -> IResult<Expr> {
    let parse_statement = preceded(tag("if"), ws(parse_expr));
    let parse_then = delimited(tag("{"), ws(many0(parse_expr)), tag("}"));
    let parse_else = preceded(
        ws(tag("else")),
        delimited(tag("{"), ws(many0(parse_expr)), tag("}")),
    );
    let parser = tuple((parse_statement, parse_then, opt(parse_else)));
    map(parser, |(statement, then, otherwise)| {
        Expr::If(Box::new(statement), then, otherwise)
    })(input)
}

fn parse_for(input: &str) -> IResult<Expr> {
    let parse_name = preceded(tag("for"), ws(parse_variable));
    let parse_collection = preceded(tag("in"), ws(parse_expr));
    let parse_body = delimited(tag("{"), ws(many0(parse_expr)), tag("}"));
    let parser = tuple((parse_name, parse_collection, parse_body));
    map(parser, |(name, collection, body)| {
        Expr::For(name, Box::new(collection), body)
    })(input)
}

fn parse_return(input: &str) -> IResult<Expr> {
    let parser = preceded(tag("return"), ws(parse_expr));
    map(parser, |expr| Expr::Return(Box::new(expr)))(input)
}

fn parse_array(input: &str) -> IResult<Expr> {
    let parser = delimited(
        tag("["),
        separated_list0(tag(","), ws(parse_expr)),
        tag("]"),
    );
    map(parser, Expr::Array)(input)
}

fn parse_get(input: &str) -> IResult<Expr> {
    let parse_number = map(digit1, |digits: &str| digits.parse::<usize>().unwrap());
    let parse_index = delimited(tag("["), parse_number, tag("]"));
    let parser = pair(parse_variable, parse_index);
    map(parser, |(name, index)| Expr::Get(name, index))(input)
}

fn parse_expr(input: &str) -> IResult<Expr> {
    alt((
        parse_return,
        parse_function,
        parse_for,
        parse_if,
        parse_let,
        parse_compare,
        parse_array,
        parse_closure,
        parse_get,
        parse_call,
        parse_constant,
    ))(input)
}

// Other parsers to be used in the evaluator, such as interpolation
pub fn parse_interpolation(input: &str) -> IResult<Vec<Expr>> {
    let parse_braces = delimited(tag("{"), ws(parse_expr), tag("}"));
    let parse_string = map(is_not("{"), |string: &str| {
        Expr::Constant(Atom::String(string.to_string()))
    });
    many0(alt((parse_braces, parse_string)))(input)
}

// Final parser which ties everything together
pub fn parser(input: &str) -> Result<Vec<Expr>, ErrorTree<&str>> {
    final_parser(many0(ws(parse_expr)))(input)
}