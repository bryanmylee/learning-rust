# Generics

Generics allow code to operate on abstract types.

### Functions

When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.

```rs
fn largest<T>(list: &[T]) -> T {
    // --snip--
}
```

### Structs

We can define structs to use a generic type parameter in one or more fields.

```rs
struct Point<T> {
    x: T,
    y: T,
}
```

### Enums

We can define enums with generic type parameters.

```rs
enum Option<T> {
    Some(T),
    None,
}
```

### Methods

We can implement methods on structs and enums and use generic types in their definitions too.

```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

Note that we have to declare `T` just after `impl` so we can use it to specify that we are implementing generic methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.

For example, we could implement methods only on `Point<f32>` instances rather on a generic `Point<T>`.

Generic type parameters in a struct definition are not always the same as those used in the struct's method signatures.

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        // --snip--
    }
}
```

## Performance of generics

Rust implements generics in a way that does not incur any performance costs. Rust performs monomorphization of code that uses generics **at compile time**.

_Monomorphization_ is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. It looks at all places where generic code is called and generates code for the concrete types the code is called with.

```rs
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}

// compiled to:
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

# Traits

A _trait_ tells the Rust compiler about functionality that a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use **trait bounds** to specify that a generic can be any type with a certain behavior.

## Defining traits

A type's behavior consists of methods we can call on that type. Different types share the same behavior if we can call the same methods on all those types.

To define shared behaviors, we declare a trait using the `trait` keyword and provide method signatures that describe the behaviors of the types that **implement this trait**.

```rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing a trait

After defining the desired behavior using the trait, we can implement it on the type with `impl {trait} for {type}`. This is similar to implementing regular methods with the added `{trait} for` portion.

```rs
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // --snip--
    }
}
```

To implement a trait defined in another crate, we would first need to bring the trait into our scope first. For example, if we want to implement a `Summary` trait from the `aggregator` crate, we would specify `use aggregator::Summary;`.

Another restriction is that we can implement a trait on a type only if **either the trait or the type is local to our crate**. For example, we can implement standard traits like `Display` on our custom types in our crate, and we can impleemnt our custom traits on standard types like `Vec<T>`, but we cannot implement `Display` on `Vec<T>` within our crate because both the trait and type are defined in the standard library and are not local to our crate.

This restriction is part of a property of programs called _coherence_, more specifically the _orphan rule_. This rule ensures that other people's code can't break your code and vice versa. Without the rule, two crates could implement the same trait for the same type and Rust would not know which implementation to use.

## Default implementations

Sometimes it is useful to have default behaviors for some or all of the methods in a trait. Then as we implement the traits on a particular type, we can keep or override each method's default behavior.

```rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

To use a default implementation instead of defining a custom implementation, we omit the implementation from the `impl {trait} for {type}` block.

```rs
impl Summary for NewArticle {} // Summary only has the one method so this block is empty.
```

Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation. This way, traits can provide a lot of useful functionality and only require implementators to specify a small part of it.

```rs
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

## Traits as parameters

We can now explore how to use traits to define functions that accept many different types. For example, we can define a function that calls the `summarize` method on its `item` parameter which is **of some type that implements the `Summary` trait**. To do this, we use `impl {trait}` as the type.

```rs
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

This is shorthand for a longer syntax called a _trait bound_.

```rs
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

We specify trait bounds by adding `: {trait}` to a generic argument.

Trait bounds are useful if we need more specific generic behavior e.g. ensuring that two arguments to a function are of the same type, both of which are bound to a `Summary` trait.

```rs
pub fn notify(item1: impl Summary, item2: impl Summary) {} // item1 and item2 could be different types.
pub fn notify<T: Summary>(item1: T, item2: T) {} // item1 and item2 must be the same type.
```

## Multiple trait bounds

We can also specify more than one trait bound to add more specificity to the generic argument passed by using the `+` syntax.

```rs
pub fn notify(item: impl Summary + Display) {}
pub fn notify<T: Summary + Display>(item: T) {}
```

## `where` clauses

Functions with multiple generic type parameters can contain a lot of trait bound information between the function's name and its parameter list, making the signature hard to read. For this reason, Rust has an alternate syntax for specifying trait bounds inside a where clause.

```rs
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}
```

## Returning types that implement traits

We can also use `impl {trait}` in the return position to return a value of some type that implements a trait. This makes the concrete type opaque to the caller.

This is especially useful when using closures and iterators.

One major restriction is that the **concrete return type must be a single type even if the type is opaque**. This means we cannot use `impl {trait}` to hide that a function returns multiple concrete types. This is due to restrictions around how `impl {trait}` is implemented in the compiler.

## Using trait bounds to conditionally implement methods

By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

```rs
use std::fmt::Display;

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // this method only exists for Pair<T> where T implements `Display` and `PartialOrd`.
    fn cmp_display(&self) {
        // --snip--
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called _blanket implementations_. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. This provides the method `to_string` to any type that implements `Display`.

```rs
impl<T: Display> ToString for T {
    // --snip--
}
```

More specific implementations can still be provided to override the blanket implementation.
