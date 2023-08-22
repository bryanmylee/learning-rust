use std::ops::Deref;

fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // &m is `&MyBox<String>`, which gets coerced into `&String` by `deref`.
    // The standard library defines `Deref` for `String` which coerces into `&str`.
    hello(&m);

    let c = CustomSmartPointer(String::from("my stuff"));
    let d = CustomSmartPointer(String::from("other stuff"));
    std::mem::drop(c);
    println!("CustomSmartPointers created.")
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer(String);

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.0);
    }
}
