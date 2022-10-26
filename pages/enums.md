# Enums

Enums allow us to define a type by enumerating its possible values.

```rs
enum IpAddr {
    V4,
    V6,
}

fn main() {
    let four = IPAddr::V4;
}
```

Enums can have associated values, and each variant can have different types and amounts of associated data.

```rs
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback)
}
```

Associated values can be scalars, compound types, structs, and even other enums.

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

These are shorthand constructs for the following associated structs.

```rs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String) // tuple struct
struct ChangeColorMessage(i32, i32, i32) // tuple struct
```

We are also able to implement functions on enums.

```rs
impl Message {
    fn call(&self) {
        // ...
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

# match control flow operator

Rust has an extremely powerful control flow operator called `match` for pattern matching. It compares values against a series of patterns and then executes code based on which pattern matches. It is an expressive way to handle control flow and lets the compiler check that all possible cases are handled.

Because Rust is an expression-based language, the `match` operator can also be used as an expression.

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The `match` arms have two parts: a pattern and some code. When the `match` expression executes, **it compares the value against each arm in order**. If the pattern matches the value, the associated code is executed. If the pattern doesn't match, execution continues to the next arm. In other words, the first matching arm will be executed.

The code associated with each arm is an expression and the resulting value of the expression in the matching arm is returned for the entire `match` expression.

## Patterns that bind to values

Match arms can bind to the parts of the values that match the pattern. This is how we can extract values out of the enum variants.

```rs
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama, Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

A common use for matching with binding values is in the `Option<T>` enum.

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

## `_` placeholder

Matches must be exhaustive and handle all cases. If we don't want to list all possible values, we can use the special `_` pattern.

```rs
let u8_value = 0u8;

match u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    _ => (),
}
```

# if let control flow operator

If we only care about one case, it may be too verbose to use the `match` statement. Instead, `if let` gives us a quick way to handle values that match one pattern while ignoring the rest.

```rs
let some_u8 = Some(3);

match some_u8 {
    Some(3) => println!("three"),
    _ => (),
}

if let Some(3) = some_u8 {
    println!("three");
}
```

`if let` also supports value bindings.

```rs
let coin = Coin::Quarter(UsState::Alaska);

match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => println("Not a quarter"),
}

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    println("Not a quarter");
}
```
