# Errors

## Unrecoverable errors

Sometimes there are errors that cannot or should not be recovered from. In these cases, we can use the `panic!` macro.

```rs
fn main() {
    panic!("Intentionally crash");
}
```

## Recoverable errors

The `Result` type is an enum with an `Ok(T)` and `Err(E)` variant.

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
          panic!("Problem opening the file: {:?}", error)
        },
    };
}
```

`Result` provides multiple helper methods to handle errors.

`unwrap` returns the value if successful and panics otherwise.

`expect` is the same as `panic` but allows us to specify a custom panic message.

`unwrap_or_else` accepts a closure which will be called if there is an error. The return value of the closure will be returned as the value of `unwrap_or_else`.

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

## Propagating errors

A common pattern is to propagate the error up from the current function.

```rs
use std::io::{self, Read, File};

// This function returns a Result of either a String or an IO error.
fn read_username_from_file() -> Result<String, io::Error> {
    // `File::open` returns a Result of either a file or an IO error.
    let f = File::open("hello.txt");

    // If there is an IO error, we'll propagate the error.
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    // `read_to_string` returns a Result of either a String or an IO error.
    // If the file read is successful, we return the string result. Otherwise,
    // we'll propagate the error.
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Error(error),
    }
}
```

## `?` shortcut for propagating errors

The `?` operator placed after a `Result` will propagate errors if they exists, or return the successful value. It can only be used in functions that return `Result` as it is defined to return an `Err(e)`.

```rs
use std::io::{self, Read, File};

fn read_username_from_file() -> Result<String, io:Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

It can also be chained.

```rs
use std::io::{self, Read, File};

fn read_username_from_file() -> Result<String, io:Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

Reading a file into a `String` is a common operation and Rust provides a library to do so.

```rs
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io:Error> {
    fs::read_to_string("hello.txt")
}
```
