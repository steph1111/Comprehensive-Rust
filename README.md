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

---
## About Rust
Rust is modern programming language developed in 2010. It is often compared to C or C++ but does not have the dangers of memory misuse. Rust is designed for speed, efficient and safe memory management, and the ability to run multiple programs in parallel.

Rust documentation may be found on the official [website](https://doc.rust-lang.org/std/index.html) or by running the following command in a terminal:
```sh
rustup doc
```

<br>

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
  <summary>Notes table of contents</summary>
  <ol>
    <li><a href="#exercise-hello-world">Exercise: Hello World!</a>
    <li><a href="#scalar-types">Scalar Types</a></li>
    <li><a href="#compound-types">Compound Types</a></li>
    <li><a href="#references">References</a></li>
    <li><a href="#slices">Slices</a></li>
    <li><a href="#types-of-string">Types of Strings</a></li>
    <li><a href="#functions">Functions</a></li>
    <li><a href="#rustdoc">Rustdoc</a></li>
    <li><a href="#methods">Methods</a></li>
    <li><a href="#function-overloading">Function Overloading</a></li>
    <li><a href="#implicit-conversions">Implicit Conversions</a></li>
    <li><a href="#arrays-and-for-loops">Arrays and for Loops</a></li>
    <li><a href="#variable-type-inference">Variable Type Inference</a></li>
    <li><a href="#static-and-constant-variables">Static and Constant Variables</a></li>
    <li><a href="#scopes-and-shadowing">Scopes and Shadowing</a></li>
    <li><a href="#stack-vs-heap">Stack vs Heap</a></li>
    <li><a href="#round_sig">FIXME</a></li>
  </ol>
</details>
<br>

---
## Exercise: Hello World!
[hello_world.rs](https://github.com/steph1111/Comprehensive-Rust/blob/master/hello_world.rs)
```rs
fn main() {
    println!("Hello 🌍!");
}
```
- Functions start with `fn`
- Blocks have curly braces `{}`
- Lines end in `;`
- Like C++, rust programs have a `main()` function

<br>

---
## Scalar Types
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

**Embedded double quotes:** # on each side or `\"`

```rs 
println!(r#"<a href="link.html">link</a>"#);
// <a href="link.html">link</a>
```
**Byte strings:** `&[u8]` value directly
```rs
println!("{:?}", b"abc"); // [97, 98, 99]
```

<br> 

---
## Compound Types
|     |Types|Literals|
| :-- | :-- | :-- |
| Arrays | `[T; N]` | `[20, 30, 40]`, [`0; 3]` | 
| Tuples | `()`, `(T,)`, `(T1, T2)`, … | `()`, `('x',)`, `('x', 1.2)`, … | 

<br>

### Arrays
```rs
// Mutable array named 'a' of type i8 
// with 10 elements with the value 42 
let mut a: [i8; 10] = [42; 10];
```
- Arrays have the same type `T` and length `N` which at compile time is constant
- The length of an array is part of its type (`[u8; 3]` and `[u8; 4]` are different types)
- Index 5 of array `a` can be accessed by `a[5]`
- Arrays are printable with formatting specifiers: 
  ```rs
  println!("a: {:?}", a);
  // {:?} gives debug output
  // {:#?} gives formatted output
  ```

### Tuples
```rs
// Tuple with t.0 being '7' and t.1 'true'
let t: (i8, bool) = (7, true);
```
- Tuples also have a fixed length 
- Group together elements of *different* types
- Index 5 of a tuple `t` can be accessed by `t.5`
- An empty tuple, `()`, is a “unit type”, it is like void

<br>

---
## References
- References exist in rust
- A reference must be deferenced before assigning to it 
```rs
fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}"); // Prints 20
}
```

<br>

---
## Slices
- Slices allow one to take a subset of a collection of data 
- Slices are created by defining starting and ending indices
- If the start index is 0, you may drop the start index. Same with the last index

```rs
fn main() {
    let mut arr: [i32; 6] = [10, 20, 30, 40, 50, 60]; // array of type i32 with 6 elements
    println!("arr: {arr:?}"); // Prints with debug formatting
    let slice: &[i32] = &arr[2..4]; // Slices arr from index 2 to 4
    println!("arr slice: {slice:?}") // Prints slice with debug format
}
```

```rs
fn main() {
    let mut arr: [i32; 6] = [10, 20, 30, 40, 50, 60]; // array of type i32 with 6 elements
    println!("arr: {arr:?}"); // Prints with debug formatting
    let slice: &[i32] = &arr[2..]; // Slices arr from index 2 to end
    println!("arr slice: {slice:?}") // Prints slice with debug format
}
```

<br>

---
## Types of String 
- In rust there are two types of string: `&str` and `string`
- `&str` is a immutable string reference. Like `const char*` in C++
- `string` is a mutable string buffer. Like `std::string` in C++ 

<br>

---
## Functions
- The main() function is always the start of the program 
- Functions are started with `fn` then the function name 
- The parameters are formatted as name then type 
- The last expression in a function body (or any block) becomes the return value. Simply omit the ; at the end of the expression.
- The return type comes after `->`. For void use `-> ()`
```rs
// Function name: is_divisible
// First param: u32 named n 
// Second param: u32 named divisor
// Return type: bool
fn is_divisible(n: u32, divisor: u32) -> bool {
   if divisor == 0 {
      return false; // Explicit return 
   }
   n % divisor == 0 // Last line, implied return because no ;
}
```

<br>

---
## Rustdoc 
- Rust code may be documented with `///`
- `///` allows for Markdown formatting
- All published Rust library crates are automatically documented at docs.rs using the rustdoc tool. 

<br>

---
## Methods
- Like in python, methods are defined with `&self` (or `mut &self`) as the first parameter 
```rs
fn area(&self) -> u32 {
   self.width * self.height
}
```

<br> 

---
## Function Overloading
- Function overloading is not supported in Rust
- Default parameters are also not supported
- Template types however are available 


<br>

---
## Implicit Conversions
- Rust will not automatically apply implicit conversions between types. So you cannot multiply a `i8` by a `i16` with no issues.
  
Convert `i8` to `i16`:
```rs
let x: i8 = 15;
let y: i16 = 1000;
```
1. Convert to `i16` with `from<T>`
```rs
println!("{x} * {y} = {}", multiply(i16::from(x), y));
```
2. Convert to `i16` with `into<T>`
```rs
println!("{x} * {y} = {}", multiply(x.into(), y));
```

<br>

---
## Arrays and for Loops
<br>

### Printing an array with debug format
```rs
fn main() {
   let array = [10, 20, 30];
   println!("array: {array:?}");
}
```
```
array: [10, 20, 30]
```

<br>

### Iterating with for
You can iterate over an array similar to the syntax to python
```rs 
fn main() {
   let array = [10, 20, 30];
   for element in array {
      println!(" {element} ")
   }
}
```
```
10
20
30
```

Or indexing with a range may be used
```rs
fn main() {
   let array = ["hello", "there", "!"];
   for i in 0..3 {
      println!("Element at index {} is {}", i, array[i]);
   }
}
```
```
Element at index 0 is hello
Element at index 1 is there
Element at index 2 is !
```
<br>

### Exercise: Transpose matrix
[transpose_matrix.rs](https://github.com/steph1111/Comprehensive-Rust/blob/master/transpose_matrix.rs)

Print a 3x3 matrix and transpose its rows into columns. Implement a `pretty_print()` function that outputs the matrix in $\LaTeX{}$

<br>

$$
\begin{bmatrix}
1 &  2&  3\\
4 &  5&  6\\
7 &  8&  9\\
\end{bmatrix} \rightarrow  \begin{bmatrix}
1 &  4&  7\\
2 &  5&  8\\
3 &  6&  9\\
\end{bmatrix}
$$

<br>

---
## Variable Type Inference 
- The type of a variable may be specified, but it is not required 
- Rust will infer the type based on initialization 
  
<br>

---
## Static and Constant Variables

<br>

### Constant variables
```rs
const PI:f64 = 3.14159265;
```
- Have no fixed address in memory
- They’re inlined to each place which uses them, this means they are put directly into the binary on the places which use them.
- Usually faster runtime but bigger executable file because it doesn't have to look up an address like static
- Cannot be changed at runtime 

<br>

### Static variables
```rs
static BANNER: &str = "Welcome to RustOS 3.14";
```
- Have a fixed address in memory
- Their value is loaded from this fixed address each place which uses them.
- Usually slower runtime because we need to perform the extra instruction of loading the data from the fixed address. However this could result in a smaller executable file (only when it is used frequently) because it doesn't have to have multiple copies of the value baked into the executable.
- Can be changed at runtime if defined as `mut` but is unsafe

<br>

---
## Scopes and Shadowing
- Shadowing is different from mutation, because after shadowing both variable’s memory locations exist at the same time. Both are available under the same name, depending where you use it in the code.
- A shadowing variable can have a different type.
```rs
fn main() {
   let a = 10;
   println!("before: {a}");

   { // Shadowed area
      let a = "hello";
      println!("inner scope: {a}");

      let a = true;
      println!("shadowed in inner scope: {a}");
   }

   println!("after: {a}");
}
```

```
before: 10
inner scope: hello
shadowed in inner scope: true
after: 10
```

<br>

---
## Stack vs Heap 
Rust offers full control and safety via compile time enforcement of correct memory management.

<br>

### Stack 
- Continuous area of memory for local variables
- Values have fixed sizes known at compile time
- Extremely fast: just move a stack pointer
- Easy to manage: follows function calls
- Great memory locality

<br>

### Heap 
- Storage of values outside of function calls
- Values have dynamic sizes determined at runtime
- Slightly slower than the stack: some book-keeping needed
- No guarantee of memory locality

<br>

### Memory example with string 
```rs
fn main() {
   let s1 = String::from("Hello");
}
```
Fixed data goes in the tack and dynamic data goes in the heap 

<a href="https://ibb.co/sRpssCy"><img src="https://i.ibb.co/ZXRccSW/Untitled-design.png" alt="Untitled-design" border="0"></a>

<br>

---
## Ownership 

<br>

### Assignment
- An assignment statement transfers ownership of a variable
- There is always exactly one variable binding which owns a value
- Note: Some data types allow for copying of values in an assignment 

```rs 
fn main() {
   let s1: String = String::from("Hello!");
   let s2: String = s1; // Ownership of s1 transferred to s2
   println!("s1: {s1}");
   // Using s1 again results in an error^
}
```

<br>

### Function Calls 
- When you pass a value to a function, the value is assigned to the function parameter. This transfers ownership
- With the first call to `say_hello`, `main()` gives up ownership of `name`. Afterwards, name cannot be used anymore within `main()`.
```rs
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name); // Transfers ownership
    print!("{}", name); // Error, name in main() no longer has ownership
}
```
- **Fix 1:** Use references:
```rs
fn say_hello(name: &String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(&name);
    print!("{}", name);
}
```
```
Hello Alice
Alice
```

- **Fix 2:** Use cloning
```rs
fn say_hello(name: String) {
   println!("Hello {name}")
}

fn main() {
   let name = String::from("Alice");
   say_hello(name.clone());
   print!("{}", name);
}
```
```
Hello Alice
Alice
```

<br>

### Lifetime
- In Rust, a lifetime is a way to specify the scope for which a reference is valid
- A lifetime is a way to ensure that a reference is not used after the data it points to has been deallocated

<br>

---
## Exercise: Designing a Library

[library.rs](https://github.com/steph1111/Comprehensive-Rust/blob/master/library.rs)

References used:
- Vec: [Struct std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) 
- Option: [Module std::option](https://doc.rust-lang.org/std/option/)
- Iterators: [Trait std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

<br>

---
## Iterators
- The Iterator trait simply says that you can call next until you get `None` back

```rs
fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}
```

  <br>

---
## Structs

<br>

### Struct basics
```rs
struct Person {
   name: String,
   age: u8,
}

fn main() {
   let mut peter = Person {
      name: String::from("Peter"),
      age: 27,
   };
   println!("{} is {} years old", peter.name, peter.age);
    
   peter.age = 28;
   println!("{} is {} years old", peter.name, peter.age);
 }
```
```
Peter is 27 years old
Peter is 28 years old
```
- The syntax `..peter` allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. It must always be the last element.
```rs
let jackie = Person {
      name: String::from("Jackie"),
      ..peter
   };
   println!("{} is {} years old", jackie.name, jackie.age);
}
```
```
Jackie is 28 years old
```

- Methods are defined in an `impl` block
- Constructors are called `new` by convention
```rs
#[derive(Debug)]
 struct Person {
   name: String,
   age: u8,
 }

 impl Person { // Constructor 
    fn new(name: String, age: u8) -> Person { // Could also use -> Self
       Person { name, age }
    }
 }

 fn main() {
   let peter = Person::new(String::from("Peter"), 27);
   println!("{peter:#?}");
 }
```
- Use `{:#?}` when printing structs to request the debug representation
```rs
Person {
   name: "Peter",
   age: 27,
}
```

<br>

### Tuple Structs
- If the field names are unimportant, you can use a tuple struct
- These are useful when units are attached to the value or data verification is necessary (only odd numbers, phone numbers, etc) 
```rs
struct Newtons(f64);

fn set_thruster_force(force: Newtons) {
    // ...
}
```

<br>

---
## Enums
-  The enum keyword allows the creation of a type which has a few different variants
-  Where structs give you a way of grouping together related fields and data, like a `Rectangle` with its `width` and `height`, enums give you a way of saying a value is one of a possible set of values 
-  For example, we may want to say that `Rectangle` is one of a set of possible shapes that also includes `Circle` and `Triangle`
-  [Useful information on enums](https://www.programiz.com/rust/enum#google_vignette)
```rs
fn main() {

   // define enum color
   #[derive(Debug)]
   enum Color {
      Green,
      Yellow,
      Red,
   }

    // initialize and access enum variants
   let green = Color::Green;
   let yellow = Color::Yellow;
   let red = Color::Red;

   // print enum values
   println!("{:?}", green);
   println!("{:?}", yellow);
   println!("{:?}", red);
}
```
```
Green
Yellow
Red
```

<br>

---
## Method Receiver
- `&self`: borrows the object from the caller using a shared and immutable reference. The object can be used again afterwards.
- `&mut self`: borrows the object from the caller using a unique and mutable reference. The object can be used again afterwards.
- `self`: takes ownership of the object and moves it away from the caller. The method becomes the owner of the object. The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted. Complete ownership does not automatically mean mutability.
- `mut self`: same as above, but the method can mutate the object.
- No receiver: this becomes a static method on the struct. Typically used to create constructors which are called new by convention.

<br>

---
## Pattern Matching
- The match keyword let you match a value against one or more patterns. The comparisons are done from top to bottom and the first match wins.
- The patterns can be simple values, similarly to `switch` in C and C++: