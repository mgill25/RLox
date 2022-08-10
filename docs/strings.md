# Representing Strings in Rust

ASCII:
    1 character = 7 bits (Originally)
    Hence, total of 128 characters

UTF-8:
    Unicode Transformation Format: 8 bits (1 byte)
    Variable Width Character Encoding
    1 Character = 1 to 4 bytes
    > 1 million characters
    Backwards Compatible with ASCII

    num_bytesxxxxx

    How UTF-8 encodes characters in bytes:
        0xxxxxxxxxxx <- 1 byte
        110xxxxxxxxx 10xxxxxx <- 2 bytes
        1110xxxxxxxx 10xxxxxx 10xxxxxxx <- 3 bytes
        11110xxxxxxx 10xxxxxx 10xxxxxxx 10xxxxxxx <- 4 bytes

    Essentially encompasses all characters in the world and also emojis
    Example: Crab Emoji takes 4 bytes

    Text in Rust is encoded using UTF-8

## Different Types of Strings in Rust

1. `&str`
2. `String`


&str (String Slice):
    - Provided by the core language
    - View into a sequence of UTF-8 encoded bytes (of dynamic length).
    - These bytes can be stored in the application's binary, the stack or the heap.
    - We can't know the amount of bytes at compile time (since it's dynamic) for example when on the heap.
        - (exception: when in the binary, then we would know at compile time)
        - Since we don't know for sure always the size, we don't use `str` directly, but the borrowed form (`&str`)
            - Commonly used as String Slice
            - We don't know the size of `str`, but we do know the size of `&str` which is the string slice
            - &str contains the address and length.
    - String slice does not *own* the underlying data.

    - So if you just need an immutable view of a string, or part of a string, just use the string slice (&str)

String:
    - Provided by the standard library
    - Always allocated on the heap
    - (addr, length, capacity)
    - *Owned* type. We can manipulate the string however we like.
        - Especially if we want to own the data and manipulate it.

String Literals:
    - Things inside double quotes. They are &str (string slices) and are stored in the application binary

```rust
let s1: &str = "Hello, World!";
let s2 = String::from("Hey there");
let s3 = "Hello World".to_string();
let s4 = "Woah dude".to_owned();
let s5 = &s4[..];
```

### Manipulating Strings

```rust
let mut s = String::from("Foo");        // Must be mut if we want to manipulate
s.push_str(", bar");

s.replace_range(.., "baz"); // replace the entire string with baz
```