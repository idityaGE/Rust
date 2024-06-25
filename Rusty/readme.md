## Rusty

### Installation of Rust

1. Windows
   - Download the installer from the [official website](https://www.rust-lang.org/tools/install)
   - Run the installer
   - Open a new terminal and run `rustc --version` to check if the installation was successful
2. Linux
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. MacOS
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

### Tools

#### Rustup

Rustup is the recommended tool to install Rust. It manages Rust versions and associated tools.

#### Rustc

Rustc is the Rust compiler. It is used to compile Rust programs.

#### Cargo

Cargo is the Rust package manager. It is used to compile, test, and run Rust programs. It also manages dependencies.

### setup

1. Create a new project
   ```bash
   cargo new project_name
   ```
2. Change directory to the project
   ```bash
   cd project_name
   ```
3. Build the project
   ```bash
   cargo build
   ```
4. Run the project
   ```bash
   cargo run
   ```
5. Test the project
   ```bash
   cargo test
   ```
6. Check the project
   ```bash
   cargo check
   ```

### References

- [Rust Programming Language](https://www.rust-lang.org/)
- [Rustup](https://rustup.rs/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Folder Structure

- `src`: Contains the source code of the project
- `target`: Contains the compiled code of the project
  - `debug`: Contains the debug build
  - `release`: Contains the release build
- `Cargo.toml`: Contains metadata about the project, like `package.json` in Node.js
- `Cargo.lock`: Contains information about the dependencies, like `package-lock.json` in Node.js

### Hello World

```rust
fn main() {  // main is the entry point of the program
    println!("Hello, world!");
}
```

### Data Types

1. Integer
   - `i8`, `i16`, `i32`, `i64`, `i128`, `isize` // Signed
   - `u8`, `u16`, `u32`, `u64`, `u128`, `usize` // Unsigned
2. Float
   - `f32`, `f64`
3. Boolean
   - `bool`
4. Character
   - `char`
5. String
   - `&str`, `String`
6. Tuple
   - `(T1, T2, T3, ..., Tn)`
7. Array
   - `[T; n]`
8. Slice
   - `&[T]`
9. Struct
   - `struct Name { field1: T1, field2: T2, ..., fieldn: Tn }`
10. Enum
    - `enum Name { Variant1, Variant2, ..., Variantn }`
11. Option
    - `Option<T>`
12. Result
    - `Result<T, E>`
13. Function
    - `fn name(arg1: T1, arg2: T2, ..., argn: Tn) -> T { ... }`

#### Example

```rust
fn main() {
    let x: i32 = 10;  // Integer
    let y: f64 = 10.5;  // Float
    let z: bool = true;  // Boolean
    let c: char = 'A';   // Character
    let s: &str = "Hello, world!";  // String
    let t: (i32, f64, bool) = (10, 10.5, true);  // Tuple
    let a: [i32; 3] = [10, 20, 30];  // Array
    let sl: &[i32] = &a;  // Slice
    let st: Name = Name { field1: 10, field2: 10.5, field3: true };  // Struct
    let e: Option<i32> = Some(10);  // Enum
    let r: Result<i32, &str> = Ok(10);  // Result
    let f: fn(i32, f64, bool) -> i32 = name;  // Function
}
```

```rust
fn main() {
  let num: u8 = 255;
  // Here u8 means unsigned--> only +ve; 8-bit --> 0 to 255 :(2^n - 1)
  // If we try to assign 256, it will throw an error
  println!("The number is: {}", num);
}
```

#### Mutable and Immutable Variables

Mutable variables can be changed, whereas immutable variables cannot be changed.

```rust
fn main() {
    let x = 10;  // Immutable variable
    // x = 20;  // Error
    let mut y = 20;  // Mutable variable
    y = 30;
    println!("x: {}, y: {}", x, y);
}
```
