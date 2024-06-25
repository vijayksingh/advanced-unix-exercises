
Here are exercises for each of the Rust concepts we've discussed. These exercises will help you practice and reinforce your understanding of each concept.

1. `use` keyword:
Exercise: Create a program that uses the `rand` crate to generate random numbers. Use the `use` keyword to bring the necessary items into scope.

```rust
// Hint: You'll need to add rand = "0.8.5" to your Cargo.toml
// Then use the 'use' keyword to import necessary items from rand
// Generate and print 5 random numbers between 1 and 100
```

2. `Vec<T>`:
Exercise: Create a function that takes a vector of integers, doubles each element, and returns a new vector with the results.

```rust
fn double_elements(input: Vec<i32>) -> Vec<i32> {
    // Your code here
}

// Test your function in the main function
```

3. `env::args()`:
Exercise: Write a program that accepts command-line arguments and reverses them. Print both the original arguments and the reversed arguments.

```rust
use std::env;

fn main() {
    // Your code here
    // Hint: Collect args, reverse them, and print both original and reversed
}
```

4. `process::exit()`:
Exercise: Write a program that asks the user for a password. If the password is "rust_is_fun", print a success message. Otherwise, print an error message and exit the program with a non-zero status code.

```rust
use std::io;
use std::process;

fn main() {
    // Your code here
}
```

5. `io::stderr()` and `writeln!` macro:
Exercise: Create a function that simulates a logging system. It should accept a log level ("INFO", "WARNING", or "ERROR") and a message, then write to stdout for INFO and to stderr for WARNING and ERROR.

```rust
fn log(level: &str, message: &str) {
    // Your code here
    // Hint: Use println! for INFO, and writeln! with io::stderr() for others
}

fn main() {
    // Test your log function with different levels
}
```

6. `fs::read_dir()`:
Exercise: Write a program that counts the number of files and directories in a given path. Print the total count of each.

```rust
use std::fs;

fn count_files_and_dirs(path: &str) -> (u32, u32) {
    // Your code here
    // Return a tuple of (file_count, dir_count)
}

fn main() {
    // Test your function with a path
}
```

7. `match` expression:
Exercise: Create a function that takes a `Result<i32, String>` and returns a string describing the result. Use a match expression to handle both Ok and Err cases.

```rust
fn describe_result(result: Result<i32, String>) -> String {
    // Your code here
    // Return "Success: <value>" for Ok, and "Error: <error message>" for Err
}

fn main() {
    // Test your function with both Ok and Err values
}
```

8. `Result`, `Ok`, and `Err`:
Exercise: Write a function that attempts to parse a string into an integer, but returns a custom error message if parsing fails.

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    // Your code here
    // Hint: Use the parse() method and map_err() to customize the error
}

fn main() {
    // Test your function with valid and invalid inputs
}
```

9. `if let` expression:
Exercise: Create a function that takes an `Option<String>` and prints the string if it exists, otherwise prints "No value". Use `if let` for this.

```rust
fn print_if_exists(opt: Option<String>) {
    // Your code here
}

fn main() {
    // Test your function with Some and None values
}
```

10. `to_string_lossy()`:
Exercise: Write a program that reads filenames from a directory and prints them, ensuring that any non-UTF8 characters are replaced.

```rust
use std::fs;

fn main() {
    // Your code here
    // Hint: Use fs::read_dir() and to_string_lossy() on each filename
}
```

These exercises will give you hands-on practice with each of the Rust concepts we've discussed. Try to complete them on your own, and don't hesitate to refer to the Rust documentation if you need more information on any of the functions or methods used.
```rust
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("Hello", 1);
  map.insert("World", 2);
  println!("{:?}", map);
}
```
```output
{"Hello": 1, "World": 2}
```
```rust
fn double_elements(input: Vec<i32>) -> Vec<i32> {
  input.iter().map(|x| x *2 ).collect()
}

println!("{:?}", double_elements(vec![1, 2, 3, 4, 5]));
```
```output
[2, 4, 6, 8, 10]
```
```rust
use std::fs;

fn count_files_and_dirs(path: &str) -> (u32, u32) {
  let mut file_count = 0;
  let mut dir_count = 0;

  for entry in fs::read_dir(path) {
    match entry {
      Ok(entry) => {
        if entry.metadata()?.is_dir() {
          dir_count += 1;
        } else {
          file_count += 1;
        }
      }
      Err(e) => {
        println!("Error reading directory: {}", e);
      }
    }
  }
  (file_count, dir_count)
}

println!("{:?}", count_files_and_dirs("/Users/vijaysingh/Projects/Rust/advanced-unix/"));
```
```output
   Compiling output v0.0.1 (/var/folders/t_/j83k01rs3l1fhn2rkj64lzd80000gn/T/codebook/rust)
error[E0308]: mismatched types
  --> src/main.rs:66:40
   |
66 | fn count_files_and_dirs(path: &str) -> (u32, u32) {
   |    --------------------                ^^^^^^^^^^ expected `(u32, u32)`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   |
   = note:  expected tuple `(u32, u32)`
           found unit type `()`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `output` (bin "output") due to 1 previous error
```
```rust

// Exercise 7
fn describe_result(result: Result<i32, String>) -> String {
  match result {
    Ok(x) => format!("Success: {}", x),
    Err(e) => format!("Error : {}", e)
  }
}

println!("{}", describe_result(Ok(10)));
println!("{}", describe_result(Err(String::from("Error"))));
```
```output
Success: 10
Error : Error
```
```rust

```
