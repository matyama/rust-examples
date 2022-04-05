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
//!
//! Finally, example of an [HList] is taken from
//! [hlist-rs](https://github.com/plausiblelabs/hlist-rs).

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
        let mut xs = self.0.to_vec();
        xs.insert(0, x);
        Vector(xs, PhantomData)
    }
}

/// Trait representing a heterogeneous list, a.k.a [HList].
///
/// Contrary to an ordinary list, elements of an [HList] (resp. [HCons]) may vary in types.
pub trait HList {
    /// Add given element to the front of this [HList].
    fn cons<H>(self, x: H) -> HCons<H, Self>
    where
        Self: Sized,
    {
        HCons(x, self)
    }

    /// Analogy to [`Vec::len`](Vec::len).
    fn len(&self) -> usize;
}

/// Structure representing the null pointer at the end of each [HList].
///
/// Alternatively, [HNil] repreents an empty [HList].
pub struct HNil;

impl HList for HNil {
    #[inline]
    fn len(&self) -> usize {
        0
    }
}

/// Structure representing a non-empty [HList] consisting of a head and tail.
pub struct HCons<H, T: HList>(pub H, pub T);

impl<H, T: HList> HCons<H, T> {
    #[inline]
    pub fn head(&self) -> &H {
        &self.0
    }

    #[inline]
    pub fn tail(&self) -> &T {
        &self.1
    }
}

impl<H, T: HList> HList for HCons<H, T> {
    #[inline]
    fn len(&self) -> usize {
        1 + self.tail().len()
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

    #[test]
    fn make_hlist() {
        let hlist = HNil;
        assert_eq!(0, hlist.len());

        let hlist = hlist.cons(1).cons("two").cons(true);
        assert_eq!(3, hlist.len());
    }
}
