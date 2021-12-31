![Rust](https://github.com/matyama/rust-examples/workflows/Rust/badge.svg)

# Rust Examples
This repository collects examples of various programming language
concepts implemented in [Rust](https://www.rust-lang.org/) and generally
introduces the language.

All the examples are documented and might serve as support materials for
future Rust tech-talks and presentations.

For detailed code description generate and see the docs:
```bash
cargo doc --open
```

## Disclaimer
Please note that these examples are a personal take on the subject (and
sort of a learning playground for myself) and you should definitely check
out the [official study materials](https://www.rust-lang.org/learn) which
are fantastic!

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
	* Dependent types (relation to Curry-Howard correspondence)
  * Coherence and orphan instances
  * Top and bottom types
* Metaprogramming:
	* Declarative macros
* Functional design patterns:
	* Traversable types implementing `FromIterator` and using `collect`

## Module Overview
1. [`memory`](src/memory.rs) - ownership and borrowing, aliasing xor
	 mutability, lifetimes
1. [`errors`](src/errors.rs) - exceptions as local values, effect types
	 `Option` and `Result`, pattern matching
1. [`typing`](src/typing.rs) - zero-cost abstraction, compile-time
	 semantics, statically checked invariants of unsafe code, top and
	 bottom types in Rust
1. [`dependent`](src/dependent.rs) - dependent types, programs as proofs
1. [`adts`](src/adts.rs) - algebraic data types, self-referential types,
	 pattern matching
1. [`dispatch`](src/dispatch.rs) - monomorphization and static vs
	 dynamic dispatch, polymorphic functions
1. [`rc`](src/rc.rs) - `&`, `Box` and `Rc` comparison (sharing,
	 mutability and `Clone` semantics)
1. [`collect`](src/collect.rs) - sequencing effects with `collect`
1. [`macros`](src/macros.rs) - declarative macros
1. [`rsqrt`](src/rsqrt.rs) - fast inverse square root, zero-cost
	 abstraction, making illegal states unrepresentable, constant generics
1. [`brands`](src/brands.rs) - zero-cost abstraction, rank-2
	 polymorphism, branded types, subtyping and variance
1. [`orphan`](src/orphan.rs) - trait system, coherence and orphan rules
