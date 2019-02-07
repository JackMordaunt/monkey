# Monkey - Writing an Interpreter in Rust

> Based on the book "Writing an Interpreter in Go" by Thorsten Ball

Aims

- Apply my Rust knowledge to a well defined problem in order to spot deficiencies.
- Hone my understanding of fundamental programming principles through converting Go idioms to Rust idioms, in order to avoid being constrained to one particular programming language's approach.
- Understand how programming languages are constructed at least enough to implement one.

## Overview

### Components

1. Lexing
1. Parsing
1. Evaluating
1. Extending

### The Monkey Language

#### Features

- C-like syntax
- variable bindings
- integers and booleans
- arithmetic expressions
- built-in functions
- first-class and higher-order functions
- closures
- a string data structure
- an array data structure
- a hash data structure

#### Syntax

```monkey
let age = 1;
let name = "Monkey";
let reuslt = 10 * (20/2);
let my_array = [1, 2, 3, 4];
let my_hash = {"name": "Jack", "age": 23};

my_array[0] // -> 1
my_hash["name"] // -> Jack

let add = fn(a, b) { return a + b; };
let add_implicit = fn(a, b) { a + b; };

add(1, 2) // -> 3

// Recursion.
let fibonacci = fn(x) {
    if (x == 0) {
        0
    } else {
        if (x == 1) {
            1
        } else {
            fibonacci(x - 1) + fibonacci(x - 2);
        }
    }
};

// Higher order functions.

let twice = fn(f, x) {
    return f(f(x));
};

let addTwo = fn(x) {
    return x + 2;
};

twice(addTwo, 2); // -> 6
```

## Lexical Analysis

Source code (text) gets sequentially transformed into structures that are easy to manipulate and execute.

```sh
Source Code -> Tokens -> Abstract Syntax Tree
```

The first transformation (source code -> tokens) is called "lexical analysis", or "lexing".

Tokens are tiny datastructures that categorize every important unit of source code. Variable names, string, number and other literals, return statements, etc.

### Algorithm Philosophy

The lexing algorithim, that is the logic for transforming mere characters into tokens, follows this simple principle: handle the simplest, lowest information, inputs first.

In other words, handle all statically known tokens (singles first, then doubles, up to keywords), then handle dynamic tokens like identifiers, numbers and other literals.

## Parsing

Parsing is the process whereby tokens are structured into a (abstract syntax) tree, such that it is possible to evaluate.

I use a recursive descent parser because, while it doesn't have the beset performance characteristics, it models how we naturally think about abstract syntax trees. Basically, start at the root node and parse recursively.

For this parser "statements" do not produce values, whereas "expressions" do produce values.

## Performance Optimisations

### Zero allocation lexing

- Reuse String buffer tied to Lexer object.