# Frutta scripting language

## Work in progress

Frutta is a scripting language that is designed to be easy to use and understand. It is a work in progress and is not yet ready for use.

## Installation

### Installing with cargo

The easiest way to install Frutta is with cargo, the Rust package manager.
It is not yet published on crates.io, so you will have to install it from the git repository (I will publish it on crates.io when it will be more stable and useable).

```bash
cargo install --git https://github.com/Fytecas/frutta-lang.git
```

And then you can run it with `frutta`.



## Usage

Frutta code is written in `.fru` files.
For example, you can write the following code in a file called `hello.fru`:

<!-- I use javascript for making it look better in github-->
```javascript
Std.print("Hello, world!")
```

You can then run the code with the following command:

```bash
frutta hello.fru
```

For more information, you can run `frutta --help`.

## Syntax

Frutta is a class-based language, it is inspired by Python and Wren.

### Comments

Comments start with '//' and go until the end of the line.

```javascript
// This is a comment
```

Blocks of comments are not supported yet. <!-- TODO: Support comments block -->

### Variables

Variables definitions are very similar to Python, you just have to write the variable name, then an equal sign, and then the value.

```javascript
a = 5
b = "Hello, world!"
```

### Functions

Functions are defined with the `fn` keyword, followed by the function name, then the arguments in parentheses, and finally the function body in curly braces.

```rust
fn add(a, b) {
    return a + b
}
```

### Classes

User-defined classes are not supported yet. <!-- TODO: Support classes -->

## Standard library

The standard library is very limited for now, but it will be expanded in the future.
It basically contains all the native, built-in functions.
You can access the standard library with the `Std` object.

### Std.print

Prints a string to the standard output.

```javascript
Std.print("Hello, world!")
```

### Std.input

Reads a string from the standard input, similar to Python's `input`.

```javascript
name = Std.input("What is your name? ")
Std.print("Hello, " + name + "!")
```

### Std.Time

The `Time` object contains functions to get the current time.

#### Time.now

Returns a DateTime object representing the current time.

```javascript
now = Std.Time.now()
Std.print(now.format("%Y-%m-%d %H:%M:%S"))
```

#### Time.sleep

Sleeps for a given number of seconds.

```javascript
Std.print("Sleeping for 2 seconds...")
Std.Time.sleep(2)
Std.print("Done!")
```
