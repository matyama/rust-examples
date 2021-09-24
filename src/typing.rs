//! This module shows some examples of how Rust's type system can be used to write safe yet
//! efficient code.
//!
//! There are three implementations of the problem of implementing float comparison. The problem is
//! that according to the *IEEE* standard [f64] is only [PartialOrd] and not [Ord] since some
//! floats cannot be compared (e.g. *nan*).
//!
//! However, we can restrict our comparison to *positive floats*. The memory representation of each
//! positive float in IEEE standard format can be interpreted as [u32]. Integer representations of
//! positive floats are then *monotonic* and thus have efficient [Ordering].
//!
//! The three examples presented here then address the two remaining problems:
//!  1. How to ensure that given [f64] is positive
//!  1. How to *safely* compare floats when [f64::to_int_unchecked] is `unsafe`
//!
//! Note that one would typically realize these as implementations of [PartialOrd] or [Ord] but we
//! keep it simple and implement the comparison as plain function.
//!
//! Also note that `std` actually defines a safe memory interpretation [f64::to_bits] so this
//! example is somewhat artificial.
//!
//! Last few examples describe the [top](https://en.wikipedia.org/wiki/Top_type) and
//! [bottom](https://en.wikipedia.org/wiki/Bottom_type) type realized in Rust's type system.
use std::cmp::Ordering;

/// Naive *positive* [f64] comparison function.
///
/// This implementation first checks both arguments whether they are positive and *panics*
/// otherwise. After this check it is safe to call [f64::to_int_unchecked] and do the comparison on
/// [u32]s;
///
/// # Pros
/// 1. Clients don't have to check for anything on the call side
/// 1. This call plays nicely with other APIs
///
/// # Cons
/// 1. Call to this function might crash the program! Also clients must somehow know this (e.g.
///    read the docs)
/// 1. Each call adds a branching instruction which might get significant when called frequently
/// 1. Typical protection against #1 will be adding the same check and thus duplicating the
///    code
pub fn cmp_f64(a: f64, b: f64) -> Ordering {
    if !a.is_sign_positive() || !b.is_sign_positive() {
        panic!("Both a and b must be positive reals");
    }

    unsafe {
        let a: u32 = a.to_int_unchecked();
        let b: u32 = b.to_int_unchecked();
        a.cmp(&b)
    }
}

/// Improved comparison function for *positive* [f64]s.
///
/// This implementation is slightly better and more idiomatic version of the naive approach.
/// Instead of calling [panic!] when an argument is not positive, we return an
/// [`Option<Ordering>`](Option).
///
/// # Pros
/// 1. Clients are forced to handle None even though they know it's never returned
/// 1. Each call adds a branching instruction which might get significant when called frequently
/// 1. If this is called frequently, the initial check if sign is positive will soon get costly
///
/// # Cons
/// 1. Clients must handle the [`Option<Ordering>`](Option) and it might not integrate well with
///    other standard APIs
/// 1. For performance-critical applications it's quite unfortunate that this version double checks
///    `a` and `b` with an `if` to make sure the `unsafe` block is sound
pub fn better_cmp_f64(a: f64, b: f64) -> Option<Ordering> {
    if !a.is_sign_positive() || !b.is_sign_positive() {
        return None;
    }

    unsafe {
        let a: u32 = a.to_int_unchecked();
        let b: u32 = b.to_int_unchecked();
        Some(a.cmp(&b))
    }
}

/// Opaque wrapper around [f64] which adds static semantics that the float has positive value.
///
/// The benefit of such abstraction is two fold:
///  1. It adds static information about legal values that the compiler can't possibly know about
///  1. while it incurs zero cost at runtime (it gets "compiled away" and behaves as an ordinary
///     [f64])
///
/// While it might seem annoying to work with [Positive] instead of plain [f64], one can `derive`
/// many traits implemented for [f64] so that instances of [Positive] can be a drop-in replacement
/// for it in many cases.
///
/// Additionally, there is an incentive in the Rust team to further mitigate the pain to work with
/// these wrappers by introducing something like a `#[newtype_derive]` macro inspired by the
/// *newtype pattern* from Haskell.
///
/// Note this could be generalized with the [num crate](https://crates.io/crates/num):
/// ```ignore
/// use num::Float;
///
/// struct Positive<F: Float>(F);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Positive(f64);

impl Positive {
    /// This forces clients to always check if it's ok. One cannot initialize a tuple struct which
    /// contains private fields.
    ///
    /// So the following code **does not compile** because `f64` is a private field in [Positive].
    /// ```compile_fail
    /// use rust_examples::typing::Positive;
    ///
    /// let _ = Positive(-24.);
    /// ```
    /// The only option is then to use this factory method and therefore check the result.
    /// ```
    /// use rust_examples::typing::Positive;
    ///
    /// assert_eq!(Positive::new(-24.), None)
    /// ```
    pub fn new(number: f64) -> Option<Self> {
        if !number.is_sign_positive() {
            return None;
        }
        Some(Self(number))
    }

