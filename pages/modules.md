# Modules

Modules let us organize code within a crate into groups for readability and easy reuse.

When using `cargo new`, Cargo creates a **binary package**. It follows a convention that `src/main.rs` is the crate root for a binary package. If the package contains a `src/lib.rs`, then Cargo knows that it contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.

We can create library packages by running `cargo new --lib {package_name}`.

## Scope and privacy

Modules allow us to control the privacy of items -- whether an item can be used by outside code (public) or is an internal implementation detail (private).

We use the `mod` keyword to define a new module which will contain other items such as structs, enums, constants, traits, or functions.

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

## Paths

Modules create a tree-like structure which we can traverse with paths. If we want to call a function, we need to know its path. A path can take two forms:

- An _absolute path_ starts from the crate root by using a crate name or a literal `crate`.
- A _relative path_ starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by `::`.

Assume we want to call `add_to_waitlist` from a new function `eat_at_restaurant` defined in the crate root. We will expose the `eat_at_restaurant` function as part of our library crate's public API.

To mark an item as public, use the `pub` keyword.

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

During compilation, we'll receive an error informing us that the `hosting` module is private. Modules define a **privacy boundary** which encapsulate the implementation details from external code.

All items (functions, methods, structs, enums, modules, and constants) are private by default. **Items in a parent module cannot use private items in a child module, but items in a child module can use items in their ancestor module**.

To expose paths, we use the `pub` keyword again.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Note that `eat_at_restaurant` can access `front_of_house` because they are defined in the same module (root module).

### `super`

We can also construct relative paths that begin in the parent module by using `super` at the start of the path.

```rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

## Public structs an enums

We can use `pub` to designate structs and enums as public.

### Public structs

If we use `pub` before a struct definition, we make the struct public but the struct's fields will still be private. Private fields can be accessed by sibling and child scopes but not parent scopes.

This usually means that the struct must provide a public associated function to construct new instances because public functions will not be able to access the private fields and therefore cannot construct new instances.

```rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String.from("Wheat");
    meal.seasonal_fruit = String::from("blueberries"); // access error.
}
```

### Public enums

By contrast, all variants of a public enum are public.

```rs
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

# `use`

Instead of specifying the full path of functions in modules every time, we can bring a path into a scope once and then call the items in that path as if they are local items with the `use` keyword.

The final path segment in the path with `use` will be brought into scope as a valid name. Paths brought into scope with `use` will also check privacy like any other paths.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Specifying a relative path with `use` requires `self` at the start of the path.

```rs
use self::front_of_house::hosting;
```

Instead of specifying the path all the way down to the function, it is more idiomatic to specify up to the parent module. **This makes it clear that the function isn't locally defined** while still minimizing repetition of the full path.

## `as`

`as` allows us to rename imported modules. One reason is to prevent naming conflicts.

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}
```

## Re-exporting names

We can combine `pub` and `use` to re-export modules. This lets us write code with one structure and expose a different structure.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

## Nested paths

If we have a large list of paths to import that share a common prefix, we can use nested paths to clean up the import.

```rs
use std::io;
use std::cmp::Ordering;
// is equivalent to
use std::{io, cmp::Ordering};

use std::io;
use std::io::Write;
// is equivalent to
use std::io::{self, Write};
```

## Glob operator

If we want to bring all public items defined in a path into scope, we can specify that path with `*`.

```rs
use std::collections::*;
```

# Separating modules into files

Modules can be separated into files with the same name as the module. Then, the module can be imported with `mod {package_name};`. When used without a block, `mod` takes a relative filepath and loads the contents of the file at the specified path.

```rs
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}

// src/lib.rs
mod front_of_house; // load the contents of the module from the file `./front_of_house.rs`.

pub use crate::front_of_house::hosting; // bring the hosting module into scope.
```

Nested modules can also be separated further.

```rs
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}

// src/front_of_house.rs
pub mod hosting;

// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;
```

Using `pub mod` re-exports the loaded module.
