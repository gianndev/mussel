use color_eyre::eyre::eyre;
use color_eyre::{Help, Report};
use nom::multi::{many0, separated_list0};
use nom::{Parser};
use nom::branch::alt;
use nom::combinator::{cut, map, opt};
use nom::error::ParseError;
use nom::sequence::{delimited, tuple};
use nom_locate::LocatedSpan;
use crate::lexer::{Token, TokenRecord};
/*

unit ::= expr*

expr ::= include
    | return
    | function
    | for
    | until
    | if
    | let
    | conditionalOrExpression

include ::= 'include' id
return ::= 'return' expr
function ::= 'fn' id '(' (id (',' id)*)? ')' block
for ::= 'for' id 'in' expr block
until ::= 'until' expr block
if ::= 'if' expr block ('else' block)?
let ::= 'let' id '=' expr

block ::= '{' expr* '}'

conditionalOrExpression: conditionalAndExpression ('||' conditionalOrExpression)?;
conditionalAndExpression: equalityExpression ('&&' conditionalAndExpression)?;
equalityExpression: relationalExpression (('==' | '!=') equalityExpression)?;
relationalExpression: additiveExpression (('<' | '>' | '<=' | '>=') relationalExpression)?;
additiveExpression: multiplicativeExpression (('+' | '-') additiveExpression)?;
multiplicativeExpression: unaryExpression (('*' | '/') multiplicativeExpression)?;
unaryExpression: ('-' | '!')? factor;

factor: object postFix* ('=' expr)?;
postFix: '(' expressionList ')' | '[' expr ']';

object: array | closure | string | integer | float | bool | id | '(' expr ')'

expressionList: (expr (',' expr)*)?;

array ::= '[' (expr (',' expr)*)? ']'
closure ::= '|' (id (',' id)*)? '|' block

//constants (with regex, just examples)
id ::= [a-zA-Z_][a-zA-Z0-9_]*
string ::= ".*"
integer ::= (-)? [0-9]+
float ::= (-)? [0-9]* \. [0-9]*
bool ::= 'true' | 'false'

*/

type IResult<'a, O> = nom::IResult<&'a [TokenRecord], O, ErrorKind>;
type Range = core::ops::Range<usize>;

fn match_token<'a>(expected: Token) -> impl Fn(&'a [TokenRecord]) -> IResult<&'a TokenRecord> {
    move |input: &'a [TokenRecord]| {
        if let Some((first, rest)) = input.split_first() {
            if first.token_type == expected {
                Ok((rest, first))
            } else {
                Err(nom::Err::Error(ErrorKind::UnexpectedToken {
                    found: first.clone(),
                    expected,
                }))
            }
        } else {
            Err(nom::Err::Error(ErrorKind::Eof))
        }
    }
}

pub(crate) enum ErrorKind {
    UnexpectedToken { found: TokenRecord, expected: Token },
    Eof,
    UnexpectedEndOfInput { record: TokenRecord },
    Internal { record: TokenRecord, kind: nom::error::ErrorKind },
    List(Vec<ErrorKind>)
}



impl<'a> ParseError<&'a [TokenRecord]> for ErrorKind {
    fn from_error_kind(input: &'a [TokenRecord], kind: nom::error::ErrorKind) -> Self {
        if let Some((first, _)) = input.split_first() {
            ErrorKind::Internal {
                record: first.clone(),
                kind
            }
        } else {
            ErrorKind::Eof
        }
    }

    fn append(input: &'a [TokenRecord], kind: nom::error::ErrorKind, other: Self) -> Self {
        let fst = if let Some((first, _)) = input.split_first() {
            ErrorKind::Internal {
                record: first.clone(),
                kind
            }
        } else {
            ErrorKind::Eof
        };
        ErrorKind::List(vec![other, fst])

    }

    fn from_char(input: &'a [TokenRecord], _c: char) -> Self {
        if let Some((first, _)) = input.split_first() {
            ErrorKind::UnexpectedToken {
                found: first.clone(),
                expected: Token::Identifier,
            }
        } else {
            ErrorKind::Eof
        }
    }

    fn or(self, other: Self) -> Self {
        ErrorKind::List(vec![self, other])
    }
}

impl ErrorKind {
    fn get_offset(&self, max: usize) -> usize {
        match self {
            ErrorKind::UnexpectedToken { found, expected: _ } => found.offset,
            ErrorKind::Eof => max,
            ErrorKind::UnexpectedEndOfInput { record } => record.offset,
            ErrorKind::Internal { record, kind: _ } => record.offset,
            ErrorKind::List(v) => v.first().unwrap().get_offset(max)
        }
    }

    pub(crate) fn create_report(self, file: &str, file_content: &str) -> Report {

        let location = self.get_offset(file_content.len());
        let span = unsafe { LocatedSpan::new_from_raw_offset(location, 0, file_content, ()) };

        let kind = self.to_string();
        let line = span.location_line();
        let column = span.get_column();

        let code = file_content;
        let code = format!("{:?}", code.to_string());
        eyre!("Error occurred while parsing '{kind}' {file}:{line}:{column}").with_section(move || code)

    }

