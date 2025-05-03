# C4 Rust Compiler

A Rust-based reimplementation of the [C4 compiler](https://github.com/rswier/c4), a minimal self-hosting C compiler.  
This version compiles a subset of the C language to custom bytecode, which is executed by a virtual machine (VM) written in Rust.

---

## 🔧 Build and Run Instructions

1. **Build the project:**

```bash
cargo build --release
```

2. **Run on the full feature test file:**

```bash
cargo run --release -- input/test_all_features.c
```

3. **Run tests:**

```bash
cargo test --test behavior_tests
```

---

## 📘 Documentation

To generate and view the project documentation:

```bash
cargo doc --no-deps --open
```

This will open the Rust API documentation for the compiler’s internal modules, including:

- `parser.rs` – The recursive descent parser
- `lexer.rs` – Tokenizes the C input
- `vm.rs` – Executes bytecode
- `instruction.rs` – Defines bytecode instructions
- and more...

You can also view the documentation in `target/doc/c4_rust/index.html` after generation.

---

## ✅ Behavior Tests

This project uses **Rust’s Built-in Test Framework** (`#[test]`) to verify compiler correctness.  
In addition, a dedicated **`behavior_tests.c`** file tests the end-to-end behavior of the compiled C code using the VM.

### Features Covered in `behavior_tests.c`

| Feature Category             | Description                                                                 |
|-----------------------------|-----------------------------------------------------------------------------|
| **Arithmetic Operators**     | `+`, `-`, `*`, `/`, `%`                                                     |
| **Comparison Operators**     | `==`, `!=`, `<`, `<=`, `>`, `>=`                                            |
| **Logical Operators**        | `!` (logical not)                                                           |
| **Bitwise Operators**        | `&`, `|`, `^` (XOR), `<<`, `>>`                                             |
| **Unary Operators**          | `-` (negation), `*` (dereference), `&` (address-of)                         |
| **Variables**                | Declaration, assignment, and usage of `int`, `char`, and `float`            |
| **Function Calls**           | Functions with and without arguments                                        |
| **Control Flow**             | `while` loops, `if` expressions, ternary (`?:`) operator                    |
| **Pointer Operations**       | Declare pointers, assign addresses, dereference                            |
| **Type Casting**             | `(int)` and float-to-int casting (no-op)                                    |
| **Sizeof Operator**          | `sizeof(int)` and `sizeof(char)`                                           |
| **Character Literals**       | Single characters like `'A'` and `'Z'`                                     |
| **String Literals**          | Including escape sequences like `"WOOOW"`                                     |
| **Print Output**             | Printing integers, floats, characters, and strings                         |
| **Floating-Point Arithmetic**| `+`, `-`, `*`, `/` on `f64` literals and mixed `int`/`float` expressions    |

---

## 🚀 Bonus Feature: Floating-Point Support

The original C4 did not support floating-point numbers.  
This project adds support for:

- Floating-point literals (`1.5`, `0.0`, etc.)
- Mixed-mode arithmetic (`int + float`, `float / int`, etc.)
- Print support for `f64`
- Float-aware bytecode instructions (`PushF`, `PrintF`, etc.)

All handled via the `Value::Flt(f64)` variant in the VM.

---

## 🧪 Testing Strategy

- Lightweight unit tests using Rust's `#[test]` framework.
- Full program behavior tests using `behavior_tests.c` to validate end-to-end correctness.
- Each compiler feature is tested both statically and at runtime.

---
