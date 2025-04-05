# Mussel

<div align="center">
<img src="doc/images/logo.png" height=250>
</div>

**Mussel** is an interpreted programming language written in Rust. It was designed as a simple and flexible language with a focus on easy-to-read syntax and powerful expression evaluation. Mussel supports basic data types, variable bindings, functions, closures, control flow constructs (such as `if` and `for`), and even string interpolation.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Language Syntax & Constructs](#language-syntax--constructs)
  - [Atoms and Constants](#atoms-and-constants)
  - [Let Bindings](#let-bindings)
  - [Function Calls and Definitions](#function-calls-and-definitions)
  - [Closures](#closures)
  - [Control Flow: if and for](#control-flow-if-and-for)
  - [Arrays and Element Access](#arrays-and-element-access)
  - [String Interpolation](#string-interpolation)
- [Tutorial & Practical Example](#tutorial--practical-example)
- [How It Works](#how-it-works)

## Features

- **Simple Syntax:** Mussel syntax is minimalistic and easy to learn.
- **Interpreted Execution:** No separate compilation step is required; simply run your Mussel scripts with the provided interpreter.
- **Dynamic Evaluation:** Supports let-bindings, function definitions, closures, and control flow constructs.
- **Error Reporting:** Uses `color_eyre` for enhanced and user-friendly error messages.
- **Built-in Functions:** Includes common functions like `println` for printing to the console.
- **String Interpolation:** Embed expressions directly within string literals for dynamic output.

## Getting Started

### Prerequisites

To build and run Mussel, you will need:
- [Rust](https://www.rust-lang.org/tools/install) (stable channel)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/mussel.git
   cd mussel
   ```

2. Build the project with Cargo:
   ```bash
   cargo build --release
   ```

3. Run the interpreter with a Mussel script:
   ```bash
   cargo run FILE=path/to/your_file.mus
   ```

## Language Syntax & Constructs

### Atoms and Constants

Mussel supports several basic literal types:
- **Numbers:** Integer literals (e.g., `42`) and floating-point numbers (e.g., `3.14`).
- **Booleans:** `true` and `false`.
- **Strings:** Enclosed in double quotes (e.g., `"Hello, World!"`).
- **Names:** Identifiers used for variable names and function names.

Example:
```mussel
"Hello, Mussel!"    // A string literal
42                  // An integer literal
3.14                // A float literal
true                // A boolean literal
```

### Let Bindings

You can bind a value to a variable using the `let` keyword. Let bindings allow you to store values that can later be referenced in expressions.

Example:
```mussel
let greeting = "Hello, World!"
println(greeting)  // Prints: Hello, World!
```

### Function Calls and Definitions

Functions are defined with the `fn` keyword. A function includes a name, parameters, and a body wrapped in curly braces. Function calls pass arguments within parentheses.

Example of function definition:
```mussel
fn add(a, b) {
    // Returns the sum of a and b
    return a + b
}
```

Function call:
```mussel
let result = add(5, 7)
println(result)  // Prints: 12
```

**Note:** In Mussel, functions are stored as closures in the interpreter's context, allowing for simple first-class function support.

### Closures

Closures are anonymous functions that capture their surrounding environment. They are defined using vertical bars `|` to enclose parameters, followed by the expression body.

Example:
```mussel
let greet = |name| "Hello, " + name + "!"
println(greet("Mussel"))
```

### Control Flow: if and for

#### If Statements

The `if` statement evaluates a condition and executes a block of code if the condition is true. An optional `else` block can handle the false case.

Example:
```mussel
if true {
    println("This is true!")
} else {
    println("This is false!")
}
```

#### For Loops

The `for` loop iterates over an array. For each item in the array, it binds the item to a variable and evaluates the loop body.

Example:
```mussel
let numbers = [1, 2, 3, 4, 5]
for num in numbers {
    println(num)
}
```

### Arrays and Element Access

Arrays are defined with square brackets and elements separated by commas. You can access elements using the `Get` expression (using square brackets after a variable name).

Example:
```mussel
let fruits = ["apple", "banana", "cherry"]
println(fruits[1])  // Prints: banana
```

### String Interpolation

Mussel supports interpolation inside string literals. When a string contains curly braces `{...}`, the interpreter attempts to parse and evaluate the expression inside the braces, then inserts the result into the string.

Example:
```mussel
let name = "Alice"
println("Hello, {name}!")  // Prints: Hello, Alice!
```

## Tutorial & Practical Example

Below is a sample Mussel script that demonstrates several features of the language:

```mussel
// Define a function to add two numbers
fn add(a, b) {
    return a + b
}

// Use a let binding to store a greeting
let greeting = "Hello, Mussel!"

// Print the greeting
println(greeting)

// Demonstrate string interpolation
let name = "Bob"
println("Welcome, {name}! Let's do some math:")

// Call the add function and store the result
let sum = add(10, 20)
println("10 + 20 = {sum}")

// Create an array of numbers
let numbers = [1, 2, 3, 4, 5]

// Iterate over the array and print each number
for num in numbers {
    println("Number: {num}")
}

// Conditional execution with if statement
if sum > 25 {
    println("The sum is greater than 25!")
} else {
    println("The sum is 25 or less!")
}
```

### Running the Example

1. Save the above script as `example.mus`.
2. Run the interpreter with the script:
   ```bash
   make run FILE=example.mus
   ```
3. You should see output similar to:
   ```
   Hello, Mussel!
   Welcome, Bob! Let's do some math:
   10 + 20 = 30
   Number: 1
   Number: 2
   Number: 3
   Number: 4
   Number: 5
   The sum is greater than 25!
   ```

## How It Works

Mussel's interpreter is divided into several modules:

- **Main Module (`main.rs`):**  
  Parses command-line arguments, reads the input file, and sets up error handling with `color_eyre`. It then parses the file content into expressions and evaluates them.

- **Parser Module (`parser.rs`):**  
  Uses the `nom` and `nom_supreme` libraries to define combinators for parsing Mussel syntax. It supports parsing atoms, let-bindings, function definitions, closures, control flow constructs, and array operations. The parser is designed to return a structured abstract syntax tree (AST) that the interpreter can evaluate.

- **Interpreter Module (`interpreter.rs`):**  
  Walks the AST and evaluates expressions. It manages a context (a symbol table) for variable bindings, executes functions (both named and closures), handles control flow, and implements built-in functions like `println`. Special attention is given to evaluating string interpolation by repeatedly evaluating embedded expressions until they stabilize.