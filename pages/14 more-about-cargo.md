# Release Profiles

In Rust, _release profiles_ are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.

Cargo has two main profiles: _dev_ when we run `cargo build` and _release_ when we run `cargo build --release`.

By adding `[profile.*]` sections in the project's _Cargo.toml_ file, we can override any subset of the default settings.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want faster compiling even if the resulting code runs slower.

# Publishing a crate to Crates.io

The crate registry at _https://crates.io_ distributes the source code of your packages, so it primarily hosts code that is open source.

## Documentation comments

Accurately documenting your packages will help other users know how and when to use them, so it's worth investing the time to write documentation.

Rust supports _documentation comments_, that will generate HTML documentation. The HTML displys the contents of documentation comments for public API items intended for programmers interested in knowing how to _use_ your crate.

Documentation comments use three slashes `///` instead of two and support Markdown notation for formatting. Place documentation comemnts just before the item they are documenting.

````rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x = 1
}
````

We can generate the HTML documentation from this documentation comment by running `cargo doc`. This command runs the `rustdoc` tool distributed with Rust and puts the generated HTML in `target/doc`.

For convenience, running `cargo doc --open` will build the HTML for the current crate's documentation, then open the result in a web browser.

### Commonly used sections

- **Panics** The scenarios in which the function being documented could panic.
- **Errors** If the function returns a `Result`, describing the kinds of errors that might occur and what conditions might cause those errors can be helpful to callers.
- **Safety** If the function is `unsafe` to call, they should be a section explaining why the function is unsafe and covering the invariants that the function expects the caller to uphold.

Most documentation comments don't need all of those sections, but it is a good checklist to remind us of all the aspects of our code that callers might be interested in knowing about.

### Documentation comments as tests

Adding example code blocks to documentation comments can help demonstrate how to use your library, and doing so also provides automatic generation of tests.

Running `cargo test` will run code examples in documentation as tests. This helps prevent outdated documented code examples.

### Commenting contained items

`//!` adds documentation to the item that contains the comments rather than adding documentation to the items following the comments. These are normally used inside the create root (`src/lib.rs`) or inside a module to document the crate or module as a whole.

```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

### Exporting a convenient public API with `pub use`

If we want to expose a convenient public API that is different from our internal code hierarchy, we can re-export items.

Instead of having users use `art::kinds::PrimaryColor` for example, we could re-export the type as `art::PrimaryColor`.

```rs
//! # Art
//!
//! A library for modelling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  // --snip--
}

pub mod utils {
  // --snip--
}
```
