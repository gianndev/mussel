# Mussel's Standard Library Tutorial

This file contains a list of all the libraries in the Mussel Standard Library, and for each one the commands are explained.

Mussel's Standard Library currently consists of the following libraries:

- random
- string
- time
- math
- os

---

## random

The `random` library provides basic utilities for generating random numbers in Mussel programs. To use this library, you must include it at the top of your script with:

```
include random
```

### Functions

#### `rand(min, max)`

Generates a random number between `min` and `max`, inclusive.

- **Arguments:**
  - `min`: a numeric value (integer or float), representing the lower bound of the random number range.
  - `max`: a numeric value (integer or float), representing the upper bound of the random number range.
- **Returns:** A random integer between `min` and `max` (rounded if float inputs are used).
- **Example:**

```mussel
include random

let n = rand(1, 10)
println("Your random number is {n}")
```

This will output a random number like:

```
Your random number is 7
```

#### Notes:

- If you pass floating-point numbers, the result is still returned as an integer using rounding.
- If `min` is greater than `max`, the program will panic.
- Both arguments must be constants or evaluable expressions resulting in numeric values.

---

## string

The `string` library provides utilities for manipulating and working with strings in Mussel programs. To use this library, you must include it at the top of your script with:

```
include string
```

### Functions

#### `length(s)`

Returns the length of the string `s`.

- **Arguments:**
  - `s`: a string whose length is to be calculated.
- **Returns:** An integer representing the number of characters in the string.
- **Example:**

```mussel
include string

let len = length("hello")
println("The length of the string is {len}")
```

This will output:

```
The length of the string is 5
```

#### `concat(s1, s2)`

Concatenates two strings `s1` and `s2` and returns the result.

- **Arguments:**
  - `s1`: the first string.
  - `s2`: the second string.
- **Returns:** A new string that is the result of concatenating `s1` and `s2`.
- **Example:**

```mussel
include string

let fullName = concat("John", " Doe")
println("Full name: {fullName}")
```

This will output:

```
Full name: John Doe
```

#### Notes:

- Strings in Mussel are immutable, so operations like `concat` return a new string without modifying the originals.
- Passing non-string arguments to these functions will result in a runtime error.

---


## time

The `time` library provides utilities for handling time in Mussel programs. 

```
include time
```

### Functions

#### `time_ms()`

Returns the time in milliseconds since the Unix epoch (January 1, 1970).

- **Arguments:**
  - None
- **Returns:** An integer representing the current time in milliseconds.
- **Example:**

```mussel
include time

let time = time_ms()
println("Current time in milliseconds: {time}")
```

#### `time_sec()`

Returns the time in seconds since the Unix epoch (January 1, 1970) as a floating-point number.

- **Arguments:**
  - None
- **Returns:** An Float representing the current time in seconds.
- **Example:**

```mussel
include time

let start = time_sec()
complex_operation()  # Replace with your operation
let delta = time_sec() - start
println("Operation took {delta} seconds")
```


#### Notes:

- This library is still in development and more functions may be added in the future.
- There will be more functionality for handling dates and times, such as formatting and parsing.

---

## math

The `math` library provides basic arithmetic and mathematical functions for Mussel programs. To use this library, include it at the top of your script with:

```
include math
```

### Functions

#### `abs(x)`

Returns the absolute value of the numeric value `x`.

- **Arguments:**
  - `x`: a number (integer or float)
- **Returns:** The absolute value of `x`
- **Example:**

```mussel
include math

let a = abs(-15)
println("The absolute value is {a}")
```

#### `sqrt(x)`

Returns the square root of the numeric value `x`.

- **Arguments:**
  - `x`: a number (integer or float)
- **Returns:** A floating-point number representing the square root of `x`
- **Example:**

```mussel
include math

let root = sqrt(25)
println("Square root of 25 is {root}")
```

#### `pow(base, exponent)`

Raises a number `base` to the power of `exponent`.

- **Arguments:**
  - `base`: a numeric value (integer or float)
  - `exponent`: a numeric value (integer or float)
- **Returns:** A floating-point number representing the result of `base` raised to the power of `exponent`
- **Example:**

```mussel
include math

let power = pow(2, 3)
println("2 to the power of 3 is {power}")
```

#### Notes:

- Ensure the arguments provided to these functions are numeric values; otherwise, a runtime error may occur.
- The `sqrt` function always returns a floating-point number.

---

## os

The `os` library provides functionalities for performing operating system-level operations in Mussel programs. To use this library, include it at the top of your script with:

```
include os
```

### Functions

#### `getcwd()`

Returns the current working directory as a string.

- **Arguments:**
  - None
- **Returns:** A string representing the current working directory.
- **Example:**

```mussel
include os

let cwd = getcwd()
println("Current working directory: {cwd}")
```

#### `listdir(path)`

Lists all entries in the specified directory.

- **Arguments:**
  - `path`: a string representing the directory path.
- **Returns:** An array of strings, each representing an entry (file or directory) in the specified path.
- **Example:**

```mussel
include os

let entries = listdir("c:/Users/mark")
println("Directory entries: {entries}")
```

#### `exists(path)`

Checks if a specified path exists.

- **Arguments:**
  - `path`: a string representing the file or directory path.
- **Returns:** A boolean (`true` or `false`) indicating whether the path exists.
- **Example:**

```mussel
include os

let flag = exists("c:/Users/mark")
println("Does the path exist? {flag}")
```

#### Notes:

- Ensure the provided path is a valid string.
- If the supplied path for `listdir` does not exist or is inaccessible, an error will be thrown.

---

More libraries and functionality will be added to the standard library as Mussel evolves. Stay tuned!