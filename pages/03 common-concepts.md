# Variables and Mutability

Variables are immutable by default.

```rs
fn main() {
    let x = 5;
    x = 6; // cannot assign twice to immutable variable
}
```

To allow mutations, use `mut`.

```rs
fn main() {
    let mut x = 5;
    x = 6;
}
```

## Constants

Unlike variables, constants cannot be mutable and cannot be used with `mut`. **Constants are always immutable**.

Declare constants with `const`.

```rs
fn main() {
    const MAX_POINTS: u32 = 100_000;
}
```

## Shadowing

Shadowing is useful for re-defining a previous variable.

```rs
fn main() {
    let x = 5;
    let x = 6;
}
```

Shadowing allows us to perform multiple transformations on a value but have the variable be immutable after those transformations.

We can also change the type of the variable but re-use the same name.

# Data types

Every value in Rust is of a certain _data type_. All types must be known statically at compile time and can affect the actual implementation of code.

```rs
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

Without the type annotation, Rust will not know which implementation of `parse()` to use.

## Scalar types

Scalars represent a single value. There are four scalars in Rust: integers, floating-point numbers, Booleans, and characters.

### Integers

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

Integers can be written in multiple literal forms.

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

If unsure, the default `i32` is generally a good choice.

### Floating-points

Rust has `f32` and `f64` for 32-bit and 64-bit floating point numbers.

### Boolean

The Boolean type has two possible values (`true` and `false`) and are one byte in size.

```rs
fn main() {
    let t: bool = true;
}
```

### Character

Character literals are defined with single quotes whereas strings are defined with double quotes.

Rust's char type is four bytes and represents a Unicode Scalar Value.

## Compound types

Compound types group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple

A tuple is a general way of grouping together some number of values with a variety of types into one compound type.

Tuples have fixed length and cannot grow or shrink once declared.

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

We can destructure tuples to get multiple values with pattern matching, or access the values with dot notation.

```rs
println!("The value of x is: {}", tup.0);
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```

### Array

An array is a fixed-length collection of values with the same type.

```rs
let arr = [1, 2, 3, 4, 5];
```

An array's length is part of its type.

```rs
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

We can initialize an array with the same value for all its elements with a convenience syntax.

```rs
let arr = [3; 5]; // creates an array of length 5 filled with 3.
```

We access values with bracket notation.

```rs
let first = arr[0];
```

# Functions

All function declarations start with `fn` and end with a bracket containing arguments.

```rs
fn function(x: i32) {
    println!("The value of x is: {}", x);
}
```

## Statements and expressions

Function bodies consist of a series of statements optionally ending in an expression.

Statements terminate in a `;` and do not return values.

Rust is an expression-based language which means many constructs are simply expressions and can be treated as values. Expressions at the end of blocks or function bodies will be implicitly returned.

```rs
let y = {
    let x = 3;
    x + 1
} // The value of y is 4.
```

To return a value early, use the `return` expression.

## Control flow

### if expressions

```rs
let number = 3;

if number < 3 {
    println!("Number smaller than 3");
} else if number == 3 {
    println!("Number is 3");
} else {
    println!("Number larger than 3");
}
```

`if` as an expression. Make sure to terminate the assigment statement of the entire `if` block with `;`.

```rs
let y = if x < 3 {
    5
} else {
    6
};
```

Of course, variables must have a single type and all expressions in an `if` block must have the same type.

### loops

```rs
loop {
    println!("again")
}
```

To return values from a loop, use a `break` expression.

```rs
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

### while loops

```rs
let mut number = 3;

while number != 0 {
    println!("{}!", number)
    number = number - 1;
}

println!("LIFTOFF!!");
```

### for loops

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is :{}", element);
    }
}
```

For loops also work over ranges.

```rs
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}
```
