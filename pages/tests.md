# Tests

Tests are Rust functions that verify that non-test code functions in an expected matter.

At its simplest, a test in Rust is a function that is annotated with the `test` attribute.

```rs
#[test]
fn check_fn() {}
```

Rust can compile any code examples that appear in API documentation and run them as tests.

A test will fail on panic. Rust provides several helper macros for assertions.

## Assertion macros

`assert!`

Asserts that its argument is true.

`assert_eq!`

Asserts that its two arguments are equal and prints the received arguments on fail.

`assert_ne!`

Asserts that its two arguments are not equal and prints the received arguments on fail.

Under the hood, `assert_eq!` and `assert_ne!` uses `==` and `!=`. When the assertions fail, these macros print their arguments using debug formatting. Therefore, the two arguments must implement the `PartialEq` and `Debug` traits. Because both traits are derivable, this is usually as simple as adding the `#[derive(PartialEq, Debug)]` annotation to the struct or enum definition.

### Custom format messages

The assertion macros take an optional argument for a format string.

```rs
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    )
}
```

## Checking for panics with `should_panic`

It's also important to check that our code handles error conditions as we expect. To do so, add the `#[should_panic]` annotation.

```rs
#[test]
#[should_panic]
fn panics_on_greater_than_100() {
    Guess::new(200);
}
```

To be more precise about the expected panic message, use the optional argument to the annotation. This will only pass if the **argument is a substring of the panic message**.

```rs
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn panics_on_greater_than_100() {
    Guess::new(200);
}
```

## Using `Result<T, E>` in tests

Instead of using the assertion macros, we can return a `Result<T, E>` from the test function. Return `Ok(())` when the test passes and `Err(message)` when it fails.

This lets us use the question mark operator in the test body which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.

You can't use the `#[should_panic]` annotation on tests that use `Result<T, E>`. Instead, you should return an `Err` value directly when the test should fail.
