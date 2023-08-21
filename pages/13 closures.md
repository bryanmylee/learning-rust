# Closures

Closures are function-like constructs that can be stored in variables. Unlike functions, **closures can capture values from the scope in which they are defined**.

```rs
fn main() {
    let x = 3;

    // can't capture dynamic environment in a fn item.
    fn equal_to_x(z: i32) -> bool { z == x }

    // this works.
    let equal_to_x = |z| z == x;
}
```

If the body of the closure only contains one expression, the curly braces are unnecessary.

```rs
fn main() {
    let increment = |value| value + 1;
}
```

## Closure type inference and annotation

Functions require type annotations because they are part of an explicit contract exposed to users. Defining this contract rigidly is important for ensuring the types and values a function uses and returns.

However, closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the conpiler is reliably able to infer the types of the parameters and the return type. Therefore in most cases, closures do not need to be type annotated.

Of course, we can still be explicit about the types if we want.

```rs
fn main() {
    let increment = |value: u32| -> u32 {
      value + 1
    };
}
```

Note that closures will still only have one type that is inferred. A single closure cannot have multiple inferred possible types. The first time we call a closure, its type will be inferred by the compiler and "locked in".

```rs
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // throws an error.
```

Another thing to note is that even if two closures have the same function signature, their types are considered different. **Each closure instance has its own unique anonymous type**.

## Storing closures using generic parameters and the `Fn` traits

We can use closures for _memoization_ or _lazy evaluation_ by making a struct that holds a closure.

To define structs, enums, or function parameters that use closures, we use generics and trait bounds.

The `Fn` traits are provided by the standard library. All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.

```rs
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

## Capturing environment with closures

When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This use of memory is overhead that we don't want to pay in more common cases where we don't want to capture the environment.

Because functions are never allowed to capture their environment, defining and using functions will never incur this overhead.

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:

1. taking ownership: `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure's _environment_. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The `Once` part of the name represents the fact that the closure can't take ownership of the same variables more than once, so it can only be called once.

2. borrowing mutably: `FnMut` can change the environment because it mutable borrows values.

3. borrowing immutably: `Fn` borrows values from the environment immutably.

When you create a closure , Rust infers which trait to use based on how the closure uses the values from the environment.

All closures implement the `FnOnce` trait (since all closures can be called once). Closures that don't move the captured variables also implement `FnMut`. Closures that don't need mutable access to the captured variables implement `Fn`.

Notice that each class of `Fn` define a subset of the previous. `FnOnce` is a superset of `FnMut`, which is a superset of `Fn`.

If you want to force the closure to take ownership of the values it uses, you can use the `move` keyword. This technique is mostly useful when passing a closure to a new thread to move the data so it's owned by the new thread.

- Because integers can be copied rather than moved, note that we use vectors to demonstrate.

```rs
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {x}");

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

Note that the value is moved at the closure definition, not when it is first used. By defining a `move` closure, the **closure takes ownership of the value**.
