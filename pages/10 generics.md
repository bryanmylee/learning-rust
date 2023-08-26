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
