//! This module includes Rust's approach to *dependent types*.
//!
//! Informally, a dependent type is a type that depends of a value of (or a term from) another
//! type.
//!
//! This example if from a talk (not only) about dependent types (~ 15:50 min):
//! [Proof Theory Impressionism: Blurring the Curry-Howard Line](https://youtu.be/jrVPB-Ad5Gc)
//!
//! More generally, according to the article *Why dependent types matter*:
//! > Dependently typed programs are, by their nature, proof carrying code.

#![allow(clippy::len_without_is_empty)]
#![allow(clippy::new_without_default)]

use std::marker::PhantomData;

/// Trait representing a type-level definition of natural numbers (Peano numbers).
pub trait Nat {
    /// Lowers a natural number from a type level value (type) to corresponding term-level value.
    fn lower() -> usize;
}

/// Type-level definition of the natural number 0.
pub struct Zero;

/// Type-level definition of a successor of a natural number `N`, i.e. the number `N + 1`.
pub struct Succ<N: Nat>(PhantomData<N>);

impl Nat for Zero {
    fn lower() -> usize {
        0
    }
}

impl<N: Nat> Nat for Succ<N> {
    fn lower() -> usize {
        N::lower() + 1
    }
}

/// A wrapper for [`Vec<A>`](Vec) which preserves the information about its size `N` at the type
/// level (i.e. compilation time).
///
/// In other words, the type of this vector is dependent on its size.
pub struct Vector<N: Nat, A>(Vec<A>, PhantomData<N>);

impl<A> Vector<Zero, A> {
    /// Create new [Zero]-sized vector.
    pub fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
}

impl<N: Nat, A> Vector<N, A> {
    /// Static information about the size of a [Vector].
    pub fn size() -> usize {
        N::lower()
    }

    /// Analogy to the [`Vec::len`](Vec::len) which takes advantage from
    /// [`Vector::size`](Vector::size).
    pub fn len(&self) -> usize {
        Self::size()
    }
}

impl<N: Nat, A: Copy> Vector<N, A> {
    /// Add given element to the front of this [Vector].
    ///
    /// Notice that the length of the newly created vector is know to be one larger than the
    /// origitnal one at compilation time!
    pub fn cons(&self, x: A) -> Vector<Succ<N>, A> {
        // This is not quite functional, but that's not the point of this example.
        let mut xs: Vec<A> = self.0.iter().copied().collect();
        xs.insert(0, x);
        Vector(xs, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_nats() {
        // Instances of 0 and 1 from the inductive definition of natural numbers (Peano numbers)
        let _zero = Zero;
        let _one = Succ::<Zero>;
    }

    #[test]
    fn sized_vec() {
        let v = Vector::<Zero, u8>::new();
        let v_prime = v.cons(42);
        assert_eq!(1, v_prime.len());
    }
}
