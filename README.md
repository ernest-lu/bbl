We would like to implement BBL: a langauge designed specifically for single-threaded programs. Our goal is simple and concise pythonic / functional syntax with C++ like performance.

We achieve this speed by compiling to C++ and not using interpretation or garbage collection. The single-threaded nature of BBL means that we can avoid the overhead of garbage collection.

## Syntax
 - We declare variables using scala like 'val' and 'var'
 - We declare funcitons using scala like 'def'
 - We use 'rep' for loops and 'while'

## Types
    - Int (auto scales)
    - Float (auto scales)
    - String 
    - List (implemented as vector in C++)
    - Should ideally also support C++ structs
    - Tuples (implemented with)

## Goals
 - Everything is an expression
 - Higher order functions
 - Recursive lambdas
 - Indentation?
 - resizable ints
    - Don't have to think about ll etc
 - type inference
 - Simple tuples

We implement our the frontend of our compiler in Rust using lexing and parsing libraries.

We implement our backend in C++, compiling our bbl langauge to C++

- Implementation phases:
    - Lexer
    - Parser
    - Type checking
    - Codegen on AST
    - Functional features (map, filter, reduce, etc)


## Roadmap
- [x] Basic lexer
- [x] Basic parser
- [ ] Type checking
- [ ] Advanced code generation
- [ ] Functional features 
