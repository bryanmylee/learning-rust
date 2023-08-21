# Closures

Closures are function-like constructs that can be stored in variables. Unlike functions, **closures can capture values from the scope in which they are defined**.

```rs
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num + random_number
    };

    expensive_closure(intensity)
}
```

If the body of the closure only contains one expression, the curly braces are unnecessary.

```rs
fn generate_workout(intensity: u32, random_number: u32) {
    let increment = |value| value + 1;
}
```

## Closure type inference and annotation

Functions require type annotations because they are part of an explicit contract exposed to users. Defining this contract rigidly is important for ensuring the types and values a function uses and returns.

However, closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the conpiler is reliably able to infer the types of the parameters and the return type. Therefore in most cases, closures do not need to be type annotated.

Of course, we can still be explicit about the types if we want.

```rs
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num + random_number
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
