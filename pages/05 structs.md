# Structs

Structs allow us to define custom data types that package together multiple related values. Structs also allow us to define methods and associated functions to specify related behaviors.

## Defining and instantiating structs

We first have to define a struct as a schema of its types.

```rs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

We can then create an instance of the struct by specifying concrete values for each field.

```rs
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

We use dot notation to access values from the struct. To modify the struct, the variable must be mutable. Rust does not allow marking only certain fields as mutable.

```rs
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String.from("anotheremail@example.com");
```

Rust also has property shorthands.

```rs
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

Rust has a convenience method for creating new struct instances from other instances. The `..` notation indicates that all **remaining fields not explicitly set** should have the same values as fields in the given instance.

```rs
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

## Tuple structs

Tuple structs have the added meaning in their struct names but do not have names associated with their fields. Tuple structs are useful when we want to give the whole tuple a name and make the tuple a different type from other tuples i.e. branding.

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## Unit-like structs

We can also define structs without any fields. They are called unit-like because they behave like the unit type `()`. Unit-like structs are useful when we need to implement a trait on some type but have no data to store in the type.

# Derived traits

Derived traits allow us to **derive** meta behaviors on our structs. One example is the `Debug` trait, which allows a debug specifier to determine how to print a struct. We don't have to implement the trait ourselves as the implementation can be derived.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1); // rect1 is Rectangle { width: 30, height: 50 }
}
```

## Print format specifiers

Debug `{:?}`

The debug format uses the `Debug` trait.

Debug with line breaks `{:#?}`

Displays the struct over multiple lines instead of one line.

```rs
println!("rect1 is {:#?}", rect1);
// rect1 is Rectangle {
//     width: 30,
//     height: 50
// }
```

# Method syntax

Methods are functions defined in the context of a struct (or an enum or trait object). The first parameter to methods is always `self` which represents the instance of the struct the method is being called on.

## Defining methods

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define methods, we start an `impl` block. Methods always take `self` as the first argument. A struct can have multiple `impl` blocks which will be merged.

Instead of specifying the type and any reference semantics on the type, we specify them directly on `self` because Rust knows to infer the type of `self` from the `impl` block e.g. `&mut self` instead of `rect: &mut Rectangle`.

Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably. It is rare for methods to take ownership; this technique is usually used when the method transforms `self` and we want to prevent the caller from using the original instance.

# Associated functions

Associated functions do not take `self` as a parameter and instead are just associated with the struct.

```rs
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

We call associated functions with `::`.

```rs
let square = Rectangle::square(6);
```
