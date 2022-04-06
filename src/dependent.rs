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
    /// Lower `Zero` from type to the term `0`.
    #[inline]
    fn lower() -> usize {
        0
    }
}

impl<N: Nat> Nat for Succ<N> {
    /// Lower `N` from a type-level to a term-level value.
    #[inline]
    fn lower() -> usize {
        N::lower() + 1
    }
}

/// Trait encoding a predecessor relation: "`Self` is a predecessor of `N`".
pub trait Pred<N> {}

/// [`Zero`](Zero) is a predecessor of [`Zero`](Zero).
impl Pred<Zero> for Zero {}

/// For each [`Succ<N: Nat>`](Succ) = N + 1 there is a predecessor `N: Nat`.
impl<N: Nat> Pred<Succ<N>> for N {}

/// Relation `M: AddEq<N, X>` is interpreted as `M + N = X`.
///
/// An obvious alternative is to define `trait EqAdd<M, N> {}` with the interpretaton:
///  > Relation `X: EqAdd<M, N>` is interpreted as `X = M + N`.
///
/// The problem with such an approach is in the specification of [Zero] being the *neutral
/// element*:
/// ```
/// # use rust_examples::dependent::{Nat, Succ, Zero};
/// # trait EqAdd<M, N> {}
/// impl<N: Nat> EqAdd<Zero, N> for N {}
/// ```
///
/// Since `N` here is generic, it would need *Specialization* for it not to overlap with the
/// specification of *associativity*:
/// ```
/// # use rust_examples::dependent::{Nat, Succ, Zero};
/// # trait EqAdd<M, N> {}
/// impl<M: Nat, N: Nat, X: Nat + EqAdd<M, N>> EqAdd<M, Succ<N>> for Succ<X> {}
/// ```
pub trait AddEq<N, X> {}

/// [Zero] is the *neutral element* of addition: `![N]: 0 + N = X = N`
impl<N: Nat> AddEq<N, N> for Zero {}

