# Unsafe Rust

Unsafe rust allows us to opt out of some of Rust's guarantees and take responsibility for manually upholding those guarantees. In exchange, we gain the ability to:

- dereference raw pointers
- call other unsafe functions or methods
- access or modify a mutable static variable
- implement unsafe traits

## Raw pointers

In some situations, we need to use raw pointers. However, pointers do not always point to valid memory. Therefore, we have to make these guarantees ourselves.

`split_at_mut` is a common example where unsafe Rust is required for pointers.

```rs
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    (&mut slice[..mid], &mut slice[mid..])
}
```

Although we know that the two slices will not overlap and therefore will be valid mutable borrows, Rust has no way to make that guarantee, and will throw an error about double mutable borrowing.

To fix this, we have to use `unsafe`. Note that it is perfectly valid to wrap `unsafe` code within a safe function. In fact, this is a common pattern to allow encapsulation of unsafe behavior to small scopes where safety guarantees can be more easily made.

```rs
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}
```

## Using `extern` functions to call external code

Rust allows interaction with other languages with the `extern` keyword, which facilitates the creation and use of a _Foreign Function Interface_. However, Rust cannot guarantee the memory safety of external functions, so these have to be called in `unsafe`.

```rs
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Side note, we can also expose Rust functions to other languages with `extern`. Just make sure that we disable name mangling on the function so that other languages can name the function properly.

```rs
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called from C!");
}
```

## Accessing and modifying a mutable static variable

We've discussed global constants before, but Rust also supports global variables, named _static variables_ in Rust. Unlike constants which can be freely duplicated wherever they are used, static variables have a fixed address in memory and will always point to the same data.

The naming convention for static variables are in `SCREAMING_SNAKE_CASE`. Static variables can only store references with the `'static` lifetime.

Static variables can be mutable, but accessing and modifying mutable static variables is _unsafe_ because it is difficult to ensure that there are no **data races**.

## Unsafe traits

A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify. Some examples of this are `Sync` and `Send`, which cannot be verified by the Rust compiler.

```rs
unsafe trait Foo {
    // method declarations
}

unsafe impl Foo for i32 {
    // method implementations
}
```
