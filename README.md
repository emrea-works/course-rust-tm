# Rust Course
## Source
Traversy Media, https://youtu.be/zF34dRivLOw

## Commands
- `rustc` is the compiler, works without cargo
- `rustup --version`
- `rustup update`
- `rustc --version`
- `rustc hello.rs` compiles hello.rs code.
- `./target./debug/rust_sandbox` runs compiled file as a default name.
### Cargo is the package manager**
- `cargo init` starts cargo project.
- `cargo run` compiles and run, creates 'target' folder.
- `cargo build` builds the folder (unoptimized).
- `cargo build --release` builds the folder (optimized).
### Syntax
- `println!()` prints to console
- `println("My place is hold by {}", "brackets")` prints
- `println!("{:?}" 1337, true, "text")` multiple types

## Notes
Semi columns are required in rust.

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



## TBC
https://youtu.be/zF34dRivLOw?t=1195
