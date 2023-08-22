# Multiple ownership with `Rc<T>`

`Rc<T>` is a reference counted smart pointer.

Calling `Rc::clone` on the pointer creates a new reference to the value and increments the reference count. When a reference is dropped, it decrements the reference count.

The value will only be cleaned up when the reference count reaches 0.

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use create::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

We could have called `a.clone()`, but Rust's convention is to use `Rc::clone` as it does not make a deep copy of the data as most types' implementations of `clone` do. `Rc::clone` only increments the reference count which doesn't take much time.

This allows us to **visually distinguish** between the deep-copy kinds of clones and the reference-count-incrementing kinds of clones.

We can get the strong count of an `Rc` with `Rc::strong_count`.

```rs
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```
