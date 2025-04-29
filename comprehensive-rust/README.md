# Notes from `comprehensive-rust`

## Cargo
### Ecosystem
* `rustc` : rust compiler
* `cargo` : rust dependency manager and build tool, also can run tests
* `rustup` : rust toolchain installer and update manager

### Using cargo
* `cargo new <package-name>` : creates a new rust project
* `cargo run` : compiles and runs 
* `cargo build` : compiles
* `cargo build --release` : produce an optimized release build at `target/release/`
* `cargo check` : check the code for errors

## Types and Values
* Variables are immutable by default
* some built in types are : `i32`, `u32`, `f32`, `char`, `bool`
* Rust determines the type of a variable depending on usage if not stated explicitly

# RESUME FROM CONTROL FLOW BASICS