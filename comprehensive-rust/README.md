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

## References

* A shared reference is a way to access a value without taking ownership
* A shared reference to a type `T` has type `&T`. A reference value is made with the & operator. The `*` operator “dereferences” a reference, yielding its value.
* references can never be null in rust
* **Exclisive references** allow for changing the referenced value
* A slice gives a view of a larger collection
```rust
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4]; // slice from 30 to 50
    let x: &[i32] = &a[..]; // full copy

    println!("s: {s:?}");
}
```
* There are 2 types of strings in rust, `&str` is a slice od UTF-8 encoded bytes and `String` is an owned buffer of UTF-8 encoded bytes.
* References can never be null and can never outlive the data they point to

## User defined data structures
* **Named Structs** are like in C or C++ without `typedef` or inheritance
```rust
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);
}
```
* **Tuple Structs** are used when name of the parameters are not important
* **Enums** are used to create a type which has a few different variants, like a `sealed interface` in kotlin
```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");
}
```
* **typealiases** are used to reduce long and complex types
* **const** are evaluated at compile time and their values are inlined wherever they are used
* **static** variables are not inlined and are stored in dedicated locations

## Pattern Matching
* Irrefutable patterns
```rust
fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;

    // Ignore everything but the last element.
    let (.., c) = tuple;
}

fn main() {
    takes_tuple(('a', 777, true));
}
```
* similarly, enums and structs can also be destructured
* Rust has some unique let control flow statements: `if let`, `while let`, `let else`
* Rust allows to define methods for specific types. like extension functions in kotlin using an `impl` block
```rust
#[derive(Debug)]
struct CarRace {
    name: String,
    laps: Vec<i32>,
}

impl CarRace {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self (covered later)
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = CarRace::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
```
* types can be abstracted over with traits. like interfaces in kotlin
```rust
trait Pet {
    fn talk(&self) -> String; // abstract implementation

    fn greet(&self) { // default implementation
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String { // implementation
        format!("Woof, my name is {}!", self.name)
    }
}

fn main() {
    let fido = Dog { name: String::from("Fido"), age: 5 };
    fido.greet();
}
```

# Resume from Day 2 > Generics