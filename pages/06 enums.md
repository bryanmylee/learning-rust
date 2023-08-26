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
