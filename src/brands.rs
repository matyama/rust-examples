//! This module demonstrates *branded types* on an example of a [Vec] with *unchecked-indexing*.
//!
//! The concept of *unchecked-indexing* means that the API of this vector is constructed in such a
//! way that index bounds check is performed statically at compile time. This is achieved via
//! lifetimes and there is no cost at runtime.
//!
//! The example is taken from the [GhostCell paper](http://plv.mpi-sws.org/rustbelt/ghostcell/).
use std::marker::PhantomData;

/// Lifetime wrapper which makes `'id` *invariant* and has no size.
///
/// The invariance of `'id` (resp. [InvariantLifetime]) comes from the underlying raw pointer type
/// `*mut &'id ()` - see [the docs](https://doc.rust-lang.org/reference/subtyping.html#variance).
///
/// Zero size is achieved via the usage of opaque type [PhantomData] which only carries the type
/// information until the compilation and then is "compiled away".
///
/// Note that the *invariance* of `'id` (resp. [InvariantLifetime]) means that one cannot change
/// the brand (`'id`) of [BrandedIndex] or [BrandedVec] via
/// [*subtyping*](https://doc.rust-lang.org/reference/subtyping.html).
///
/// # Example: Size Test
/// ```
/// use rust_examples::brands::InvariantLifetime;
///
/// assert_eq!(std::mem::size_of::<InvariantLifetime<'_>>(), std::mem::size_of::<()>());
/// ```
#[derive(Default, Clone, Copy)]
pub struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

/// Thin wrapper for [usize] which is bound to particular instance of a [BrandedVec] via `'id` and
/// serves as an access token to the interior values.
#[derive(Clone, Copy)]
pub struct BrandedIndex<'id> {
    idx: usize,
    _marker: InvariantLifetime<'id>,
}

/// Thin wrapper for [Vec] which can only be accessed via an associated [BrandedIndex]
pub struct BrandedVec<'id, T> {
    inner: Vec<T>,
    _marker: InvariantLifetime<'id>,
}

/// Public API of the [BrandedVec] as presented in the
/// [GhostCell paper](http://plv.mpi-sws.org/rustbelt/ghostcell/).
///
/// # Summary
/// This implementation defines a vector which
///  1. has static bounds check (i.e. no additional index check in [`get`](BrandedVec::get) and
///     [`get_mut`](BrandedVec::get_mut))
///  1. is monotonic (i.e. can only be appened to via [`push`](BrandedVec::push))
///  1. iterior pointers can only be accessed via associated [BrandedIndex]
///  1. is a *zero-cost abstraction* over a [Vec]
///
/// # Example 1
/// ```
/// use rust_examples::brands::BrandedVec;
///
/// let vec1 = vec![10, 11];
/// let vec2 = vec![20, 21];
///
/// BrandedVec::make(vec1, move |mut bvec1| {
///     bvec1.push(12);
///     let i1 = bvec1.push(13);
///
///     BrandedVec::make(vec2, move |mut bvec2| {
///         let i2 = bvec2.push(22);
///         *bvec2.get_mut(i2) -= 1;
///
///         assert_eq!(bvec1.get(i1), &13);
///         assert_eq!(bvec2.get(i2), &21);
///     });
/// });
/// ```
///
/// # Example 2
/// This example demonstrates that [BrandedIndex] is always bound to the [BrandedVec] for which it
/// was originally created and as such cannot be used for another [BrandedVec].
/// ```compile_fail
/// use rust_examples::brands::BrandedVec;
///
/// let vec1 = vec![10, 11];
/// let vec2 = vec![20, 21];
///
/// BrandedVec::make(vec1, move |mut bvec1| {
///     let i1 = bvec1.push(12);
///
///     BrandedVec::make(vec2, move |mut bvec2| {
///         // Can't use index that was branded for `bvec1`
///         bvec2.get(i1)
///     });
/// });
///
/// ```
///
/// # Example 3
/// Finally, [BrandedVec] is a *zero-cost abstraction* for ordinary [Vec].
/// ```
/// use std::mem::size_of;
/// use rust_examples::brands::BrandedVec;
///
/// assert_eq!(size_of::<BrandedVec<'_, u8>>(), size_of::<Vec<u8>>());
/// ```
impl<'id, T> BrandedVec<'id, T> {
    /// Construct new [BrandedVec] from given [Vec] and run a closure `f` with it.
    ///
    /// `for<'a>` here is an example of *rank-2 polymorphism*, meaning that the closure given
    /// by `f` must be valid for any choice of `'a`, not just `'id`.
    ///
    /// So `make` here is allowed to "pick" a fresh lifetime `'id` for each new [BrandedVec] but
    /// the closure `f`, when it receives this branded vector, must treat the `'id` brand opaquely.
    pub fn make<R>(inner: Vec<T>, f: impl for<'a> FnOnce(BrandedVec<'a, T>) -> R) -> R {
        f(Self {
            inner,
            _marker: InvariantLifetime::default(),
        })
    }

    /// Appends given `value` to this [BrandedVec] and returns [BrandedIndex] of this item which is
    /// bound to `self`. Since [BrandedVec] can only be appended to, this [BrandedIndex] is
    /// guaranteed to *always* be within bounds.
    pub fn push(&mut self, value: T) -> BrandedIndex<'id> {
        let idx = self.inner.len();
        self.inner.push(value);
        BrandedIndex {
            idx,
            _marker: self._marker,
        }
    }

    /// Method which associates (brands) given `idx` with this [BrandedVec] if it is within bounds
    /// - i.e. performs the bounds check which `Vec::get` does.
    pub fn get_index(&self, idx: usize) -> Option<BrandedIndex<'id>> {
        if idx < self.inner.len() {
            Some(BrandedIndex {
                idx,
                _marker: self._marker,
            })
        } else {
            None
        }
    }

    /// Get shared reference to the interior value at given [BrandedIndex] without performing a
    /// bounds check.
    pub fn get(&self, index: BrandedIndex<'id>) -> &T {
        // Safety: By the construction of `BrandedIndex` and from the fact that `BrandedVec` can
        // only be appended to (`BrandedIndex` is monotonic)
        unsafe { self.inner.get_unchecked(index.idx) }
    }

    /// Get mutable reference to the interior value at given [BrandedIndex] without performing a
    /// bounds check.
    pub fn get_mut(&mut self, index: BrandedIndex<'id>) -> &mut T {
        // Safety: By the construction of `BrandedIndex` and from the fact that `BrandedVec` can
        // only be appended to (`BrandedIndex` is monotonic)
        unsafe { self.inner.get_unchecked_mut(index.idx) }
    }
}
