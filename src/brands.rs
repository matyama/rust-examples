use std::marker::PhantomData;

#[derive(Default, Clone, Copy)]
pub struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

#[derive(Clone, Copy)]
pub struct BrandedIndex<'id> {
    idx: usize,
    _marker: InvariantLifetime<'id>,
}

pub struct BrandedVec<'id, T> {
    inner: Vec<T>,
    _marker: InvariantLifetime<'id>,
}

/// Public API of the [BrandedVec] as presented in the
/// [GhostCell paper](http://plv.mpi-sws.org/rustbelt/ghostcell/).
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
/// Finally, [BrandedVec] is a *zero-cost abstraction* for ordinary [Vec].
/// ```
/// use std::mem::size_of;
/// use rust_examples::brands::BrandedVec;
///
/// assert_eq!(size_of::<BrandedVec<'_, u8>>(), size_of::<Vec<u8>>());
/// ```
impl<'id, T> BrandedVec<'id, T> {
    pub fn make<R>(inner: Vec<T>, f: impl for<'a> FnOnce(BrandedVec<'a, T>) -> R) -> R {
        f(Self {
            inner,
            _marker: InvariantLifetime::default(),
        })
    }

    pub fn push(&mut self, value: T) -> BrandedIndex<'id> {
        let idx = self.inner.len();
        self.inner.push(value);
        BrandedIndex {
            idx,
            _marker: InvariantLifetime::default(),
        }
    }

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

    pub fn get(&self, index: BrandedIndex<'id>) -> &T {
        unsafe { self.inner.get_unchecked(index.idx) }
    }

    pub fn get_mut(&mut self, index: BrandedIndex<'id>) -> &mut T {
        unsafe { self.inner.get_unchecked_mut(index.idx) }
    }
}
