# Mussel's Standard Library Tutorial

This file contains a list of all the libraries in the Mussel Standard Library, and for each one the commands are explained.

Mussel's Standard Library currently consists of the following libraries:

- random
- string

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

#### `split(s1, s2)`

Splits string `s1` by using `s2` as a delimiter.

- **Arguments:**
  - `s1`: the first string which is to be splitted.
  - `s2`: the second string which is the delimeter.
- **Returns:** An array of strings
- **Example:**

```mussel
include string

let parts = split("John,Doe,Jane,Doe", ",")
println("Parts: {parts}")
```

This will output:

```
Parts: [John, Doe, Jane, Doe]
```

#### `reverse(s)`

Reverses string `s`.

- **Arguments:**
  - `s`: a string which will be reversed
- **Returns:** a new string which is the reverse of `s`
- **Example:**

```mussel
include string

let name = "john"
println("Reversed name: {reverse(name)}")
```

This will output:

```
Reversed name: nhoj
```

#### `trim(s)`

Trims string `s`.

- **Arguments:**
  - `s`: a string which will get trimmed
- **Returns:** a new string without leading and trailing whitespace
- **Example:**

```mussel
include string

let name = "  John   "
println("Trimmed name: {trim(name)}")
```

This will output:

```
Trimmed name: John 
```

#### `ltrim(s)`

Left trims string `s`.

- **Arguments:**
  - `s`: a string which will get trimmed
- **Returns:** a new string without leading whitespace
- **Example:**

```mussel
include string

let name = "  John   "
println("Trimmed name: {ltrim(name)}")
```

This will output:

```
"Trimmed name: John   "
```

#### `rtrim(s)`

Right trims string `s`.

- **Arguments:**
  - `s`: a string which will get trimmed
- **Returns:** a new string without trailing whitespace
- **Example:**

```mussel
include string

let name = "  John   "
println("Trimmed name: {rtrim(name)}")
```

This will output:

```
Trimmed name:   John
```

#### Notes:

- Strings in Mussel are immutable, so operations like `concat` return a new string without modifying the originals.
- Passing non-string arguments to these functions will result in a runtime error.

---
