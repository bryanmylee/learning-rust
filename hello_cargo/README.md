# Cargo

Cargo is Rust's built-in package manager.

`cargo new {project_name}`

Creates a new project with the default project template.

`cargo build`

Compiles the source code found in `src` into `target`.

By default, this compiles an executable for debugging under `target/debug`. Add `--release` to compile a binary with optimizations to `target/release`.

`cargo run`

Convenience to run the compiled binary in `target`.

`cargo check`

Lint the source code. This process is much faster than building the entire program with `cargo build`.
