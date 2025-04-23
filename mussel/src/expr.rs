use std::fmt;
use crate::error::{FileIdentifier, FileSet, NotSupportedOperationError};
use crate::parser::Expression;

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

impl Expr {

    pub fn from_parser(files:& FileSet, file: FileIdentifier, parsed: Vec<Expression>)
                       -> Result<Vec<Expr>, NotSupportedOperationError> {

        let content = files.get_content(file).unwrap_or_else(|| {
            panic!("Failed to retrieve content");
        });

        Self::from_parser_block(file, content, parsed)

    }

    fn from_parser_inner(file: FileIdentifier, content: &str, expr: Expression) -> Result<Expr, NotSupportedOperationError> {
        Ok(match expr {
            Expression::Include { id } => {
                Expr::Include(id.get_content(content).to_string())
            }
            Expression::Return { expr } =>  {
                Expr::Return(Box::new(Self::from_parser_inner(file, content, *expr)?))
            }
            Expression::Function { id, args, block } => {
                let name = id.get_content(content).to_string();
                let args = args.iter()
                    .map(|arg| arg.get_content(content).to_string())
                    .collect::<Vec<String>>();
                let body = Self::from_parser_block(file, content, block)?;
                Expr::Function(name, args, body)
            }
            Expression::For { id, expr, block } => {
                let name = id.get_content(content).to_string();
                let body = Self::from_parser_block(file, content, block)?;
                let expr = Box::new(Self::from_parser_inner(file, content, *expr)?);
                Expr::For(name, expr, body)
            }
            Expression::Until { expr, block } => {
                let expr = Box::new(Self::from_parser_inner(file, content, *expr)?);
                let body = Self::from_parser_block(file, content, block)?;
                Expr::Until(expr, body)
            }
            Expression::If { expr, block, else_block } => {
                let expr = Box::new(Self::from_parser_inner(file, content, *expr)?);
                let body = Self::from_parser_block(file, content, block)?;
                let else_body = if let Some(else_block) = else_block {
                    Some(Self::from_parser_block(file, content, else_block)?)
                } else {
                    None
                };
                Expr::If(expr, body, else_body)
            }
            Expression::Let { id, expr } => {
                let name = id.get_content(content).to_string();
                let expr = Box::new(Self::from_parser_inner(file, content, *expr)?);
                Expr::Let(name, expr)
            }
            Expression::Binary { left, operator: (operator, token), right } => {
                let lhs = Box::new(Self::from_parser_inner(file, content, *left)?);
                let rhs = Box::new(Self::from_parser_inner(file, content, *right)?);
                return if let Some(binOp) = operator.into() {
                    Ok(Expr::Binary(lhs, binOp, rhs))
                } else if let Some(op) = operator.into() {
                    Ok(Expr::Compare(lhs, op, rhs))
                } else {
                    Err(NotSupportedOperationError::new(
                        file,
                        token,
                        format!("Unsupported operator: {operator:?}"),
                    ))
                }
            }
            Expression::Unary { operator: (_, record), .. } => {
                return Err(NotSupportedOperationError::new(
                    file,
                    record,
                    "Unary operations are not supported".to_string(),
                ));
            }
            Expression::Assignment { region, .. } => {
                return Err(NotSupportedOperationError::new(
                    file,
                    region,
                    "Assignment operations are not supported".to_string(),
                ));
            }
            Expression::Identifier(name) => {
                let name = name.get_content(content).to_string();
                Expr::Constant(Atom::Name(name))
            }
            Expression::String(token) => {
                let string = token.get_content(content).to_string();
                if (string.len() < 2) {
                    return Err(NotSupportedOperationError::new(
                        file,
                        token,
                        format!("Invalid string: {string}"),
                    ));
                }
                let string = string[1..string.len() - 1].to_string();
                Expr::Constant(Atom::String(string))
            }
            Expression::Integer(token) => {
                let number = token.get_content(content).to_string();
                return if let Ok(asInt) = number.parse::<i64>() {
                    Ok(Expr::Constant(Atom::Number(asInt)))
                } else {
                    Err(NotSupportedOperationError::new(
                        file,
                        token,
                        format!("Invalid integer: {number}"),
                    ))
                }
            }
            Expression::Float(token) => {
                let number = token.get_content(content).to_string();
                return if let Ok(asFloat) = number.parse::<f64>() {
                    Ok(Expr::Constant(Atom::Float(asFloat)))
                } else {
                    Err(NotSupportedOperationError::new(
                        file,
                        token,
                        format!("Invalid float: {number}"),
                    ))
                }
            }
            Expression::Bool(bool) => {
                let boolean = bool.get_content(content).to_string();
                return if let Ok(asBool) = boolean.parse::<bool>() {
                    Ok(Expr::Constant(Atom::Boolean(asBool)))
                } else {
                    Err(NotSupportedOperationError::new(
                        file,
                        bool,
                        format!("Invalid boolean: {boolean}"),
                    ))
                }
            }
            Expression::Array(inner) => {
                let items = Self::from_parser_block(file, content, inner)?;
                Expr::Array(items)
            }
            Expression::Closure { args, block } => {
                let args = args.iter()
                    .map(|arg| arg.get_content(content).to_string())
                    .collect::<Vec<String>>();
                let body = Self::from_parser_block(file, content, block)?;
                Expr::Closure(args, body)
            }
            Expression::Call { region, left, args } => {
                let name = Self::from_parser_inner(file, content, *left)?;
                let args = Self::from_parser_block(file, content, args)?;
                return if let Expr::Constant(Atom::Name(name)) = name {
                    Ok(Expr::Call(name.to_string(), args))
                } else {
                    return Err(NotSupportedOperationError::new(
                        file,
                        region,
                        format!("Invalid function call, only call a function directly yet: {name}"),
                    ));
                }
            }
            Expression::Index { region, left, index } => {
                let name = Self::from_parser_inner(file, content, *left)?;
                let index = Self::from_parser_inner(file, content, *index)?;
                return if let Expr::Constant(Atom::Name(name)) = name {
                    if let Expr::Constant(Atom::Number(index)) = index {
                        Ok(Expr::Get(name.to_string(), index as usize))
                    } else {
                        Err(NotSupportedOperationError::new(
                            file,
                            region,
                            format!("Invalid index, not supported yet: {index}"),
                        ))
                    }
                } else {
                    Err(NotSupportedOperationError::new(
                        file,
                        region,
                        format!("Invalid array access, not supported yet: {name}"),
                    ))
                }
            }
        })
    }
    fn from_parser_block(file: FileIdentifier, content: &str, block: Vec<Expression>) -> Result<Vec<Expr>, NotSupportedOperationError> {
        block.into_iter().map(|expr| {
            Self::from_parser_inner(file, content, expr)
        }).collect()
    }

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


