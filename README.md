# FPPC-rs: Flexible Path Pattern Calculus in Rust

> **High-performance implementation of the Flexible Path Pattern Calculus (FPPC) for GQL.**

This repository contains the Rust implementation of FPPC, a gradual type system for the Graph Query Language (GQL). This project aims to migrate the original Python prototype presented at OOPSLA 2025 to a high-performance compiled language, providing static analysis capabilities for graph queries with significant efficiency gains.

## 📄 Context

Graph Query Language (GQL) is the new ISO standard for querying property graphs. However, it lacks a formal static type system, leading to runtime errors or empty results without prior warning.

**FPPC** (Flexible Path Pattern Calculus) addresses this by extending GQL with:

  * **Gradual Typing:** Supporting both precise and imprecise (unknown) types.
  * **Property-based Filtering:** Extending pattern matching to check property types statically.

This implementation focuses on performance and robustness, utilizing Rust's type safety and the LALRPOP parser generator.

## 🚀 Features (Current Status)

This project is currently in the **Parsing & AST** phase.

  * **Modular LALRPOP Grammar:** The syntax is defined across modular files for maintainability (`types`, `label`, `expr`, `pattern`).
  * **Full Node Pattern Support:** Parses descriptors with variables, labels, and property records.
  * **Complex Filter Expressions:** Supports `WHERE` clauses with correct operator precedence (Arithmetic, Logical, Comparison).
  * **Concrete Syntax Adaptations:**
      * **Unions:** Uses `|` (pipe) instead of `+` for label unions (e.g., `Person | Student`).
      * **Records:** Distinguishes between **Open** `{...}` and **Closed** `{{...}}` property records.
  * **Interactive REPL:** A built-in console for testing parsing rules on the fly.

## 🛠️ Installation & Usage

Ensure you have Rust installed (stable toolchain).

### 1\. Build the Project

The build script automatically processes the `.lalrpop` grammar files.

```bash
cargo build
```

### 2\. Run Tests

The project follows a TDD approach. You can run the regression suite (ported from the Python prototype) to validate the parser:

```bash
cargo test
```

### 3\. Interactive Console (REPL)

You can experiment with the parser directly using the CLI tool:

```bash
cargo run
```

**Available Commands:**

  * `label <input>`: Parse label expressions (e.g., `Person & Teacher`).
  * `descriptor <input>`: Parse node descriptors (e.g., `(n: Person {name: str})`).
  * `path <input>`: Parse full path patterns (e.g., `(x WHERE x.age > 18)`).
  * `expr <input>`: Parse filter expressions (e.g., `x.a > 10 AND y.b < 5`).
  * `simple <input>` / `property <input>`: Parse types and property records.

**Example Session:**

```text
> path (x: Person {name: str})
✓ Valid: Node(Descriptor(Some(Var(x)), DescriptorType(Label(Person), Open({name: Base(String)}))))
```

## 📂 Project Structure

```text
.
├── build.rs                # Concatenates modular grammar files before compilation
├── Cargo.toml              # Dependencies: lalrpop, regex, etc.
└── src
    ├── ast/                # Abstract Syntax Tree definitions (Rust structs)
    │   ├── descriptor.rs
    │   ├── expr.rs
    │   ├── label.rs
    │   ├── pattern.rs
    │   ├── types.rs
    │   └── var.rs
    ├── grammar/            # LALRPOP grammar definitions
    │   ├── descriptor.lalrpop
    │   ├── expr.lalrpop
    │   ├── label.lalrpop
    │   ├── mod.lalrpop     # Root grammar file
    │   ├── pattern.lalrpop
    │   └── types.lalrpop
    ├── lib.rs              # Library entry point & TDD Test Suite
    └── main.rs             # CLI/REPL implementation (for experimentation/debugging)
```

## 📚 References

  * **Paper:** Ye, W., Toro, M., et al. (2025). *Flexible and Expressive Typed Path Patterns for GQL*. Proc. ACM Program. Lang. 9, OOPSLA2.
  * **Original Prototype:** Python implementation (referenced for functional equivalence).

## 👤 Author

**Felipe Ignacio Avendaño Araya**
*University of Chile*

-----

*This project is part of a Computer Science Engineering thesis to provide a high-performance implementation of FPPC.*
