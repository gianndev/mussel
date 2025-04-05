# Mussel

<div align="center">
<img src="doc/images/logo.png" height=250>
</div>

**Mussel** is an interpreted programming language written in Rust. It was designed as a simple and flexible language with a focus on easy-to-read syntax and powerful expression evaluation. Mussel supports basic data types, variable bindings, functions, closures, control flow constructs (such as `if` and `for`), and even string interpolation.

## Features

- **Simple Syntax:** Mussel syntax is minimalistic and easy to learn.
- **Interpreted Execution:** No separate compilation step is required; simply run your Mussel scripts with the provided interpreter.
- **Dynamic Evaluation:** Supports let-bindings, function definitions, closures, and control flow constructs.
- **Error Reporting:** Uses `color_eyre` for enhanced and user-friendly error messages.
- **Built-in Functions:** Includes common functions like `println` for printing to the console.
- **String Interpolation:** Embed expressions directly within string literals for dynamic output.

## Practical Example

You can find some examples of Mussel code in the `examples/` folder

## How It Works

Mussel's interpreter is divided into several modules:

- **Main Module (`main.rs`):**  
  Parses command-line arguments, reads the input file, and sets up error handling with `color_eyre`. It then parses the file content into expressions and evaluates them.

- **Parser Module (`parser.rs`):**  
  Uses the `nom` and `nom_supreme` libraries to define combinators for parsing Mussel syntax. It supports parsing atoms, let-bindings, function definitions, closures, control flow constructs, and array operations. The parser is designed to return a structured abstract syntax tree (AST) that the interpreter can evaluate.

- **Interpreter Module (`interpreter.rs`):**  
  Walks the AST and evaluates expressions. It manages a context (a symbol table) for variable bindings, executes functions (both named and closures), handles control flow, and implements built-in functions like `println`. Special attention is given to evaluating string interpolation by repeatedly evaluating embedded expressions until they stabilize.