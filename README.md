# challenge-rust

build all: 

``cargo tarpaulin --ignore-tests --out Lcov --all --output-dir target\tarpaulin``
``cargo build --all --release``
> Release build also creates PDB files and lets you debug the optimized version as well!



## Chat GOT cargo cheat sheet

Creating a new project
To create a new Rust project with Cargo, use the cargo new command:


cargo new my_project
This will create a new directory called my_project with the following directory structure:

```
my_project
├── Cargo.toml
└── src
    └── main.rs
```
The Cargo.toml file is the project's manifest file, which contains metadata about the project and its dependencies. The src directory contains the source code for the project, and the main.rs file is the entry point for the project.

Building and running a project
To build a Rust project with Cargo, use the cargo build command:

```
cargo build
```

This will compile the project and create an executable file in the target/debug directory.

To run the project, use the cargo run command:

```
cargo run
```

This will build and run the project in one step.

Adding dependencies
To add a dependency to a Rust project, add it to the dependencies section of the Cargo.toml file. For example:

```
[dependencies]
reqwest = "0.10"
```
This will add the reqwest crate as a dependency. You can then use the crate in your code by adding the following line at the top of your source file:

```
extern crate reqwest;
```
Publishing a crate
To publish a Rust crate to crates.io, use the cargo publish command:

```
cargo publish
```
This will build and publish the crate to crates.io. You will need to have an account on crates.io and be logged in with cargo login before you can publish a crate.

Testing a project
To run the tests for a Rust project, use the cargo test command:

```
cargo test
```
This will run all the tests in the project and print the results to the console.

Updating dependencies
To update the dependencies for a Rust project, use the cargo update command:

```
cargo update
```
This will update all the dependencies for the project to the latest version.

Cleaning a project
To delete the compiled files for a Rust project, use the cargo clean command:

```
cargo clean
```
This will delete all the compiled files in the target directory, allowing you to start a fresh build.

## Chat GPT Rust cheat sheet
Variables and types
```
let x = 5; // x is a variable with the value 5
let y: i32 = 10; // y is a 32-bit integer with the value 10

let s = "Hello, world!"; // s is a string slice
let s = String::from("Hello, world!"); // s is a owned String

let t = true; // t is a boolean value
let f: bool = false; // f is a boolean value
```
# Functions
```
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let result = add(5, 10); // result is 15

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

greet("Alice"); // prints "Hello, Alice!"
```

# Control structures
```
let x = 5;

if x < 10 {
    println!("x is small");
} else {
    println!("x is not small");
}

let mut i = 0;

while i < 10 {
    println!("{}", i);
    i += 1;
}

for j in 0..10 {
    println!("{}", j);
}
```

# Arrays and slices
```
let a = [1, 2, 3, 4, 5]; // a is an array of 5 integers
let b = &a[1..3]; // b is a slice of a with elements [2, 3]

let c = [0; 10]; // c is an array of 10 zeros

let d = [1, 2, 3];
let e = [4, 5, 6];
let f = [d, e]; // f is a two-dimensional array [[1, 2, 3], [4, 5, 6]]
```
# Tuples
```
let t = (1, "hello"); // t is a tuple with elements (1, "hello")

let (x, y) = t; // x is 1, y is "hello"

let x = t.0; // x is 1
let y = t.1; // y is "hello"
```
# Structs
```
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 5, y: 10 };

println!("x = {}", p.x); // x = 5
println!("y = {}", p.y); // y = 10
```
# Enums
```
enum Color {
    Red,
    Green,
    Blue,
}

let c = Color::Red;

match c {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    Color::Blue => println!("Blue"),
}

enum Option<T> {
    Some(T),
    None,
}

let x: Option<i32> = Some(5);

match x {
    Some(n) => println!("Some({})", n),
    None => println!("None"),
```