    fn to_string(self) -> String {
        match self {
            ErrorKind::UnexpectedToken { found, expected } => {
                format!("Unexpected token: {:?}, expected {:?}", found.token_type, expected)
            },
            ErrorKind::Eof => "End of file reached".to_string(),
            ErrorKind::UnexpectedEndOfInput { record } => {
                format!("Unexpected end of input at token: {:?}", record.token_type)
            }
            ErrorKind::Internal { record: _, kind } => {
                format!("{:?}", kind)
            }
            ErrorKind::List(list) => {
                let mut result = String::new();
                for error in list {
                    result.push_str(&error.to_string());
                    result.push('\n');
                }
                result
            }
        }
    }

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

    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    Unary { operator: UnaryOperator, expr: Box<Expression> },

    Assignment { left: Box<Expression>, right: Box<Expression> },

    Identifier(TokenRecord),
    String(TokenRecord),
    Integer(TokenRecord),
    Float(TokenRecord),
    Bool(TokenRecord),
    Array(Vec<Expression>),
    Closure { args: Vec<TokenRecord>, block: Vec<Expression> },

    Call { left: Box<Expression>, args: Vec<Expression> },
    Index { left: Box<Expression>, index: Box<Expression> },
}

enum PostFixExpr {
    Call(Vec<Expression>),
    Index(Box<Expression>),
}

impl PostFixExpr {
     fn apply(self, left: Expression) -> Expression {
         match self {
             PostFixExpr::Call(args) => Expression::Call { left: Box::new(left), args },
             PostFixExpr::Index(index) => Expression::Index { left: Box::new(left), index },
         }
     }
}

fn expression_list(input: &[TokenRecord]) -> IResult<Vec<Expression>> {
    let (input, expr) = separated_list0(match_token(Token::Comma), expr)(input)?;
    Ok((input, expr))
}

fn post_fix(input: &[TokenRecord]) -> IResult<PostFixExpr> {
    let call = delimited(match_token(Token::LParenthesis), expression_list, match_token(Token::RParenthesis));
    let index = delimited(match_token(Token::LBracket), expr, match_token(Token::RBracket));
    alt((
        map(call, |args| PostFixExpr::Call(args)),
        map(index, |index| PostFixExpr::Index(Box::new(index))),
    ))(input)
}

fn factor(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = object(input)?;

    let (input, postfix) = many0(post_fix)(input)?;

    let mut left = left;
    for post in postfix {
        left = post.apply(left);
    }

    let (input, assign) = opt(tuple((
        match_token(Token::Equals),
        expr
    )))(input)?;

    let left = if let Some((_, right)) = assign {
        Expression::Assignment {
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
        map(match_token(Token::Minus), |_| UnaryOperator::Negate),
        map(match_token(Token::Bang), |_| UnaryOperator::Not),
    )))(input)?;
    let (input, expr) = factor(input)?;
    if let Some(op) = op {
        Ok((input, Expression::Unary { operator: op, expr: Box::new(expr) }))
    } else {
        Ok((input, expr))
    }

}


fn multiplicative_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = unary_expression(input)?;
    let (input, right) = opt(tuple((
        alt((
            map(match_token(Token::Star), |_| BinaryOperator::Multiply),
            map(match_token(Token::RSlash), |_| BinaryOperator::Divide),
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
            map(match_token(Token::Plus), |_| BinaryOperator::Add),
            map(match_token(Token::Minus), |_| BinaryOperator::Subtract),
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
            map(match_token(Token::LessThan), |_| BinaryOperator::LessThan),
            map(match_token(Token::GreaterThan), |_| BinaryOperator::GreaterThan),
            map(match_token(Token::LessThanEquals), |_| BinaryOperator::LessThanOrEqual),
            map(match_token(Token::GreaterThanEquals), |_| BinaryOperator::GreaterThanOrEqual),
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
            map(match_token(Token::Equals), |_| BinaryOperator::Equal),
            map(match_token(Token::NotEquals), |_| BinaryOperator::NotEqual),
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
        match_token(Token::And),
        conditional_and_expression
    )))(input)?;
    if let Some((_, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::And,
            right: Box::new(right)
        }))
    } else {
        Ok((input, left))
    }
}
fn conditional_or_expression(input: &[TokenRecord]) -> IResult<Expression> {
    let (input, left) = conditional_and_expression(input)?;
    let (input, right) = opt(tuple((
        match_token(Token::Or),
        conditional_or_expression
    )))(input)?;
    if let Some((_, right)) = right {
        Ok((input, Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Or,
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

pub fn parser(input: &[TokenRecord]) -> Result<Vec<Expression>, ErrorKind> {
    match unit(input) {
        Ok((left, expr)) =>
            if !left.is_empty() {
                Err(ErrorKind::UnexpectedEndOfInput {record: left[0].clone() })
            } else {
                Ok(expr)
            }
        Err(err) => match err {
            nom::Err::Incomplete(_) => Err(ErrorKind::Eof),
            nom::Err::Error(e) => Err(e),
            nom::Err::Failure(e) => Err(e)
        }
    }
}