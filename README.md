
# Monkey Lang Interpreter - Rust Library Implementation

This is an interpreter for Monkey, a simple programming language designed for educational purposes. The Monkey language is dynamically typed and features a C-like syntax.


**Note: This interpreter is intended to be used as an internal library for CLI (Clap) and Web/Svelte WebAssembly (WASM) applications, as well as GUI applications built with the Tauri framework.**

## Features

- Lexer: The interpreter includes a lexer that tokenizes the input Monkey code.
- Parser: The parser converts the tokens into an abstract syntax tree (AST).
- Interpreter: The interpreter traverses the AST and executes the Monkey code.
- REPL: The interpreter provides a Read-Eval-Print Loop (REPL) for interactive code execution.
- Basic Data Types: Monkey supports basic data types such as integers, booleans, strings, arrays, and hash maps.
- Control Flow: Monkey provides if/else statements andwhile loops for control flow.
- Functions: Monkey supports defining and calling functions.
- Closures: Monkey supports closures, allowing functions to access variables from their surrounding scopes.
- Error Handling: The interpreter provides basic error handling and displays informative error messages.

## Usage (WIP)

The Monkey Lang Interpreter can be used as an internal library for various platforms:

- CLI: You can integrate the interpreter into your command-line applications to execute Monkey code.
- Web (WASM + Svelte): The interpreter can be compiled to WebAssembly (WASM) and used in web applications built with Svelte.
- GUI (Tauri): The interpreter can be integrated into GUI applications using the Tauri framework for building cross-platform desktop apps.
