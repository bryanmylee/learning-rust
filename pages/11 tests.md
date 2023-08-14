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

## Showing function output

If a test passes, Rust's test library captures anything printed to standard output. To show function output, add `--nocapture`.

```bash
cargo test -- --nocapture
```

## Running a subset of tests by name

Pass the name of any test function to run only that test.

```bash
cargo test one_hundred
```

Rust will use prefix matching to filter tests. Because the module in which tests appear become part of the tests' names, we can run all tests in a module by filtering on the module name.

## Ignoring some tests

Adding `#[ignore]` after `#[test]` excludes the test from regular tests. If we want to only run the ignored tests, we can use `cargo test -- --ignored`.

# Test organization

The Rust community distinguishes between _unit tests_ and _integration tests_.

## Unit tests

Unit tests test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected.

Unit tests are placed in the `src` directory in each file with the code that they are testing.

The convention is to create a module named `tests` to contain the test functions and to annotate the module with `#[cfg(test)]`.

The annotation tells Rust to compile and run the test code only when using `cargo test`.

### Testing private functions

Rust's privacy rules allow testing of private functions because tests are simply defined in a child module.

```rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration tests

Integration tests only call functions that are part of the library's public API. Their purpose is to test whether many parts of the library work together as expected.

Integration tests are placed in the `tests` directory at the top level of the project directory next to `src`.

Each integration test in `tests` is a separate crate so we need to bring our library into each test crate's scope.

```rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

To run a specific integration test file, use the `--test` argument.

```bash
cargo test --test integration_test
```

This command runs only the tests in `tests/integration_test.rs`.

### Submodules in integration tests

As we add more integration tests, we may split integration tests into submodules.

We may want to extract common functions like setup and cleanup helpers into separate files. To exclude these files from the test runner, define them in files in subdirectories of `tests`. Files in subdirectories of the `tests` directory do not get compiled as separate crates or have sections in the test output.

`mod.rs` will be treated as the index module for the directory it is in.

```rs
// tests/common/mod.rs
pub fn setup() {
  // --snip--
}

// tests/integration_test.rs
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

### Integration tests for binary crates

Binary crates do not expose functions that other crates can use and therefore cannot be directly tested with integration tests.

This is one of the reasons Rust projects that provide a binary have a straightforward `src/main.rs` file that calls logic that lives in the `src/lib.rs` file. The library crate can be tested with integration tests. If the important functionality works, then the small amount of code in the `src/main.rs` file will work as well.
