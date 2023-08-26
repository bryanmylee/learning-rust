# Patterns

Patterns can be used in:

- `match` arms
- `if let` conditional expressions
- `let else` conditional expressions
- `while let` conditional loops
- `for` loops
- `let` statements
- Function parameters

## Refutability

Patterns that will match for any possible value passed in are _irrefutable_. Patterns that can fail to match for some values passed in are _refutable_.

Function parameters, `for` loops, and `let` statements can only accept irrefutable patterns, because the program cannot do anything with unmatched values otherwise.

## Pattern syntax

### Matching named variables

Named variables are irrefutable patterns that match any value.

### Multiple patterns

In `match` expressions, we can match multiple patterns using `|`.

```rs
match x {
  1 | 2 => println!("one or two"),
  _ => println!("anything"),
}
```

### Ranges of values

The `..=` syntax allows us to match an inclusive range of values. Ranges are only allowed with numeric or `char` values.

```rs
match x {
  1..=5 => println!("one two three four five"),
  _ => println!("anything"),
}
```

### Destructuring

We can use patterns to destructure structs, enums, tuples, and references.

#### Destructuring structs

```rs
let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);

// shorthand syntax
let Point { x, y } = p;
assert_eq!(0, x);
assert_eq!(7, y);
```

We can also destructure with literal values to create a refutable pattern for matching.

```rs
match p {
    Point { x, y: 0 } => println!("On the x axis at {x}"),
    Point { x: 0, y } => println!("On the y axis at {y}"),
    Point { x, y } => println!("On neither axis: ({x}, {y})"),
}
```

#### Destructuring enums

Enum destructuring can be done over each variant, and will always be refutable. We can also use named variables to store matched values.

```rs
match msg {
    Message::Quit => println!("Quit called"),
    Message::Move { x, y } => {
        println!("Moved ({x}, {y})");
    },
    _ => (),
}
```

#### Destructuring nested structs and enums

```rs
match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
        println!("Changing color to rgb({r} {g} {b})");
    },
    _ => (),
}
```

### Ignoring values in a pattern

#### Ignoring entire values

Use `_` as a wildcard that matches any value without using it.

```rs
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

#### Ignoring parts of values

```rs
let (_, y) = (3, 4);
```

#### Ignoring unused variables

Start variable names with `_` to ignore unused variable warnings.

Note that unlike `_` alone, variables starting with `_` will still get assigned the value and will take ownership of the values they are given.

```rs

let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s); // error, `s` consumed by `_s`.
```

#### Ignoring remaining parts of a value

`..` will expand to as many values as needed to complete the match. This can be useful as a catch-all for struct fields.

```rs
let origin = Point { x: 0, y: 0, z: 0 };

let Point { x, .. } = origin;
```

`..` can also be used in tuples at most once.

```rs
let numbers = (2, 4, 8, 16, 32);

let (first, .., last) = numbers;
```

Note that the ignore syntax is similar to the exclusive range _iterator_, which uses two dots `..`.

To summarize:

- the range pattern matcher uses `..=` to match the inclusive range iterator
- the ignore syntax uses `..`
- there is no equivalent for an exclusive pattern like the exclusive range iterator `..`

### Match guards

A _match guard_ is an additional `if` condition after a `match` arm.

This can be used to provide an additional condition check for the arm to be selected.

```rs
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {x}"),
    Some(x) => println!("{x}"),
    None => (),
}
```

When used with `|`, the `if` condition matches against the whole pattern. Instead of `4 | 5 | (6 if condition)`, it behaves like `(4 | 5 | 6) if condition`.

### `@` bindings

The `@` operator allows us to hold a value at the same time we're testing the value.

```rs
enum Message {
    Hello { id: i32 },
}

match msg {
    Message::Hello { id: matched_id @ 3...7 } => {
        println!("Found an id in range: {matched_id}");
    },
    Message::Hello { id: 10...12 } => {
        println!("Found another id in range");
    },
    _ => (),
}
```
