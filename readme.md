# Rlox - Lox in Rust

This is a simple implementation of JLox/CLox in the Rust programming language.
The purpose of this project is to teach myself Rust and nothing else.

The original implementation is being written in Java (where the language gets out of my way),
and then a re-write in Rust to make it fast as well as learn Rust.

## Notes

- A direct translation was not super difficult.
- Java classes = Rust structs. Constructors = Impl
- Using Rust's Enum type to represent Literals where `Object` is used in Java
- Ownership and move semantics make things tricky (but not super tricky so far)
- **Design decision**: How do we represent characters and strings? How do we iterate over the source code?
- Single characters are represented as `&String`, which is not a good idea. Perhaps `u8` or `&str` or `char`?
  - What options do we have?

- **New observation**: I've come up with some new terminology and concepts to classify how I write my code. Broadly speaking,
"bits of code" can be classified into

## TODO


- [ ] Better string representation in Lexer
- [ ] Add Error Handling
- [ ] Figure out how the Syntax tree will work. Recursive data structures are supposed to be nasty in Rust.
- [ ] Add Top-down Recursive Descent Parser
- [ ] Panic-mode Error Recovery via Synchronization
- [ ] Support for Binary Expression Parsing
- [ ] Support for Ternary Expressions
- [ ] Support for Statements
- [ ] Measure performance vs JLox implementation

## Resources

- Eli Bendersky's Blog
- Crafting Interpreters
- https://lisperator.net/pltut/parser/the-ast