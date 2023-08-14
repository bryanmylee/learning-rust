# Ownership

There are some basic rules of ownership:

- Each value in Rust has a variable that is its **owner**.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Scoping

Curly braces define a scope in which a variable exists. Variables declared inside a scope are not valid outside the scope and will be dropped.

```rs
{
    let s = "hello";
}
// `s` no longer valid.
```

## Strings

The string type is more complex than the scalar and compound types as it is stored on the heap and its memory must be cleaned up.

String literals are hardcoded into the program and are immutable. To store strings which are dynamically sized, we use the `String` type. `String` provides a convenience namespaced function `::from`.

```rs
let s = String::from("hello");
```

`String` can be mutated.

```rs
let mut s = String::from("hello");
s.push_str(", world!"); // push_str relies on a mutable `self`.
```

## Move semantics

When using non-primitive data types, assigning a variable to another variable does not copy its value but instead moves ownership of the value to the new variable. This is because the `String` is a reference to some data on the heap.

- copying the data in heap may be expensive
- sharing a reference (double ownership) will lead to a _double free_ error when both variables go out of scope and try to free the same memory.

```rs
let s1 = String::from("Hello");
let s2 = s1;

println!("{}, world", s1); // value used here after move
```

## Clone

If we do want to deeply copy the heap data of the `String`, we use a common method called `clone`.

```rs
let s1 = String::from("Hello");
let s2 = s1.clone();
```

Primitives are automatically cloned on assignment.

```rs
let s1 = 5;
let s2 = s1;
```

In addition, all types that implement the `Copy` trait are automatically cloned. Some common types that implement `Copy` are:

- all integer types.
- Boolean `bool`.
- character type `char`.
- all floating point types.
- tuples if they only contain types that are also `Copy`.

## Ownership and functions

Passing a variable to a function will move non-`Copy` types and clone `Copy` types.

```rs
let s = String::from("hello");
takes_ownership(s);
// `s` no longer valid.
let x = 5;
makes_copy(x);
```

Returning values will also transfer ownership.

```rs
let s = String::from("hello")
let s = takes_and_gives_back(s);
// `s` is valid again because `takes_and_gives_back` returns ownership.
```

# References and borrowing

## Rules of references

- At any time, we can have either but not both: one mutable reference; or multiple immutable references.
- References must always be valid.

If we do not want a function to take ownership, we use **references**.

```rs
fn main() {
    let s = String::from("hello");
    // Create a reference to `s`.
    let len = calculate_length(&s);
}

// Automatically dereferences the reference.
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

We cannot get a mutable reference to borrowed content.

```rs
fn change(s: &String) -> usize {
    s.push_str(", world"); // Because push_str tries to take a mutable reference to self, this fails.
}
```

## Mutable references

We can create and pass mutable references.

```rs
fn main() {
    let mut s = String::from("hello");
    let len = calculate_length(&mut s);
}

fn change(s: &mut String) -> usize {
    s.push_str(", world");
}
```

The limitation with mutable references is that **we can only have one mutable reference to any data in a particular scope**.

```rs
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    println!("{} {}", r1, r2);

}
```

To resolve this, we can use curly braces to create new scopes.

```rs
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // the mutable reference goes out of scope and is dropped here.
    let r2 = &mut s;

    change(&mut s);
    // this is fine because the previous reference only exists in the scope of the function arguments.
    change(&mut s);

    println!("{}", s);
}
```

**We also cannot borrow a mutable reference to a variable that has immutable references**. That is because users of an immutable reference do not expect the values to change from under them.

## Dangling references

Rust prevents dangling references.

```rs
fn main() {
    let r = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s will be dropped when this function goes out of scope, therefore &s will refer to invalid memory.
```

# Slices

Slices let us reference a **contiguous sequence of elements in a collection without taking ownership**.

Internally, slices keep track of a pointer to the starting element and a length. However unlike manually managing a pointer and length, slices are tied to the original value. Therefore, we do not have to worry about the pointer or length being out of sync with the original value.

```rs
let s = String::from("hello world");
let hello = &s[0..5]; // pointer to the start with length 5.
let world = &s[6..11]; // pointer to the 7th character with length 5.
```

```rs
// Defining the signature with `&str` allows both string literals and `String` to be used.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let hello = first_word(&s[..]); // `hello` has an immutable borrowed reference to `s`.
    s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable.
}
```

```rs
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

This slice has type `&[i32]`.

## Ranges

A range is represented with `start..end`.

`start` can be omitted to start from `0`, `end` can be omitted to end on `len`, and both can be omitted to create a range over the whole collection.

Ranges can be used to create slices.

```rs
let s = String::from("hello world");
let slice = &s[..];
```

> String slice range indices must occur at valid UTF-8 character boundaries.

String literals are also slices to a specific point in the binary, and therefore are immutable.
