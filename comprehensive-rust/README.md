# Notes from `comprehensive-rust`

From the perspective of a fresher who only knows kotlin

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

## Control Flow Basics
* Blocks are enclosed by braces `{}`, each block has a value of the last expression and a type
* `match` is like the `when` expression in kotlin.
```rust
fn main() {
    let val = 1;
    match val { // evaluated from top to bottom
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => { // like else
            println!("something else");
        }
    }
}
```
* `loop` statement loops forever until a break
* rest are same as other languages `if`, `for`, `while`, `break`, `continue`
* loops can be labelled like this
```rust
fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 { // labelled the loop as "outer"
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer; // break the labelled loop
            }
        }
    }
    dbg!(elements_searched); // dirty debug
}
```
* Macros are expanded into rust code during compilation: `println!()`, `format!()`, `dbg!()`, `todo!()` are some useful macris from the standard lib

## Tuples and Arrays
* Arrays can be initialised like this
```rust
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1]; // [Type: No of elements]
    a[2] = 0;
}
```
* Tuples are like `Pairs<A, B>` in kotlin
```rust
fn main() {
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}
```
* `for` supports interating over arrays but not tuples
* Pattern matching works the same as kotlin
* Arrays can contain other arrays

## Shared References (Borrowing)

* A way to access a value without taking ownership
# CONTINUE FROM DAY 1 Afternoon > Shared references