extern crate nom;
extern crate nom_locate;

use std::error::Error;
use color_eyre::eyre::eyre;
use color_eyre::{Report, Section};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::bytes::complete::{take_while, take_while1};
use nom::character::complete::{char, multispace1};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom_locate::{position, LocatedSpan};
use nom_supreme::error::{BaseErrorKind, ErrorTree};
use nom_supreme::final_parser::final_parser;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Plus,              // '+'
    Minus,             // '-'
    Star,              // '*'
    RSlash,            // '/'
    LSlash,            // '\'
    Equals,            // '='
    EqualsEquals,      // '=='
    NotEquals,         // '!='
    LessThan,          // '<'
    GreaterThan,       // '>'
    LessThanEquals,    // '<='
    GreaterThanEquals, // '>='
    LParenthesis,      // '('
    RParenthesis,      // ')'
    LBracket,          // '['
    RBracket,          // ']'
    LBrace,            // '{'
    RBrace,            // '}'
    Comma,             // ','
    Bar,               // '|'
    Fn,                // 'fn'
    Include,           // 'include'
    For,               // 'for'
    In,                // 'in'
    If,                // 'if'
    Else,              // 'else'
    Until,             // 'until'
    Let,               // 'let'
    Return,            // 'return'

    Ignore, //Comment and Whitespace (should be filtered before lexing)

    Integer(),
    Float(),
    Boolean(),
    String(),
    Identifier(),
}

#[derive(Debug)]
pub(crate) struct TokenRecord {
    token_type: Token,
    offset: usize,
    length: usize,
}

//remembers location and file
pub type Span<'a> = LocatedSpan<&'a str, &'a str>;
type IResult<'a, O> = nom::IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn identifier(input: Span) -> IResult<Token> {
    let (input, ident) =
        recognize(pair(take_while1(is_ident_start), take_while(is_ident_char)))(input)?;

    let token = match *ident.fragment() {
        "fn" => Token::Fn,
        "include" => Token::Include,
        "for" => Token::For,
        "in" => Token::In,
        "if" => Token::If,
        "else" => Token::Else,
        "until" => Token::Until,
        "let" => Token::Let,
        "return" => Token::Return,
        "true" => Token::Boolean(),
        "false" => Token::Boolean(),
        _ => Token::Identifier(),
    };

    Ok((input, token))
}

fn whitespace(input: Span) -> IResult<Token> {
    map(multispace1, |_| Token::Ignore)(input)
}

fn comment(input: Span) -> IResult<Token> {
    alt((
        map(delimited(tag("//"), take_until("\n"), tag("\n")), |_| {
            Token::Ignore
        }),
        map(delimited(tag("/*"), take_until("*/"), tag("*/")), |_| {
            Token::Ignore
        }),
    ))(input)
}

fn number(input: Span) -> IResult<Token> {
    map_res(
        recognize(pair(digit1, opt(pair(char('.'), digit1)))),
        |num_str: Span| {
            if num_str.contains('.') {
                Ok::<Token, ErrorTree<&'_ str>>(Token::Float())
            } else {
                Ok::<Token, ErrorTree<&'_ str>>(Token::Integer())
            }
        },
    )(input)
}

fn string_literal(input: Span) -> IResult<Token> {
    let (input, _) = delimited(char('"'), take_while(|c| c != '"'), char('"'))(input)?;
    Ok((input, Token::String()))
}

fn simple_token(input: Span) -> IResult<Token> {
    alt((
        map(tag("=="), |_| Token::EqualsEquals),
        map(tag("!="), |_| Token::NotEquals),
        map(tag("<="), |_| Token::LessThanEquals),
        map(tag(">="), |_| Token::GreaterThanEquals),
        map(tag("+"), |_| Token::Plus),
        map(tag("-"), |_| Token::Minus),
        map(tag("*"), |_| Token::Star),
        map(tag("/"), |_| Token::RSlash),
        map(tag("\\"), |_| Token::LSlash),
        map(tag("="), |_| Token::Equals),
        map(tag("<"), |_| Token::LessThan),
        map(tag(">"), |_| Token::GreaterThan),
        map(tag("("), |_| Token::LParenthesis),
        map(tag(")"), |_| Token::RParenthesis),
        map(tag("["), |_| Token::LBracket),
        map(tag("]"), |_| Token::RBracket),
        map(tag("{"), |_| Token::LBrace),
        map(tag("}"), |_| Token::RBrace),
        map(tag(","), |_| Token::Comma),
        map(tag("|"), |_| Token::Bar),
    ))(input)
}

fn one_token(input: Span) -> IResult<TokenRecord> {
   let start = position(input)?.0;
    let result = alt((
        whitespace,
        comment,
        simple_token,
        number,
        string_literal,
        identifier,
    ))(input)?;
    let as_record = TokenRecord {
        token_type: result.1,
        offset: start.location_offset(),
        length: result.0.location_offset() - start.location_offset(),
    };

    Ok((result.0, as_record))
}

// top parser: parse *many* tokens
fn tokens(input: Span) -> IResult<Vec<TokenRecord>> {
    many0(one_token)(input).map(|(span, vec)| {

        let filtered = vec
            .into_iter()
            .filter(|x| match x.token_type {
                Token::Ignore => false,
                _ => true,
            }).collect();
        (span, filtered)
    })
}

pub fn lex(input: Span) -> Result<Vec<TokenRecord>, Report> {
    final_parser(tokens)(input).map_err(|e| format_error(e))
}

fn format_error(error: ErrorTree<Span>) -> Report {
    let base = match error {
        ErrorTree::Base { location, kind } => {
            generate_report(location, kind)
        }
        ErrorTree::Stack { base, contexts: _contexts } => {
            format_error(*base)
        }

        ErrorTree::Alt(other) => {
            for x in other {
                return format_error(x);
            }
            eyre!("Error occurred while parsing")
        }
    };
    base
}

fn generate_report(location: Span, kind: BaseErrorKind<&str, Box<dyn Error+Send+Sync>>) -> Report {
    let file = location.extra;
    let kind = kind.to_string();
    let line = location.location_line();
    let column = location.get_column();

    let code = location.into_fragment().to_string();
    let code = format!("{:?}", code.to_string());
    eyre!("Error occurred while parsing '{kind}' {file}:{line}:{column}").with_section(move || code)

}