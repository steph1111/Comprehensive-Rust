<!-- PROJECT INTRO -->
<h1 align="center">Comprehensive Rust 🦀</h1>

<h3 align="center"> Three Day Rust Course </h3>

<p align="center">
  <br />
  <a href="https://google.github.io/comprehensive-rust/">Course</a>
  ·
  <a href="https://doc.rust-lang.org/std/index.html">Documentation</a>
  ·
  <a href="https://www.rust-lang.org/tools/install">Install</a>
</p>
</div>

<br>

<!-- ABOUT THE PROJECT -->

---
## About Rust
Rust is modern programming language developed in 2010. It is often compared to C or C++ but does not have the dangers of memory misuse. Rust is designed for speed, efficient and safe memory management, and the ability to run multiple programs in parallel.

Rust documentation may be found on the official [website](https://doc.rust-lang.org/std/index.html) or by running the following command in a terminal:
```sh
rustup doc
```

<br>

<!-- GETTING STARTED -->

---
## Getting Started
<br>

### Installing Rust
1. Run the following command in a terminal.
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Restart the terminal environment.
3. Ensure rust installed by entering the following command.
   ```sh
   rustc --version
   ```
4. Additional installation information may be found [here](https://doc.rust-lang.org/book/ch01-01-installation.html).

<br>

### Creating a program
1. Use `touch` to create a file that ends in the file extension `.rs`
   ```sh
   touch hello_world.rs
   ```
2. Compile the code with `rustc`
   ```sh
   rustc hello_world.rs
   ```
3. Run the executable (it will have the same name as the .rs file)
   ```sh
   ./hello_world
   ```
<br>

<!-- USAGE -->

---
<!-- TABLE OF CONTENTS -->
<details>
  <summary>Quick Reference</summary>
  <ol>
    <li><a href="#hello_world">sig_float</a>
  </ol>
</details>
<br>

---
## Hello World!
```rs
fn main() {
    println!("Hello 🌍!");
}
```
- Functions start with `fn`
- Blocks have curly braces {}
- Like C++, rust programs have a `main()` function

<br>

---
## Basic Syntax
<br>

### Scalar Types
|     |Types|Literals|
| :-- | :-- | :-- |
| Signed integers | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123i64` |
| Unsigned integers | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10u16`| 
| Floating point | `f32`, `f64` | `3.14`, `-10.0e20`, `2f32` | 
| Strings | `&str` | `"foo"`, `"two\nlines"` |
| Unicode scalar values | `char` |	`'a'`, `'α'`, `'∞'`
| Boolean | `bool`  | `true`, `false`

<br> 

**Notes:**

- iN, uN, and fN are N bits wide,
- isize and usize are the width of a pointer

<br>

**Embedded double quotes:** # on each side or \"

```rs 
println!(r#"<a href="link.html">link</a>"#);
// <a href="link.html">link</a>
```
**Byte strings:** `&[u8]` value directly
```rs
println!("{:?}", b"abc"); // [97, 98, 99]
```

<br> 

### Compound Types
|     |Types|Literals|
| :-- | :-- | :-- |
| Arrays | `[T; N]` | `[20, 30, 40]`, [`0; 3]` | 
| Tuples | `()`, `(T,)`, `(T1, T2)`, … | `()`, `('x',)`, `('x', 1.2)`, … | 

<br>

**Arrays**
- Arrays have the same type `T` and length `N` which at compile time is constant
- The length of an array is part of its type (`[u8; 3]` and `[u8; 4]` are different types)
- Index 5 of array `a` can be accessed by `a[5]`
- Arrays are printable with formatting specifiers: 
  ```rs
  println!("a: {:?}", a);
  // {:?} gives debug output
  // {:#?} gives formatted output
  ```
  


**Tuples**
- Tuples also have a fixed length 
- Group together elements of *different* types
- Index 5 of a tuple `t` can be accessed by `t.5`
- An empty tuple, `()`, is a “unit type”, it is like void

<br>

## References