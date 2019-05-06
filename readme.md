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

## Differences between Rust impl and reference Go impl

### Parse function mapping

Parse functions in the reference are stored in a map as token:function pairs.
Instead of this I use a match statement where the branches are the token kind,
and the arm block is the function.

This would be closely analagous to using a switch statement in Go, by switching
over the token.

The match arms are static. This means that you know exactly what tokens you can
parse at compile time. Whereas the map approach is dynamic; you have to register
each fn before parsing and handle changes at runtime. Furthermore, the map 
(theoretically) consumes more memory. 

### Token representation

Go simulates enums by using statically declared constants. Thus, the set of different tokens is represented by constant strings. This implicitly couples string literals with token kinds. In Rust we have an enum type, so token kind is just an enum. 

### AST

The AST is a tree structure with nodes. Each node represents some piece of syntax yet all different nodes must be composable in the same tree.

To achieve this in Go, the reference uses a marker interface. Which means every node type must "opt-in" by implementing a dummy interface.

This simulates an open ended enum, where not all variants are known at compile time. In Rust, we just use a Node enum containing all possible node variants.

### Naming

I thought some of the naming style in the reference were very procedural and noisy. The names I chose are quick to grok, easier to type and have a more fluent feel. The names on the left are "noisy" in part because they repeat information that is already embedded in the context. 

For example, since the peeking operation _returns_ a `Token` we can immediately see that the subject of the peek is a `Token`. Embedding the subject also in the name is pure noise, thus `peek` is the better name.

- `peekToken` -> `peek`
- `curTokenIs` -> `token`
- `nextToken` -> `advance`
- `expectPeek` -> `expect`
- `ParseProgram` -> `parse`


### Error Handling

The reference appends errors to a list at the place the error occurs.
My version bubbles each error up to the parsing loop, which is where it gets appended to a list. 

The syntactic difference is that my methods return errors in their signatures, and the semantic difference is that bubbling errors allows you to record context for the error. 


### Testing 

Since Go doesn't have operator overloading, the reference approaches testing from a procedural perspective. It tests each part of the data structure for the right types of objects and values of primitives.

Rust does have operator overloading, so I can just `want == got` for my test making it much more succinct. 

Both styles of testing are dominantly table driven. 