    /// Interprets [Positive] as an [u32]
    ///
    /// Note that this is not OOP, one can call [Positive::as_u32] as an ordinary function:
    /// ```
    /// use rust_examples::typing::Positive;
    ///
    /// let pos = Positive::new(42.).expect("positive number");
    /// let int = unsafe { Positive::as_u32(&pos) };
    /// assert_eq!(int, 42);
    /// ```
    ///
    /// # Safety
    /// The safety is guaranteed by the construction of [Positive] instances via [Positive::new].
    #[inline(always)]
    pub unsafe fn as_u32(&self) -> u32 {
        self.0.to_int_unchecked::<u32>()
    }
}

/// Safe and efficient version of comparison of two [Positive] floats.
///
/// This approach is a combination of the *make illegal states unrepresentable* and *fail early*
/// principles. Instead of having two [f64] arguments we enforce the user to pass in instances of
/// [Positive] which makes the comparison both trivial and safe.
///
/// # Pros
/// 1. Now his operation is completely safe even though it uses unsafe. It is not possible to
///    compile and run a program which calls this function with a negative number. This is a form of
///    formal validation done by the compiler and thus much stronger result than any (unit) test!
/// 1. The constructor of the wrapper type pushes the clients to check for errors early on
/// 1. The wrapper type carries certain semantics which can be taken to a benefit in the
///    implementation
///
/// # Cons
/// 1. Clients must wrap their data into the wrapper type which might get tedious and not worth it
///    for non-critical data flows (although, this might be mitigated in the future).
pub fn safe_cmp_f64(a: Positive, b: Positive) -> Ordering {
    // Rust's `unsafe` keyword is basically saying: Hey, Borrow Checker, I'm now resposible for the semantics of the memory management
    // in this block of code. Note that any crash in this scope results in an undefined behavior!
    unsafe { a.as_u32().cmp(&b.as_u32()) }
}

/// Structure that defines single field which has the type of the
/// [*top type*](https://en.wikipedia.org/wiki/Top_type) in Rust.
///
/// Because Rust's type system lacks central hierarchy and polymorphism is realized by (not only)
/// [bounded parametric polymorphism](https://en.wikipedia.org/wiki/Parametric_polymorphism), the
/// *top type* is actually the most generic type parameter - a type parameter which can be realized
/// by any type value.
///
/// One might think that the most generic type parameter is simply `<T>` but in Rust this
/// implicitly adds a bound that `T: Sized`. Therefore the most generic version is `<T: ?Sized>`
/// which relaxes this restriction.
pub struct TopTypeExample<T: ?Sized> {
    pub top_type: T,
}

/// This example presents several constructs which represent an empty set of values.
///
/// The, so far unstable, type [`!`](!) is Rust's version of the
/// [*bottom type*](https://en.wikipedia.org/wiki/Bottom_type).
///
/// # Example: Never type
/// The *never_type* [`!`](!) shown below cannot be instantiated as it represents no actual value.
/// ```compile_fail
/// struct Void {
///     bottom_type: !,
/// }
/// ```
///
/// # Example: Enum with no variants
/// Another example of a type which can't be instantiated is an `enum` with no variant.
/// ```
/// enum Void {}
/// ```
///
/// # Example: Declarative macros
/// There's a reason why *macros* end with the never type [`!`](!) - macros are language constructs
/// which are *replaced* by the code snippet they define at compilation time. Therefore there is no
/// actual value they represent by themself, just like [`!`](!).
/// ```
/// println!("Rust");
/// ```
///
/// # Example: Other constructs
/// There are other language constructs which result in no value. These are typically statements
/// like single branch `if` or `return` an `break`.
pub struct BottomTypeExample;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::greater(2., 1., Ordering::Greater)]
    #[case::less(1., 2., Ordering::Less)]
    #[case::equal(1., 1., Ordering::Equal)]
    #[should_panic]
    #[case::error(2., -1., Ordering::Greater)]
    #[should_panic]
    #[case::error(-1., 2., Ordering::Less)]
    fn safe_float_cmp(#[case] a: f64, #[case] b: f64, #[case] expected: Ordering) {
        let a = Positive::new(a).expect(&format!("a shold be a positive float, got {}", a));
        let b = Positive::new(b).expect(&format!("b shold be a positive float, got {}", b));
        assert_eq!(safe_cmp_f64(a, b), expected);
    }
}
