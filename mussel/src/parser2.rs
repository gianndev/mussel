// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use color_eyre::{Help};
use nom::multi::{many0, separated_list0};
use nom::{Parser};
use nom::branch::alt;
use nom::combinator::{cut, map, opt};
use nom::sequence::{delimited, tuple};
use nom_supreme::final_parser::{final_parser, ExtractContext};
use crate::error;
use crate::error::{FileIdentifier, LError};
use crate::expr::{BinOp, Operator};
use crate::lexer::{Token, TokenRecord};



/// This is the main parser for the language.
/// It takes a slice of tokens and returns an AST.
///
///
///
/// # Grammar
///
/// ```
/// // Entry point
/// unit ::= expr*
///
/// expr ::= include
///     | return
///     | function
///     | for
///     | until
///     | if
///     | let
///     | conditionalOrExpression
///
/// include ::= 'include' id
/// return ::= 'return' expr
/// function ::= 'fn' id '(' (id (',' id)*)? ')' block
/// for ::= 'for' id 'in' expr block
/// until ::= 'until' expr block
/// if ::= 'if' expr block ('else' block)?
/// let ::= 'let' id '=' expr
///
/// block ::= '{' expr* '}'
///
/// // Math precedence
/// conditionalOrExpression: conditionalAndExpression ('||' conditionalOrExpression)?;
/// conditionalAndExpression: equalityExpression ('&&' conditionalAndExpression)?;
/// equalityExpression: relationalExpression (('==' | '!=') equalityExpression)?;
/// relationalExpression: additiveExpression (('<' | '>' | '<=' | '>=') relationalExpression)?;
/// additiveExpression: multiplicativeExpression (('+' | '-') additiveExpression)?;
/// multiplicativeExpression: unaryExpression (('*' | '/') multiplicativeExpression)?;
/// unaryExpression: ('-' | '!')? factor;
///
/// factor: object postFix* ('=' expr)?;
/// // calls and array indexing
/// postFix: '(' expressionList ')' | '[' expr ']';
/// expressionList: (expr (',' expr)*)?;
///
/// // lowest expression
/// object: array | closure | string | integer | float | bool | id | '(' expr ')'
///
/// array ::= '[' (expr (',' expr)*)? ']'
/// closure ::= '|' (id (',' id)*)? '|' block
///
/// # literals
/// id ::= 'id'
/// string ::= 'string'
/// integer ::= 'integer'
/// float ::= 'float'
/// bool ::= 'true' | 'false'
/// ```



/// Defines a custom Result type with the input of TokenRecords and the custom ErrorType
type IResult<'a, O> = nom::IResult<&'a [TokenRecord], O, ParseError>;

/// Used to define where an Expression was defined. Currently not used.
type Range = core::ops::Range<usize>;


/// Custom Error Type for better error reporting
/// `self.get_offset` returns where the error occurred in the input stream (character index)
/// `self.to_string` returns a readable error message.
///
/// implements `ParseError` to use as a nom error
pub(crate) enum ParseError {

    // Unexpected Token
    UnexpectedToken { found: TokenRecord, expected: Token },

    // Fallback when the parser reaches the end of the input
    Eof,

    // Unexpected Token
    UnexpectedEnd { found: TokenRecord },

    // nom::error::ErrorKind is the standard nom error, needed for ParseError
    Internal { record: TokenRecord, kind: nom::error::ErrorKind },

    // Combining multiple errors
    // fixme: This might not be useful for the user in the future
    List(Vec<ParseError>)
}

impl<'a> nom::error::ParseError<&'a [TokenRecord]> for ParseError {
    fn from_error_kind(input: &'a [TokenRecord], kind: nom::error::ErrorKind) -> Self {


        if let Some((first, _)) = input.split_first() {
            if kind == nom::error::ErrorKind::Eof {
                ParseError::UnexpectedEnd {
                    found: first.clone()
                }
            } else {
                ParseError::Internal {
                    record: first.clone(),
                    kind
                }
            }
        } else {
            ParseError::Eof
        }
    }

    fn append(input: &'a [TokenRecord], kind: nom::error::ErrorKind, other: Self) -> Self {
        let fst = if let Some((first, _)) = input.split_first() {
            ParseError::Internal {
                record: first.clone(),
                kind
            }
        } else {
            ParseError::Eof
        };
        ParseError::List(vec![fst, other])
    }

    fn from_char(input: &'a [TokenRecord], _c: char) -> Self {
        if let Some((first, _)) = input.split_first() {
            ParseError::UnexpectedToken {
                found: first.clone(),
                expected: Token::Identifier,
            }
        } else {
            ParseError::Eof
        }
    }