/// Addition is associative: `![N]: (M + 1) + N = (X + 1)` if `X = M + N` => `M: AddEq<N, X>`
impl<M, N, X> AddEq<N, Succ<X>> for Succ<M>
where
    M: Nat + AddEq<N, X>,
    N: Nat,
    X: Nat,
{
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

impl<N: Nat, A> Vector<N, A> {
    /// Add given element to the front of this [Vector].
    ///
    /// Notice that the length of the newly created vector is know to be one larger than the
    /// origitnal one at compilation time!
    pub fn cons(self, x: A) -> Vector<Succ<N>, A> {
        let mut xs = self.0;
        xs.insert(0, x);
        Vector(xs, PhantomData)
    }
}

/// Trait representing a heterogeneous list, a.k.a [HList] of length `N`.
///
/// Similarly to the simple example of [Vector], a `HList` also depends on its length `N`.
///
/// Contrary to an ordinary list, elements of an [HList] (resp. [HCons]) may vary in types.
///
/// # Example
/// ```
/// # use rust_examples::dependent::{HCons, HList, HNil};
/// // hlist = true : "two" : 1 : []
/// let hlist = HNil.cons(1).cons("two").cons(true);
/// assert_eq!(3, hlist.len());
/// ```
pub trait HList<N: Nat> {
    /// Add given element to the front of this [HList].
    fn cons<H>(self, x: H) -> HCons<Succ<N>, N, H, Self>
    where
        Self: Sized,
    {
        HCons(x, self, PhantomData, PhantomData)
    }

    /// Concatenate this and given `HList`.
    ///
    /// [`HList<M>`](HList) ++ [`HList<N>`](HList) = [`HList<M + N>`](HList)
    ///
    /// TODO: not implemented yet
    fn conctat<M, L, X, R>(self, _hlist: L) -> R
    where
        Self: Sized,
        M: Nat + AddEq<N, X>,
        L: HList<M>,
        X: Nat,
        R: HList<X>,
    {
        todo!("not implemented yet")
    }

    /// Analogy to [`Vec::len`](Vec::len).
    fn len(&self) -> usize {
        N::lower()
    }
}

/// Structure representing the null pointer at the end of each [HList].
///
/// Alternatively, [HNil] repreents an empty [HList].
pub struct HNil;

/// [HNil] is an empty [HList] (i.e. of length `N = 0`).
impl HList<Zero> for HNil {}

/// Structure representing a non-empty [HList] consisting of a head and tail.
///
/// `N` encodes the length of this [HList] while `M = N - 1` is the length of the tail [HList].
pub struct HCons<N, M, H, T>(
    /// Head of this [HList].
    pub H,
    /// Tail of this [HList] which itself is a [HList] of length `M = N - 1`.
    pub T,
    /// Evidence that this [HList] (resp. `HCons`) has length `N`.
    PhantomData<N>,
    /// Evidence that the tail [HList] in this `HCons` has length `M = N - 1`.
    PhantomData<M>,
);

impl<N: Nat, M: Nat + Pred<N>, H, T: HList<M>> HCons<N, M, H, T> {
    #[inline]
    pub fn new(h: H, t: T) -> Self {
        Self(h, t, PhantomData, PhantomData)
    }

    /// Take an immutable reference to the head element of this [HList] represented by [HCons].
    #[inline]
    pub fn head(&self) -> &H {
        &self.0
    }

    /// Take an immutable reference to the tail [HList] contained in this [HCons].
    #[inline]
    pub fn tail(&self) -> &T {
        &self.1
    }
}

/// [`HCons<N, M, _, T>`](HCons) is a `HList` of length `N > 0` if `T` is a `HList` of length
/// `M = N - 1`.
impl<N, M, H, T> HList<N> for HCons<N, M, H, T>
where
    N: Nat,
    M: Nat + Pred<N>, // M = N - 1
    T: HList<M>,      // T: Hlist<N - 1>
{
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
    fn nat_pred() {
        fn pred<M, N>() -> usize
        where
            N: Nat,
            M: Nat + Pred<N>,
        {
            M::lower()
        }

        assert_eq!(0, pred::<Zero, Zero>());
        assert_eq!(0, pred::<Zero, Succ<Zero>>());
        assert_eq!(1, pred::<Succ<Zero>, Succ<Succ<Zero>>>());
    }

    #[test]
    fn add_nats() {
        fn add<M, N, X>()
        where
            M: Nat + AddEq<N, X>, // M + N = X
            N: Nat,
            X: Nat,
        {
        }

        // 0 + 0 = 0
        add::<Zero, Zero, Zero>();
        // 0 + 1 = 1
        add::<Zero, Succ<Zero>, Succ<Zero>>();
        // 1 + 0 = 1
        add::<Succ<Zero>, Zero, Succ<Zero>>();
        // 1 + 1 = 2
        add::<Succ<Zero>, Succ<Zero>, Succ<Succ<Zero>>>();
        // 2 + 1 = 3
        add::<Succ<Succ<Zero>>, Succ<Zero>, Succ<Succ<Succ<Zero>>>>();
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

/// Negative compilation tests for [Pred] relation.
///
/// # 0 is not a predecessor of 2
/// ```compile_fail
/// # use rust_examples::dependent::{Nat, Pred, Succ, Zero};
/// # use rust_examples::dependent::NotPredTest;
/// NotPredTest::check::<Zero, Succ<Succ<Zero>>();
/// ```
///
/// # 1 is not a predecessor of 3
/// ```compile_fail
/// # use rust_examples::dependent::{Nat, Pred, Succ, Zero};
/// # use rust_examples::dependent::NotPredTest;
/// NotPredTest::check::<Succ<Zero>, Succ<Succ<Succ<Zero>>>>();
/// ```
///
/// # 1 is not a predecessor of 0
/// ```compile_fail
/// # use rust_examples::dependent::{Nat, Pred, Succ, Zero};
/// # use rust_examples::dependent::NotPredTest;
/// NotPredTest::check::<Succ<Zero>, Zero>;
/// ```
///
/// # 2 is not a predecessor of 1
/// ```compile_fail
/// # use rust_examples::dependent::{Nat, Pred, Succ, Zero};
/// # use rust_examples::dependent::NotPredTest;
/// NotPredTest::check::<Succ<Succ<Zero>>, Succ<Zero>>;
/// ```
pub struct NotPredTest;

impl NotPredTest {
    pub fn check<M, N>()
    where
        N: Nat,
        M: Nat + Pred<N>,
    {
    }
}

/// Negative compilation tests for [AddEq] relation.
///
/// # `0 + 1 != 0`
/// ```compile_fail
/// # use rust_examples::dependent::*;
/// NotAddTest::check::<Zero, Succ<Zero>, Zero>;
/// ```
/// # `1 + 1 != 3`
/// ```compile_fail
/// # use rust_examples::dependent::*;
/// NotAddTest::check::<Succ<Zero>, Succ<Zero>, Succ<Succ<Succ<Zero>>>>;
/// ```
pub struct NotAddTest;

impl NotAddTest {
    pub fn check<M, N, X>()
    where
        M: Nat + AddEq<N, X>,
        N: Nat,
        X: Nat,
    {
    }
}
