# Advanced Types

## Newtype pattern

The newtype pattern is useful for more than implementing external traits on external types.

Newtypes can also be used to ensure values are never confused with each other. For example, making the guarantee that the `Millimeter(u32)` type and `Meter(u32)` types aren't confused.

Another use of the newtype pattern is abstracting away implementation details of the type. The new type can expose a public API that is different from the inner type.

## Type synonyms with type aliases

Rust also provides the ability to declare a _type alias_ to give an existing type another name.

```rs
type Kilometers = i32;
```

In this case, `Kilometers` is not a new type. Values that have the type `Kilometers` will be treated the same as values of type `i32`.

The main usecase for aliases is to reduce repetition.

```rs
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));
```

## Never type

Rust has a special `!` type that's known in type theory lingo as the `empty type`. It stands in place of the return type when a function will never return.

Functions that never return are called _diverging functions_.

Formally, expressions of type `!` can be coerced into any other type. For example, the `panic!` macro returns `!`, so its type is coerced into other types without affecting their resulting type.

```rs
let x = Some(5); // x is Some(i32);
let y = match x {
    Some(val) => val, // i32,
    None => panic!("no value"), // !
} // i32 | ! = i32
```

## Infallible

The `Infallible` type has the same role as the never type `!`. It is an enum with no variants, so a value of this type can never actually exist. This is useful for generic APIs that use a `Result<T, E>` but want to indicate that the result is always `Ok` by marking the type as `Result<T, Infallible>`.

## Dynamically sized types and the sized trait

Because Rust needs to know certain details, such as how much space to allocate for a certain type, there is the concept of _dynamically sized types_ (DSTs) or _unsized types_. These types let us write code using values whose size can only be known at runtime.

`str` (not `&str`) is an example of a DST, which is why we can't create a variable of type `str`.

Rust needs to know the size of a type at compile time, and all values of a type must use the same amount of memory. Therefore, although `&T` is a single value that stores the memory address of where `T` is located, if `T` is a DST, `&T` will store _two_ values: the address and its size.

We must combine DSTs will some kind of indirection like `Box<T>` or `Rc<T>`.

Trait objects are all DSTs by definition, which is why we have to wrap `dyn T` as `&dyn T`, `Box<dyn T>`, or `Rc<dyn T>`.

To work with DSTs, Rust has the `Sized` trait to determine whether a type's size is known at compile time. The `Sized` trait is automatically implemented for everything whose size is known at compile time. Rust also implicitly adds a bound on `Sized` to every generic function.

```rs
fn generic<T>(t: T) {
    // --snip--
}
```

is treated as:

```rs
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will only work on types that have a known size at compile time. However, we can use the following special syntax to relax this restriction:

```rs
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

which reads as "`T` may or may not be `Sized`". This syntax is only available for `Sized`. Also, because `T` may not be sized, we have to use some indirection.