    fn or(self, other: Self) -> Self {
        ParseError::List(vec![self, other])
    }
}
impl ExtractContext<&[TokenRecord], ParseError> for ParseError {
    fn extract_context(self, _: &[TokenRecord]) -> ParseError {
        self
    }
}

pub fn offset_to_line_column(text: &str, offset: usize) -> Option<(usize, usize)> {
    if offset > text.len() {
        return None;
    }

    let mut current_offset = 0;
    for (line_number, line) in text.lines().enumerate() {
        let line_len = line.len();

        if offset <= current_offset + line_len {
            let column = offset - current_offset + 1;
            return Some((line_number + 1, column));
        }

        // Add 1 for the '\n' character that .lines() strips
        current_offset += line_len + 1;
    }

    // If offset is at the very end (after the last line)
    if offset == text.len() {
        if let Some(last_line_number) = text.lines().count().checked_sub(1) {
            return Some((last_line_number + 1, text.lines().last().unwrap_or("").len() + 1));
        }
    }

    None
}


#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Into<Option<BinOp>> for BinaryOperator {
    fn into(self) -> Option<BinOp> {
        match self {
            BinaryOperator::Add => Some(BinOp::Add),
            BinaryOperator::Subtract => Some(BinOp::Sub),
            BinaryOperator::Multiply => Some(BinOp::Mul),
            BinaryOperator::Divide => Some(BinOp::Div),
            _ => None,
        }
    }
}

impl Into<Option<Operator>> for BinaryOperator {
    fn into(self) -> Option<Operator> {
        match self {
            BinaryOperator::Equal => Some(Operator::Equal),
            BinaryOperator::NotEqual => Some(Operator::NotEqual),
            BinaryOperator::LessThan => Some(Operator::LessThan),
            BinaryOperator::GreaterThan => Some(Operator::GreaterThan),
            BinaryOperator::LessThanOrEqual => Some(Operator::LessThanEqual),
            BinaryOperator::GreaterThanOrEqual => Some(Operator::GreaterThanEqual),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug)]
pub(crate) enum Expression {
    Include { id: TokenRecord},
    Return { expr: Box<Expression> },
    Function { id: TokenRecord, args: Vec<TokenRecord>, block: Vec<Expression> },
    For { id: TokenRecord, expr: Box<Expression>, block: Vec<Expression> },
    Until { expr: Box<Expression>, block: Vec<Expression> },
    If { expr: Box<Expression>, block: Vec<Expression>, else_block: Option<Vec<Expression>> },
    Let { id: TokenRecord, expr: Box<Expression> },

    Binary { left: Box<Expression>, operator: (BinaryOperator, TokenRecord), right: Box<Expression> },
    Unary { operator: (UnaryOperator, TokenRecord), expr: Box<Expression> },

    Assignment { region: TokenRecord, left: Box<Expression>, right: Box<Expression> },

    Identifier(TokenRecord),
    String(TokenRecord),
    Integer(TokenRecord),
    Float(TokenRecord),
    Bool(TokenRecord),
    Array(Vec<Expression>),
    Closure { args: Vec<TokenRecord>, block: Vec<Expression> },

    Call { region: TokenRecord, left: Box<Expression>, args: Vec<Expression> },
    Index { region: TokenRecord, left: Box<Expression>, index: Box<Expression> },
}


/// Represents a Call or Index. This is turned into a `Expression` in the `factor` method
/// The Call and Index expression store the left side of the expression, so this extra step is
/// needed to satisfy the borrow checker.
enum PostFixExpr {
    Call(TokenRecord, Vec<Expression>),
    Index(TokenRecord, Box<Expression>),
}



/// This Function test for a specific token type.
fn match_token<'a>(expected: Token) -> impl Fn(&'a [TokenRecord]) -> IResult<&'a TokenRecord> {
    move |input: &'a [TokenRecord]| {
        if let Some((first, rest)) = input.split_first() {
            if first.token_type == expected {
                Ok((rest, first))
            } else {
                Err(nom::Err::Error(ParseError::UnexpectedToken {
                    found: first.clone(),
                    expected,
                }))
            }
        } else {
            Err(nom::Err::Error(ParseError::Eof))
        }
    }
}


/// Main entry function for the parser
pub fn parser(file: FileIdentifier, input: &[TokenRecord]) -> Result<Vec<Expression>, Box<dyn LError>> {
    let max_length = input.last().map(|last| last.offset + last.length).unwrap_or(0);
    final_parser(unit)(input).map_err(|a| to_external_error(a, file, max_length))
}

