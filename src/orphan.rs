//! This module demonstrates *Orphan rules* and *coherence* of Rust's trait system.
//!
//! # Coherence
//! *Coherence* can informally be understood as a property of trait implementations (typeclass
//! instances) in which the execution semantics does not change with different scopes (contexts).
//!
//! *This definition paraphrases a definition from this
//! [post](http://blog.ezyang.com/2014/07/type-classes-confluence-coherence-global-uniqueness/).*
//!
//! # Global uniqueness of instances
//! This property states that for any type, there is at most one instance resolution for a given
//! type class. An instance violating this property is called the
//! [*orphan instance*](https://wiki.haskell.org/Orphan_instance).
//!
//! In Rust this property is achieved by the *orphan rules* which state that at least one of the
//! following must hold for any crate and `impl Type for Trait`:
//!  1. `Type` is owned (defined) by the implementing crate
//!  1. `Trait` is owned (defined) by the implementing crate
//!
//! *For more see [Chalk's docs](https://rust-lang.github.io/chalk/book/clauses/coherence.html).*
//!
//! # Example: Orphan rule
//! In the example below neither the type [Vec] nor the trait [ToString] is owned by this crate, so
//! the code would create an *orphan instance* which is disallowed and won't compile.
//! ```compile_fail
//! struct Data(String);
//!
//! impl ToString for Vec<Data> {
//!     fn to_string(&self) -> String {
//!         self.iter().map(|data| data.0.clone()).collect::<Vec<_>>().join(" ")
//!     }
//! }
//! ```
//!
//! # Example: Newtype pattern
//! Typical solution to the problem above is the [*newtype*](https://wiki.haskell.org/Newtype)
//! pattern.
//! ```
//! struct Data(String);
//!
//! // The *newtype* for `Vec<Data>`
//! struct DataVec(Vec<Data>);
//!
//! impl ToString for DataVec {
//!     fn to_string(&self) -> String {
//!         self.0.iter().map(|data| data.0.clone()).collect::<Vec<_>>().join(" ")
//!     }
//! }
//! ```
//! In this case `DataVec` is a new type defined in this crate which is enough to satisfy the
//! orphan rules.
//!
//! # Discussion
//!
//! ## Cons
//!  - The ergonomics of defining and using new types is not great. First there's the obvious code
//!    bloat and more importandly the type system doesn't simply generalize all the `impl`s of the
//!    wrapped type which leads to the `self.0` gymnastics.
//!  - Someone might argue that for end applications it might be beneficial to dynamically change
//!    code behavior depending on the implementing module
//!
//! ## Pros
//!  - From a theoretical point of view, function's behavior should not depend on the *scope* in
//!    which it is invoked but rather on the *type* on which it is called (i.e. types should fully
//!    determine behavior)
//!  - This prevents subtle and hard to spot bugs such as the *Hash table problem* or ordering
//!    inversion in certain data structures
//!  - Due to these rules `cargo` can resolve dependencies with two versions of the same crate.
//!    This increases the distribution of development in the Rust ecosystem (for instnance one does
//!    not have to wait for an update of crate X when updating Y when both depend on Z)
//!  - For instance this allows adding extensions to the `std` crate without creating breaking
//!    changes (resulting in a minor or major version change)
//!  - Future Rust could potentially support *Specialization* which would not be possible with
//!    orphan instances (i.e. allow safe ad-hoc behavior for typeclass instances which are strict
//!    subsets of more general ones defined elsewhere)
//!  - The ergonomics of "newtypes" could be improved with something like `#[newtype_deriving]`

/// Module that defines single data type called `Entity`
pub mod model {

    /// Union type which defines two variants [`X`](Entity::X) and [`Y`](Entity::Y)
    #[derive(Debug, PartialEq, Eq)]
    pub enum Entity {
        X,
        Y,
    }
}

/// Module which exposes an operation which depends on an instance of [Ord] for [model::Entity]
pub mod module_a {
    use crate::orphan::model::Entity;
    use std::cmp::Ordering;

    /// Implements [Ord] for [Entity] in which [`X`](Entity::X) > [`Y`](Entity::Y)
    impl Ord for Entity {
        fn cmp(&self, other: &Self) -> Ordering {
            use Entity::*;
            match (self, other) {
                (X, X) | (Y, Y) => Ordering::Equal,
                (X, Y) => Ordering::Greater,
                (Y, X) => Ordering::Less,
            }
        }
    }

    impl PartialOrd for Entity {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    /// Sorts given entities using [Ord] implementation for [Entity] contained in this module
    pub fn prioritize(entities: &mut [Entity]) {
        entities.sort();
    }
}

#[cfg(test)]
mod tests {
    use crate::orphan::model::Entity;
    use crate::orphan::module_a::prioritize;

    #[test]
    fn idempotent_prioritize() {
        let mut entities = vec![Entity::X, Entity::Y];

        prioritize(&mut entities);
        assert_eq!(entities, vec![Entity::Y, Entity::X]);

        prioritize(&mut entities);
        assert_eq!(entities, vec![Entity::Y, Entity::X]);
    }
}

/// This test demonstrates that Rust disallows *Orphan Instances*.
///
/// # Variation of the *Hash table Problem*
/// ```compile_fail
/// pub mod module_b {
///     use crate::orphan::model::Entity;
///     use std::cmp::Ordering;
///
///     // Implements `Ord` for `Entity` in which `X` < `Y`
///     impl Ord for Entity {
///         fn cmp(&self, other: &Self) -> Ordering {
///             use Entity::*;
///             match (self, other) {
///                 (X, X) | (Y, Y) => Ordering::Equal,
///                 (X, Y) => Ordering::Less,
///                 (Y, X) => Ordering::Greater,
///             }
///         }
///     }
///
///     impl PartialOrd for Entity {
///         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///             Some(self.cmp(other))
///         }
///     }
///
///     // Sorts given entities using `Ord` implementation for `Entity` contained in this module
///     pub fn prioritize(entities: &mut [Entity]) {
///         entities.sort();
///     }
/// }
///
/// #[test]
/// #[should_fail]
/// fn priority_inversion() {
///     use crate::orphan::{model::Entity, module_a, module_b};
///
///     let mut entities = vec![Entity::X, Entity::Y];
///
///     module_a::prioritize(&mut entities);
///     assert_eq!(entities, vec![Entity::Y, Entity::X]);
///
///     module_b::prioritize(&mut entities);
///
///     // This would fail if orphan instances were allowed
///     //  - i.e. the behavior would depend on the *scope* rather than *types*
///     assert_eq!(entities, vec![Entity::Y, Entity::X]);
/// }
/// ```
pub struct OrphanInstanceTest;
