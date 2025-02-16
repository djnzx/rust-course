## System Programming in Rust. Syllabus. I semester

### Lecture 1. Intro

- programming intro
- different paradigms
- different classifications
- why Rust

### Lecture 2. Basic types

- fundamental datatypes
- signed/unsigned
- negative representation
- 8/16/32/64/128/size
- f32/f64 representation
- bool
- unit
- never

### Lecture 3. Arrays

- arrays
- methods
- slices
- for
- iterations
- enumeration
- performance

### Lecture 4. Strings, Char, Encodings

- strings
    - methods
    - slices
- encodings, UTF-8
- for
- iterations

### Lecture 5. Product types: Tuple, Named Tuple, Struct

- what programming is
- data vs code
- product data types
    - tuples
    - named tuples
    - structs
- destructurization
- examples
- pattern matching

### Lecture 6. Coproduct types: Enum

- what programming is
- data vs code
- coproduct datatypes
    - enum
- pattern matching
-

[//]: # (TODO: Standard enums with examples)

- standard enums
    - Option[A], no null, no undefined
    - Result[A, E]
    - Ordering
    - combining Option
    - combining Result with ?

### Lecture 7. Pattern Matching

- pure data
- if guard
- exhaustiveness
- or `|` combination
- else case
- Products: (Tuple, Named Tuple, Struct)
- Coproducts: Enum
- Arrays, slices, ranges
- matches! macro
- if let

### Lecture 8. Control Flow.

- `if` / `else` basic
- `if` / `else` stacked
- binary operations, boolean algebra
- `if` returns
- `for` / `while` / `loop`
- `continue` / `break`
- `loop` returns
- `match`
- `match` returns
- labels

### Lecture 9. Functions, Closures

- grouping things by functionality
- single responsibility
- modules
- use
- structurizing code
- functions
    - input parameter(s)
    - output parameter(s)
- closures

### Lecture 10. Impl, Traits

- impl - wiring code to data, &self
    - impl for struct + fn + static fn + const
    - impl for enum + fn + static fn + const
- traits - sharing behavior
    - impl for trait
- standard traits
    - Add / Sub / Mul / ...

-

[//]: # (TODO: Standard traits with examples)

    - Display
    - Debug
    - From
    - Into
    - Equality, partial
    - Ordering
    - Equality

### Lecture 12. Collections, Generics

- array
- String
- Vector
- VecDeque
- HashMap
- HashSet
- BTreeMap
- BTreeSet

### Lecture 12. Iterators

https://doc.rust-lang.org/std/iter/trait.Iterator.html

- iterators
    - constructing
    - using
        - not reusable
        - .next
        - map
        - filter
        - indexes
        - zip
        - skip_while
        - take_while
        - flatmap
        - flatten
        - partition
    - terminating
        - find
        - first
        - last
        - sum
        - count
        - collect
        - for each

### Lecture 13. Memory Management. Scopes

- value vs reference
- stack vs heap
- reference
- dereference
- mutability vs immutability
- borrowing and ownership

### Lecture 14. I/O

- input/output
    - random
    - console, parsing
    - text file
    - binary file
    - network socket
    - port i/o
    - API / webserver

### Lecture 15. Testing

- what to test
- how to test
- assert_eq
- #[test]
