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

# Interior mutability

_Interior mutability_ is a design pattern in Rust that allows mutation of data even when there are immutable references to that data.

To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.

We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that. The `unsafe` code is then wrapped in a safe API and the outer type is still immutable.

## Enforcing borrowing rules at runtime with `RefCell<T>`

The `RefCell<T>` type represents single ownership over the data it holds. Unlike `Box<T>`, the borrowing rules' invariants are enforced **at runtime**. If we break these rules, instead of a compiler error, the program will panic and exit.

Usually, compile-time checks are better as they allow bugs to be caught sooner in the development process, and there is no impact on runtime performance as all analysis is done beforehand.

Some ownership rules are impossible to check statically, and Rust makes the safe and conservative assumption to reject them. However, there are certain memory-safe situations where we can be sure our code follows the borrowing rules. `RefCell` allows us to accomplish this goal.

## A mutable borrow to an immutable value

Due to borrowing rules, when we have an immutable value, we cannot borrow it mutable.

```rs
fn main() {
    let x = 5;
    let y = &mut x; // fails to compile.
}
```

There are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code. Using `RefCell<T>` is one way to get this ability. However, `RefCell<T>` does not get around the borrowing rules completely. The rules are just deferred to runtime checks instead.

### Interior mutability for mock objects

A _test double_ is a general programming concept for a type used in place of another type during **testing**. _Mock objects_ are specific types of test doubles that record what happens during a test so we can assert that the correct actions occurred.

## Tracking borrows at runtime with `Refcell<T>`

We use `&` and `&mut` to create immutable and mutable references. Similarly with `RefCell<T>`, we use the `borrow` and `borrow_mut` methods which are part of the safe API that belongs to `RefCell<T>`. The `borrow` and `borrow_mut` methods return the smart pointer types `Ref<T>` and `RefMut<T>`, which implement `Deref` so they can be treated like regular references.

`RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. The respective counts increment whenever `borrow` or `borrow_mut` are called. When a `Ref<T>` or `RefMut<T>` goes out of scope, the respective counts decrement. Just like compile-time checks, `RefCell<T>` enforces either one unique `RefMut<T>` or multiple `Ref<T>` only at any given time.

# Multiple owners of mutable data

With an `Rc<T>` that holds a `RefCell<T>`, we can have multiple owners of mutable data.

```rs
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
```
