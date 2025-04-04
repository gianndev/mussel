use crate::parser::*;

// Declares the 'interpreter' function, which takes an expression ('Expr') as input.
pub fn interpreter(expr: Expr) {
    // Matches the expression to identify its variant.
    match expr {
        // Handles the 'Call' variant, extracting the function name and its argument.
        Expr::Call(name, arg) => {
            
            // Checks if the function name is "println", which is used for printing output.
            if name == "println" {
                
                // Prints the argument using its Display implementation.
                // This ensures the argument is displayed in a user-friendly format.
                println!("{arg}");
            }
        },
        _ => {}, // Ignores other variants (e.g., 'Let') for now.
    }
}
