# Smart pointers

A pointer is a general concept for a variable that contains an address in memory.

The most common kind of pointer in Rust is a reference. References are indicated by the `&` symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of pointer we use most often.

_Smart pointers_ on the other hand are data structures that not only act like a pointer but also have additional metadata and capabilities.

Unlike references which only borrow the data they refer to, smart pointers **own** the data they point do.

Smart pointers are usually implemented using structs that implement the `Deref` and `Drop` traits.

- the `Deref` trait allows an instance of the smart pointer to behave like a reference so you can write code that works with either references or smart pointers.
- the `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

The most common smart pointers in the standard library are:

- `Box<T>` for allocating values on the heap.
- `Rc<T>` for a reference counting type that enables multiple ownership.
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, which enforces the borrowing rules **at runtime instead of compile time**.

In addition, we'll cover the _interior mutability_ pattern where an immutable type exposes an API for mutating an interior value, and _reference cycles_ and how they can leak memory.

# Using `Box<T> to point to data on the heap`

Boxes allow us to store data on the heap rather than the stack. Boxes don't have performance overheads other than being stored on the heap. They are most commonly used when:

- you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size (e.g. on the stack).
- you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so.
- when you want to own a value and only care that it's a type that implements some trait rather than being of a specific type.

```rs
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

We create a new box with `Box::new`, passing the referenced value as the first argument. Then, we can use the value returned just like any other variable.

Just like any owned value, when the box goes out of scope, it will be deallocated. The deallocation occurs for both the box on the stack and the data it points to on the heap.

## Recursive types with Boxes

At compile time, Rust needs to know how much space a type takes up. One type whose size cannot be known at compile time is a _recursive type_. Because the recursion could go on infinitely, Rust cannot pre-determine its size on the stack.

### Cons list

A _cons list_ is a data structure that comes from Lisp. The `cons` function creates a pair from its arguments, which are usually a single value and another pair. A cons list is basically a singly-linked list.

```rs
enum List {
    Cons(i32, List), // recursive without indirection.
    Nil,
}
```

Rust determines the size of its types statically. For sum / enum types, Rust uses the size of the variant that needs the most space. For product / struct types, Rust adds the size of each field

Because our recursive type refers to itself, it will have infinite size. To get a known size for a recursive type, we have to put it on the heap.

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

# Treating smart pointers like regular references with `Deref`

Implementing `Deref` allows us to customize the behavior of the _dereference operator_ `*`. In doing so, we can write code that operates on references and use that code with smart pointers too.

## Following the pointer to the value

A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.

```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Comparing a number and a reference to a number is not allowed. We must use the `*` operator to follow the reference. If we used a `Box<T>` type, this rule remains.

```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

## Defining our own smart pointer

```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

To make `MyBox` act like a reference, implement the `Deref` trait.

```rs
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

At compile time, Rust substitutes the `*` operator with a call to `deref` then a plain dereference. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements `Deref`. Note that `deref` still only returns a reference to the value due to the ownership system. We don't want to move ownership of the value out of the box.

```rs
*(y.deref())
```

## Mutable `Deref`

Similar to how `Deref` overrides the `*` operator on immutable references, `DerefMut` overrides the `*` operator on mutable references.

## Implicit deref coercions

Deref coercion converts a reference to a type that implements `Deref` into a reference to a type that `Deref` can convert the original type into.

Deref coercion happens automatically when we pass a reference to a particular type's value as an argument to a function or method with a non-matching signature.

```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}
```

Deref coerion makes it possible to call `hello` with a reference to a value of type `MyBox<String>` or a string slice.

Rust can turn `&MyBox<String>` into `&String` by calling `deref`. The standard library provides an implementation of `Deref` on `String` that returns a string slice, so Rust then calls `deref` again to turn `&String` into `&str`.

When `Deref` is defined for the types involved, Rust will use `Deref::deref` as many times as necessary at compile time to get a reference to match the parameter's type.

Rust does deref coercion in three cases:

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

# Running code on cleanup with `Drop`

The second important trait to the smart pointer pattern is `Drop`, which lets us customize what happens when a value goes out of scope.

`Drop` can be implemented for any type, and the code can be used to release resources like files or network connections.

```rs
fn main() {
    let c = CustomSmartPointer(String::from("my stuff"));
    let d = CustomSmartPointer(String::from("other stuff"));
    println!("CustomSmartPointers created.")
}

struct CustomSmartPointer(String);

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.0);
    }
}
```

Note that variables are dropped in reverse order of their creation.

## Dropping values early

Occasionally, we might want to clean up a value early. One example is when using smart pointers that manage locks: we might want to force the `drop` method that releases the lock to run so other code in the same scope can acquire the lock.

We cannot call `Drop::drop` directly because Rust would still automatically call `drop` on the value at the end of the scope. This would cause a _double free_ error.

Instead, we have to use the `std::mem::drop` function.

```rs
fn main() {
    let c = CustomSmartPointer(String::from("my stuff"));
    let d = CustomSmartPointer(String::from("other stuff"));
    std::mem::drop(c);
    println!("CustomSmartPointers created.")
}
```
