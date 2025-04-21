# Mussel Tutorial

Consider this file as a tutorial that explains how the **Mussel programming language** works.

## Data types

Mussel supports the following basic data types:

- **Strings**: only supported between double quotes `"`
- **Integers**: both positive and nevative numbers are supported
- **Float numbers**: they use the dot `.` to separate the whole part from the decimal part in numbers (like `3.14`)
- **Booleans**: they are basically only `true` and `false` (both must be lowercase)
- **Arrays**: a list of values stored between square brackets and separated by a `,` (like `[1, 2, 3, 4]`)

## Variables
Mussel uses the keyword `let` to define a variable, using as logic the name of the variable, then `=` and then the value of the variable

```
let name = "John" // string
let age = 45 // integer
let is_male = true // boolean
let letters = ["a", "b", "c", "d", "e", "f"] // array
```

⚠️ *There is no need to manually specify the data type and no need to put a `;` at the end of each line: Mussel is smart enough to understand everything by itself*

For variable names, the use of `_` has recently been supported. This allows both Camel case and Snake case notation to be used.

## println() function
To print somethin on screen, like the content of a variable, you can use the `println()` function

```
let greet = "Hello dude!"
println(greet)
```

## input() function
To allow a user to enter data and have this data saved in a variable, the `input()` function is used

```
let name = input("Enter your name: ")
println("Hello {name}")
```

## Conditionals: *if* and *else*
Conditionals are represented by the keywords `if` and `else`, followed by the condition and then curly braces are needed to contain the part of the code that is executed if the condition is met

The Mussel language supports the following comparison operators:

| Operator     | Meaning              |
|--------------|----------------------|
| **`==`**     | Is equal to          |
| **`!=`**     | Is not equal to      |
| **`<`**      | Less than            |
| **`>`**      | Greater than         |
| **`<=`**     | Less than or equal   |
| **`>=`**     | Greater than or equal|

Below there is an example of Mussel code that uses everything we have seen until now:

```
let number = 0

if number > 0 {
    println("{number} is positive.")
} else if number < 0 { 
    println("{number} is negative.")
} else { 
    println("The number is zero.")  
}
```

## *for* loop
The for loop iterates over an array. For each item in the array, it binds the item to a variable and evaluates the loop body.

```
let numbers = [1, 2, 3, 4, 5]

for num in numbers {
    println(num)
}
```

## *until* loop
The until loop executes a code as long as a given condition is true: it is the equivalent of the *while loop* in Mussel (in fact i think *until* is a better key-word than *while*)

```
let variable = 1

until variable != 1 {
    println("PRINTED")
    let variable = 2
}
```

## Arrays
Arrays are defined with square brackets and elements separated by commas. You can access elements using the Get expression (using square brackets after a variable name).

⚠️ *Like most of programming languges out there, in Mussel the first element of the array has position 0, and the second element has the position number 1*

```
let fruits = ["apple", "banana", "cherry"]
println(fruits[1])
```

## String Interpolation
Mussel supports interpolation inside string literals. When a string contains curly braces {...}, the interpreter attempts to parse and evaluate the expression inside the braces, then inserts the result into the string.

```
let name = "Alice"
println("Hello, {name}!")
```

## Functions
Functions are defined with the `fn` keyword. A function includes a name, parameters, and a body wrapped in curly braces. Function calls pass arguments within parentheses.

Example of function definition:

```
// Defined a function to add two numbers
fn add_numbers(a, b) {
    return a + b
}

let result = add_numbers(5, 7)
println(result)
```

*Note: In Mussel, functions are stored as closures in the interpreter's context, allowing for simple first-class function support.*

For function names, the use of `_` has recently been supported. This allows both Camel case and Snake case notation to be used.

## Comments
To add a comment in Mussel code, use `//` for inline comments.

```
// This is a comment
println("Hello") // prints a string
```

⚠️ *Multiline comments are not supported yet, but you can use `//` for multiple lines if necessary*

## Standard Library

In addition to all the stuff mentioned so far, which is included in "pure Mussel", it is possible to use external modules that allow you to extend the capabilities of Mussel, and which together represent the Mussel Standard Library.

In a *.mus* file you can include a library thanks to the keyword `include`, which must be placed at the beginning of the code, followed by the name of the library you want to import

```
include random

let n = rand(1, 10)
println("Your random number is {n}")
```

For detailed information on the individual libraries of the Standard Library, you can consult the dedicated file [here](/doc/Standard_Library.md).