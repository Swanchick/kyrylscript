# KyrylScript

**KyrylScript** is a lightweight, high-performance programming language designed for embedded systems — bridging the gap between high-level expressiveness and low-level control.

> **Note:** This project is in active development. Expect frequent changes and potential bugs.

---

## Features

### Custom Lexer and Parser
Hand-written in Rust, featuring clean syntax trees and well-defined tokenization rules — giving full control over parsing behavior with no external dependencies.

### Runtime and Scoping
Lexical scoping with reference-based variable tracking. No garbage collector — memory is managed manually using reference IDs, making it ideal for resource-constrained environments.

### Type System
Built-in support for a rich set of primitive and compound types:
`number` · `string` · `boolean` · `list` · `tuple` · `function` · `native`

### KIRIL — Kyryl's Intermediate Runtime & Intermediate Language
A purpose-built runtime compiled from scratch, designed to run as a **separate process** that accepts a stream of IL (Intermediate Language) instructions. KIRIL is capable of running **without an OS**, utilizing a custom allocator for full environment independence.

---

## Getting Started

### 1. Clone the Repository
```bash
git clone https://github.com/Swanchick/kyrylscript.git
cd kyrylscript
```

### 2. Build & Run
```bash
cargo run -- examples/test.ks -p ks-cli
```

---

## License

GNU General Public License v3.0 © 2026 Kyryl Lebedenko

---

*Created with love by Kyryl Lebedenko :3*

