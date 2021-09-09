![Rust](https://github.com/matyama/rust-examples/workflows/Rust/badge.svg)

# Rust Examples
This repository contains support materials for Rust tech-talks explaining various techniques and Rust specificts.

## Topics and Concepts
* *Static* vs *dynamic* dispatch
* *Ownership* and memory management
    * *Aliasing XOR Mutability* principle
	* `Box`, `Rc` and `&` and their `Clone` semantics
* Type system features:
	* Enums as Algebraic Data Types
	* Pattern matching
	* Failure effects as types: `Option` instead of `null` pointer,
		`Result` instead of recoverable exceptions
	* (Bounded) Parametric polymorphism
    * Rank-2 polymorphism
    * Subtyping and variance
    * Constant generics
	* Zero-cost abstraction
	* Types as carriers of (static) semantics
* Metaprogramming:
	* Declarative macros
* Functional design patterns:
	* Traversable types implementing `FromIterator` and using `collect`
