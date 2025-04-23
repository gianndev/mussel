// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

extern crate nom;
extern crate nom_locate;

use std::error::Error;
use std::ops::Range;
use color_eyre::eyre::eyre;
use color_eyre::{Report, Section};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::bytes::complete::{take_while, take_while1};
use nom::character::complete::{char, multispace1};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt, recognize};
use nom::error::{ErrorKind, FromExternalError, ParseError};
use nom::InputLength;
use nom::multi::many0;
use nom::sequence::{delimited, pair, tuple};
use nom_locate::{position, LocatedSpan};
use nom_supreme::final_parser::{final_parser, ExtractContext};
use crate::error;
use crate::error::{FileIdentifier, FileSet};

/// Represents a type of Token
/// < byte, so trivial to copy
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    And,               // 'and'
    Or,                // 'or'
    Not,               // 'not'

    Ignore, //Comment and Whitespace (should be filtered before parsing)

    Integer,
    Float,
    Boolean,
    String,
    Identifier,
}

/// Represents an instance of a Token.
/// Stores the type of token, along with the source location where the token originated.
///
/// Implements Clone, so it can be cloned in the parser.
/// The record doesn't store data, only location, so copying is cheap.
///
#[derive(Debug, Clone)]
pub(crate) struct TokenRecord {
    pub(crate) token_type: Token,
    pub(crate) offset: usize,
    pub(crate) length: usize,
}

impl TokenRecord {

    /// Returns the underlying content of the token.
    /// use .chars() or just this?
    pub(crate) fn get_content<'a>(&self, input: &'a str) -> &'a str {
        &input[self.offset..self.offset + self.length]
    }


    pub fn range(&self) -> Range<usize> {
        self.offset..self.offset + self.length
    }
}

//remembers location and file
pub type Span<'a> = LocatedSpan<&'a str>;
type IResult<'a, O> = nom::IResult<Span<'a>, O, TokenError>;

#[derive(Debug, PartialEq)]
pub struct TokenError {
    pub index: usize,
}
impl<I: InputLength> ParseError<I> for TokenError {
    fn from_error_kind(input: I, _: ErrorKind) -> Self {
        TokenError {
            index: input.input_len()
        }
    }
    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
impl FromExternalError<LocatedSpan<&str>, TokenError> for TokenError {
    fn from_external_error(_: LocatedSpan<&str>, _: ErrorKind, other: TokenError) -> Self {
        TokenError {
            index: other.index,
        }
    }
}

impl ExtractContext<Span<'_>, TokenError> for TokenError {
    fn extract_context(self, input: Span<'_>) -> TokenError {
        TokenError {
            index: input.len() - self.index,
        }
    }
}

/// Identifier start check
/// Matches the following regex: [a-zA-Z_]
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}


/// Identifier continue check, (after first character)
/// Matches the following regex: [a-zA-Z0-9_]
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Tests for identifiers and keywords.
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
        "true" => Token::Boolean,
        "false" => Token::Boolean,
        "or" => Token::Or,
        "and" => Token::And,
        "not" => Token::Not,
        _ => Token::Identifier,
    };

    Ok((input, token))
}

/// Tests for whitespace. Will be filtered out
fn whitespace(input: Span) -> IResult<Token> {
    map(multispace1, |_| Token::Ignore)(input)
}

/// Tests for comments. Will be filtered out
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

/// Tests a number literal.
/// Matches the following regex: [0-9]+(\.[0-9]+)?
fn number(input: Span) -> IResult<Token> {
    map_res(
        recognize(tuple((opt(char('-')), digit1, opt(pair(char('.'), digit1))))),
        |num_str: Span| {
            if num_str.contains('.') {
                Ok::<Token, TokenError>(Token::Float)
            } else {
                Ok::<Token, TokenError>(Token::Integer)
            }
        },
    )(input)
}

/// Tests a string starting and ending with double quotes.
fn string_literal(input: Span) -> IResult<Token> {
    let (input, _) = delimited(char('"'), take_while(|c| c != '"'), char('"'))(input)?;
    Ok((input, Token::String))
}

/// Tests for other symbols literals
fn simple_token(input: Span) -> IResult<Token> {
        alt((
            map(tag("=="), |_| Token::EqualsEquals),
            map(tag("!="), |_| Token::NotEquals),
            map(tag("<="), |_| Token::LessThanEquals),
            map(tag(">="), |_| Token::GreaterThanEquals),
            map(tag("+"),  |_| Token::Plus),
            map(tag("-"),  |_| Token::Minus),
            map(tag("*"),  |_| Token::Star),
            map(tag("/"),  |_| Token::RSlash),
            map(tag("\\"), |_|Token::LSlash),
            map(tag("="),  |_| Token::Equals),
            map(tag("<"),  |_| Token::LessThan),
            map(tag(">"),  |_| Token::GreaterThan),
            map(tag("("),  |_| Token::LParenthesis),
            map(tag(")"),  |_| Token::RParenthesis),
            map(tag("["),  |_| Token::LBracket),
            map(tag("]"),  |_| Token::RBracket),
            map(tag("{"),  |_| Token::LBrace),
            map(tag("}"),  |_| Token::RBrace),
            map(tag(","),  |_| Token::Comma),
            map(tag("|"),  |_| Token::Bar),
        ))(input)
}

/// Matches exactly one token.
/// Combines the token type with the location into a `TokenRecord`.
fn one_token(input: Span) -> IResult<TokenRecord> {
   let start = position(input)?.0;
    let result = alt((
        whitespace,
        comment,
        number,
        simple_token,
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

/// Parses all tokens
/// Filters out whitespace and comments
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

/// Main entry point for the lexer.
pub fn lex(files: &FileSet, file: FileIdentifier) -> Result<Vec<TokenRecord>, error::TokenError> {
    let input = files.get_content(file).expect("File not found");
    let input = LocatedSpan::new(input);
    final_parser(tokens)(input).map_err(|a: TokenError| {
        error::TokenError::new(file, a.index)
    })
}

