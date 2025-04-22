use std::fmt;

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
