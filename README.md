# Mussel

<div align="center">
<img src="doc/images/logo.png" height=250>
</div>

**Mussel** is an interpreted programming language written in Rust. It was designed as a simple and flexible language with a focus on easy-to-read syntax and powerful expression evaluation. Mussel supports basic data types, variable bindings, functions, closures, control flow constructs (such as `if` and `for`), and even string interpolation.

## Tutorial

You can find a detailed tutorial to learn how Mussel works [here](/doc/Tutorial.md).

## Practical Example

You can find some examples of Mussel code in the [examples](/examples/) folder.

## How to run Mussel code?

1. **Install Rust:**

   Rust is required to build Mussel code. You can download it from [rust-lang.org](https://www.rust-lang.org/).

2. **Clone the repo:**

    ```
    git clone https://github.com/gianndev/mussel.git
    cd mussel
    ```

3. **Compile the Rust code:**
    
    To build the release version of Mussel you can use the Makefile just typing
    ```
    make release
    ```

4. **Run Mussel code:**

    Once you've created a file with the **.mus** file extension (the official extension of Mussel) you can run the Mussel code typing in the terminal
    ```
    make run FILE=path/to/the/file.mus
    ```
    Make sure to insert the correct path of the Mussel file

## Version

The current latest version of Mussel is 0.0.2

## License

Mussel is released under the Apache License 2.0