fn to_external_error(internal: ParseError, file: FileIdentifier, max_length: usize) -> Box<dyn LError> {
    match internal {
        ParseError::UnexpectedToken { found, expected } => {
            let message = format!("Unexpected token: {:?}, expected {:?}", found.token_type, expected);
            Box::new(error::UnexpectedTokenError::new(file, found, message))
        },
        ParseError::Eof => {
            Box::new(error::UnexpectedEndOfFileError::new(file, max_length))
        }
        ParseError::Internal { record, kind } => {
            let message = format!("Error Parsing: {:?}", kind);
            Box::new(error::UnexpectedTokenError::new(file, record, message))
        }
        ParseError::List(list) => {
            let mut compound = error::ErrorCollection::new();
            for internal_ in list {
                compound.add_error(to_external_error(internal_, file, max_length));
            }

            Box::new(compound)
        }
        ParseError::UnexpectedEnd { found } => {
            let message = "Invalid syntax".to_string();
            Box::new(error::UnexpectedTokenError::new(file, found, message))
        }
    }
}

// <editor-fold desc="Rules">

fn expression_list(input: &[TokenRecord]) -> IResult<Vec<Expression>> {
    let (input, expr) = separated_list0(match_token(Token::Comma), expr)(input)?;
    Ok((input, expr))
}

fn post_fix(input: &[TokenRecord]) -> IResult<PostFixExpr> {
    let call = tuple((match_token(Token::LParenthesis), expression_list, match_token(Token::RParenthesis)));
    let index = tuple((match_token(Token::LBracket), expr, match_token(Token::RBracket)));
    alt((
        map(call, |(l, args, r)| PostFixExpr::Call(l.clone(), args)),
        map(index, |(l, index, r)| PostFixExpr::Index(l.clone(), Box::new(index))),
    ))(input)
}

fn factor(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = object(input)?;

    let (input, postfix) = many0(post_fix)(input)?;

    fn apply(expr: PostFixExpr, left: Expression) -> Expression {
        match expr {
            PostFixExpr::Call(record, args) =>
                Expression::Call { region: record, left: Box::new(left), args },
            PostFixExpr::Index(record, index) =>
                Expression::Index { region: record, left: Box::new(left), index },
        }
    }

    let mut left = left;
    for post in postfix {
        left = apply(post, left);
    }

    let (input, assign) = opt(tuple((
        match_token(Token::Equals),
        expr
    )))(input)?;

    let left = if let Some((l, right)) = assign {
        Expression::Assignment {
            region: l.clone(),
            left: Box::new(left),
            right: Box::new(right)
        }
    } else {
        left
    };

    Ok((input, left))
}

fn unary_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, op) = opt(alt((
        map(match_token(Token::Minus), |f| (UnaryOperator::Negate, f.clone())),
        map(match_token(Token::Not), |f| (UnaryOperator::Not, f.clone())),
    )))(input)?;
    let (input, expr) = factor(input)?;
    if let Some(op) = op {
        Ok((input, Expression::Unary { operator: op.clone(), expr: Box::new(expr) }))
    } else {
        Ok((input, expr))
    }

}


