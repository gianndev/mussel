use crate::parser::*; // Imports everything from the 'parse' module, allowing access to 'Expr' and other definitions.

// Declares the 'interpreter' function, which takes an expression ('Expr') as input.
pub fn interpreter(expr: Expr) {
    // Matches the expression to identify its variant.
    match expr {
        // Handles the 'Call' variant, extracting the function name and its argument.
        Expr::Call(name, arg) => {
            
            // Checks if the function name is "println", which is used for printing output.
            if name == "println" {
                
                // Prints the argument in debug format ('{:?}'), which includes details for debugging.
                println!("{arg:?}");
            }
        },
    }
}
