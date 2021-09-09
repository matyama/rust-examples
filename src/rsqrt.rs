//! Example of [Fast inverse square root](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
use std::cmp::max;

const THREE_HALFS: f32 = 1.5;

/// Approximates the inverse square root of given number.
///
/// Note that this a port of the original *C* implementation and as such it is generally *unsafe*.
/// Calling this fuction with a negative [f32], zero, nan or infinity will result in a panic (for
/// these is the *inverse square root* undefined).
pub fn rsqrt(number: f32) -> f32 {
    let x2 = number * 0.5;
    let mut y = number;

    // evil floating point bit level hacking
    let i = y.to_bits();

    // what the fuck?
    let i = 0x5f3759df - (i >> 1);

    y = f32::from_bits(i);

    // 1st iteration
    y = y * (THREE_HALFS - (x2 * y * y));

    // 2nd iteration, this can be removed
    // y = y * (THREE_HALFS - (x2 * y * y));

    y
}

/// Thin wrapper around [f32] with additional semantics that the values can only be positive floats
/// and excluding infinity and nan.
///
/// # Zero-cost abstraction
/// ```
/// use std::mem::size_of;
/// use rust_examples::rsqrt::PositiveFloat;
///
/// assert_eq!(size_of::<PositiveFloat>(), size_of::<f32>());
/// ```
#[derive(Debug, PartialEq)]
pub struct PositiveFloat(f32);

impl PositiveFloat {
    /// Constructs new [PositiveFloat] from given [f32] only if:
    ///  * it is sign positive
    ///  * is a *normal* float value (i.e. not a zero, nan or infinity)
    ///
    /// Note that this factory ensures the safety of [PositiveFloat::fast_rsqrt] as it makes the
    /// illegal states mentioned above *unrepresentable*.
    ///
    /// # Example
    /// ```
    /// use rust_examples::rsqrt::PositiveFloat;
    ///
    /// assert_eq!(PositiveFloat::new(-4.2), None);
    /// assert_eq!(PositiveFloat::new(0.0), None);
    /// assert_eq!(PositiveFloat::new(f32::NAN), None);
    /// assert_eq!(PositiveFloat::new(f32::INFINITY), None);
    /// ```
    #[inline]
    pub fn new(v: f32) -> Option<Self> {
        if v.is_sign_positive() && v.is_normal() {
            Some(Self(v))
        } else {
            None
        }
    }

    /// Retrieves inner [f32] value
    #[inline]
    pub fn inner(&self) -> f32 {
        self.0
    }

    /// Calculates the inverse square root of given number
    #[inline]
    pub fn rsqrt(&self) -> Self {
        // This is safe because `x -> 1 / sqrt(x)` is known to be positive
        Self(self.0.sqrt().recip())
    }

    /// Approximates the inverse square root of given number.
    ///
    /// This implementation is safe in the sense that by the construction of [PositiveFloat], it is
    /// not possible to call [PositiveFloat::fast_rsqrt] on any invalid value: negative floats,
    /// zero, nan or infinity.
    ///
    /// Constant generic parameter `ITERS` determines the number of iterations of the Newton's
    /// method used to find the approximation. Note that despite the fact that it is [usize], the
    /// implementation executes at least one iteration even if it is set to `0`.
    pub fn fast_rsqrt<const ITERS: usize>(&self) -> Self {
        let x2 = self.0 * 0.5;
        let i = self.0.to_bits();
        let mut y = f32::from_bits(0x5f3759df - (i >> 1));

        // Newton's method (at least one iteration)
        for _ in 0..max(ITERS, 1) {
            y *= THREE_HALFS - (x2 * y * y);
        }

        // This is safe because `x -> 1 / sqrt(x)` is known to be positive
        Self(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::approx;
    use quickcheck::TestResult;
    use rstest::*;

    const EPS: f64 = 0.005;

    #[rstest]
    #[case::nan(f32::NAN, None)]
    #[case::inf(f32::INFINITY, None)]
    #[case::zero(0.0, None)]
    #[case::neg(-1.0, None)]
    #[case::pos(4.2, Some(PositiveFloat(4.2)))]
    fn positive_float(#[case] number: f32, #[case] expected: Option<PositiveFloat>) {
        assert_eq!(PositiveFloat::new(number), expected);
    }

    #[rstest]
    fn wikipedia_example() {
        let estimate = rsqrt(0.15625);
        let target = 2.52982;
        assert!(
            approx!(estimate, 2.52982; EPS),
            "{} is not within {} of {}",
            estimate,
            EPS,
            target
        );
    }

    #[quickcheck]
    fn rsqrt_approximates_inverse_square_root(number: f32) -> TestResult {
        if number.is_sign_negative() || !number.is_normal() {
            return TestResult::discard();
        }

        let estimate = rsqrt(number);
        let target = number.sqrt().recip();
        let close_approx = approx!(estimate, target; EPS);

        TestResult::from_bool(close_approx)
    }

    #[quickcheck]
    fn safe_rsqrt_approximates_inverse_square_root(number: f32) -> TestResult {
        if let Some(number) = PositiveFloat::new(number) {
            let estimate = number.fast_rsqrt::<1>().inner();
            let target = number.rsqrt().inner();
            let close_approx = approx!(estimate, target; EPS);

            TestResult::from_bool(close_approx)
        } else {
            TestResult::discard()
        }
    }
}