fn multiplicative_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = unary_expression(input)?;
    let (input, right) = opt(tuple((
        alt((
            map(match_token(Token::Star), |f| (BinaryOperator::Multiply, f.clone())),
            map(match_token(Token::RSlash), |f| (BinaryOperator::Divide, f.clone())),
        )),
        multiplicative_expression
    )))(input)?;
    if let Some((op, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}

fn additive_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = multiplicative_expression(input)?;
    let (input, right) = opt(tuple((
        alt((
            map(match_token(Token::Plus), |f| (BinaryOperator::Add, f.clone())),
            map(match_token(Token::Minus), |f| (BinaryOperator::Subtract, f.clone())),
        )),
        additive_expression
    )))(input)?;
    if let Some((op, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}

fn relational_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = additive_expression(input)?;
    let (input, right) = opt(tuple((
        alt((
            map(match_token(Token::LessThan), |f| (BinaryOperator::LessThan, f.clone())),
            map(match_token(Token::GreaterThan), |f| (BinaryOperator::GreaterThan, f.clone())),
            map(match_token(Token::LessThanEquals), |f| (BinaryOperator::LessThanOrEqual, f.clone())),
            map(match_token(Token::GreaterThanEquals), |f| (BinaryOperator::GreaterThanOrEqual, f.clone())),
        )),
        relational_expression
    )))(input)?;
    if let Some((op, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}

fn equality_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = relational_expression(input)?;
    let (input, right) = opt(tuple((
        alt((
            map(match_token(Token::EqualsEquals), |f| (BinaryOperator::Equal, f.clone())),
            map(match_token(Token::NotEquals), |f| (BinaryOperator::NotEqual, f.clone())),
        )),
        equality_expression
    )))(input)?;
    if let Some((op, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}



fn conditional_and_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = equality_expression(input)?;
    let (input, right) = opt(tuple((
        map(match_token(Token::And), |f| (BinaryOperator::And, f.clone())),
        conditional_and_expression
    )))(input)?;
    if let Some((op, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}
fn conditional_or_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = conditional_and_expression(input)?;
    let (input, right) = opt(tuple((
        map(match_token(Token::Or), |f| (BinaryOperator::Or, f.clone())),
        conditional_or_expression
    )))(input)?;
    if let Some((op, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}

fn array(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::LBracket)(input)?;
    let (input, expr) = separated_list0(match_token(Token::Comma), expr)(input)?;
    let (input, _) = match_token(Token::RBracket)(input)?;
    Ok((input, Expression::Array(expr)))
}

fn closure(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::Bar)(input)?;
    let (input, args) = separated_list0(match_token(Token::Comma), match_token(Token::Identifier))(input)?;
    let (input, _) = match_token(Token::Bar)(input)?;
    let (input, block) = block(input)?;
    Ok((input, Expression::Closure { args: args.into_iter().cloned().collect(), block }))
}

fn object(input: &[TokenRecord]) -> IResult<Expression> {
    alt((
        array,
        closure,
        map(match_token(Token::String), |r| Expression::String(r.clone())),
        map(match_token(Token::Integer), |r| Expression::Integer(r.clone())),
        map(match_token(Token::Float), |r| Expression::Float(r.clone())),
        map(match_token(Token::Boolean), |r| Expression::Bool(r.clone())),
        map(match_token(Token::Identifier), |r| Expression::Identifier(r.clone())),
        delimited(match_token(Token::LParenthesis), expr, match_token(Token::RParenthesis)),
    ))(input)
}


fn let_statement(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::Let)(input)?;
    let (input, id) = cut(match_token(Token::Identifier))(input)?;
    let (input, _) = cut(match_token(Token::Equals))(input)?;
    let (input, expr) = cut(expr)(input)?;
    Ok((input, Expression::Let { id: id.clone(), expr: Box::new(expr) }))
}

fn if_statement(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::If)(input)?;
    let (input, expr_) = expr(input)?;
    let (input, block_) = block(input)?;
    //there might be a better way than to throw away the first token of the tuple
    let (input, else_block) = opt(tuple((
        match_token(Token::Else),
        alt((block, map(expr, |e| vec![e])))
    )))(input)?;
    Ok((input, Expression::If {
        expr: Box::new(expr_),
        block: block_,
        else_block: else_block.map(|(_, vec)| vec)
    }))
}


fn until(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::Until)(input)?;
    let (input, expr) = expr(input)?;
    let (input, block) = block(input)?;
    Ok((input, Expression::Until {
        expr: Box::new(expr),
        block
    }))
}

fn for_loop(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::For)(input)?;
    let (input, id) = match_token(Token::Identifier)(input)?;
    let (input, _) = match_token(Token::In)(input)?;
    let (input, expr) = expr(input)?;
    let (input, block) = block(input)?;
    Ok((input, Expression::For {
        id: id.clone(),
        expr: Box::new(expr),
        block
    }))
}

fn function(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::Fn)(input)?;
    let (input, id) = match_token(Token::Identifier)(input)?;
    let (input, _) = match_token(Token::LParenthesis)(input)?;
    let (input, args) = separated_list0(match_token(Token::Comma), match_token(Token::Identifier))(input)?;
    let (input, _) = match_token(Token::RParenthesis)(input)?;
    let (input, block) = block(input)?;
    Ok((input, Expression::Function {
        id: id.clone(),
        args: args.into_iter().cloned().collect(),
        block
    }))
}

fn return_statement(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::Return)(input)?;
    let (input, expr) = expr(input)?;
    Ok((input, Expression::Return { expr: Box::new(expr) }))
}

fn include(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, _) = match_token(Token::Include)(input)?;
    let (input, id) = match_token(Token::Identifier)(input)?;
    Ok((input, Expression::Include { id: id.clone() } ))
}

fn expr(input: &[TokenRecord]) -> IResult<Expression> {
    alt((
        include,
        return_statement,
        function,
        for_loop,
        until,
        if_statement,
        let_statement,
        conditional_or_expression
    ))(input)
}

fn block(input: &[TokenRecord]) -> IResult<Vec<Expression>> {
    let (input, _) = match_token(Token::LBrace)(input)?;
    let (input, expr) = many0(expr)(input)?;
    let (input, _) = match_token(Token::RBrace)(input)?;
    Ok((input, expr))
}

fn unit(input: &[TokenRecord]) -> IResult<Vec<Expression>> {
    many0(expr)(input)
}

// </editor-fold>