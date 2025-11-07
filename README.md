# 🦀 Understanding Ownership in Rust

---

This Rust program demonstrates the **core concept of ownership** — one of the most unique and powerful features of the Rust programming language.  
It explores how Rust manages **memory safety without a garbage collector** through rules that govern **variable scope, borrowing, moving, and cloning**.

---

## ⚙️ Setting Up the Environment

Before running the program, make sure **Rust** and **Cargo** are installed on your system.

### 🦀 Step 1: Check for Rust Installation
```bash
rustc --version
```
If Rust isn’t installed yet, you can install it using rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```
After installation, verify both:

```bash
rustc --version
cargo --version
```

## 📁 Step 2: Create the Project Folder
Create a new Rust project using Cargo:

```bash
cargo new Ownership
```
Then navigate into the folder:

```bash
cd Ownership
```
Replace the contents of src/main.rs with the following code.

## 📜 Rust Code
rust
```
fn main() {

    // prinln!("{}",s); // variable s is not valid here since it hasn't been declared yet
    let _s: &str = "hello";    // variable is valid here
    
    println!("{}", _s); // variable is valid here because it has already been declared.
    
    // s can do stuff here.

    let mut _d = String::from(_s);
    
    _d = String::from("ahoy"); // "hello" that was assigned is cleared from memory.
    _d.push_str(", world"); // push_str() appends a literal to a string

    println!("{_d}");

    // Memory Allocation in Integers (Copy Trait)
    let _x = 5;
    let _y = _x; // Simple copy since integers are stored on the stack

    // Memory Allocation in Strings (Move Semantics)
    let s1 = String::from("hello");
    let _s2 = s1; // Ownership of s1 is moved to s2 — s1 is now invalid

    // Cloning explicitly copies heap data
    let _s3 = _s2.clone();
    println!("_s2: {_s2}, _s3: {_s3}");
    
    let s = String::from("hello"); // s comes into scope

    // takes_ownership(s); // s moves into function and is no longer valid here
    let _f = takes_ownership(s); // Ownership is transferred and returned
    let x = 5;                   // x comes into scope

    makes_copy(x);               // x implements Copy, so it's still valid after
}

// When the scope ends, variables go out of scope and their memory is released.

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // `some_string` goes out of scope here, and `drop` is called automatically

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Copy types are unaffected when passed — stack values are simply duplicated
```

---

## ▶️ Step 3: Build and Run
🧱 Build
```bash
cargo build
```
🎬 Run
```bash
cargo run
```

---

## 🧠 Key Concepts Demonstrated
Concept	Description
Ownership	Each value in Rust has a single owner. When the owner goes out of scope, the value is dropped.
Move Semantics	Assigning a variable to another transfers ownership (e.g., let s2 = s1;). The original variable becomes invalid.
Clone	Explicitly copies data on the heap. Useful when you need two distinct variables with the same value.
Copy Trait	Primitive types like integers (i32, bool) implement the Copy trait and are duplicated on assignment instead of moved.
Borrowing & References	This example focuses on ownership; borrowing will be explored in later chapters.
Automatic Memory Management	Rust’s ownership model ensures safe, deterministic memory cleanup without a garbage collector.

---

## 🧾 Example Output
```bash
hello
ahoy, world
_s2: hello, _s3: hello
hello
5
```

---

## 🚀 Future Enhancements
Add examples demonstrating borrowing (&T) and mutable references (&mut T).

Visualize memory layout with diagrams.

Expand with comments showing compile-time errors when ownership rules are broken.

---

## 🦀 Built With
Rust

Cargo

---

## 📄 License
This project is open-source and available under the MIT License.
