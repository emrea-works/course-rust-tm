# Rust Course
## Source
Traversy Media, https://youtu.be/zF34dRivLOw

## Commands
- `rustc` is the compiler, works without cargo
- `rustup --version`
- `rustup update` to see if there is updates
- `rustc --version`
- `rustc hello.rs` compiles hello.rs code.
- `./target./debug/rust_sandbox` runs compiled file as a default name.
### Cargo 
Cargo is the **package manager**
- `cargo init` starts cargo project.
- `cargo run` compiles and run, creates 'target' folder.
- `cargo build` builds the folder (unoptimized).
- `cargo build --release` builds the folder (optimized).
- cargo.toml is the package manifest file. 
### Syntax
- `println!()` prints to console
- `println("My place is hold by {}", "brackets")` prints in placeholder
- `println!("{:?}" 1337, true, "text")`, `"{:?}"` is called _debug trait_, prints multiple types
## Notes
Semi columns are required in rust.
main.rs is the entry point. 

Multiple types `println("{:?}")`.

### Variables
- Variables gold primitive data or reference to data.
- Variables are immutable by default. 
- Rust is a block-scoped language.
- const type of variables requires value type decleration.
- constant vars requires uppercase decleration.

### Types
**Primitive types**
  - Integers (Number of bits taken in memory)
    - unsigned: u8, u16, u32, u64, u128
    - integers: i8, i16, i32, i64, i128
  - Floats: f32, f64
  - Boolean (bool)
  - Characters (char) (not string, strings are different)
  - Tuples (lists)
  - Arrays (lengths are fixed)
- Single quotes are reserved only for char type and only can get unicode or a single string value (character)
- Single quoted char types supports emoji glyphs

**Strings**
There are two types of strings
- Primitive: Immutable, fixed-length string somewhere in memory
- String: Growable, heap-allocated data structure - Use when you need to modify or own string data.

## Tuples
- Tuples are group together values of different types
- Max 12 elements

## Arrays
- Arrays fixed list where elements are the same data types.
- Are stack allocated:

## Vectors 
- Resizable arrays

## Conditionals
- Same like C++ or JavaScript, only the statements are naked.

## Loops
- While conditions naked, just like in if conditionals
- For statement is typical for ... in ... type, without paranthesis, has a unique way of statement with dots
  - for _x in 0..10 means loop from 0 till to 10.

## Functions
- Closure functions are helping much when quick returns out of scopes

## Pointers
- Like in C++, points to a resource in memory directly.

## Structs
- ``impl`` works like classes in C++ or Java, has constructor, methods in its scope.

## Enums
- ``match`` word being used in the function is a typical switch conditional.

## TBC
https://youtu.be/zF34dRivLOw?t=3615
