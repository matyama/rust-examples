//! Example of [Fast inverse square root](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
use derive_more::{Add, Mul};
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
/// # Automatic derivation or arithmetic operators
/// Notice that some of impls on [PositiveFloat] are automatically derived using
/// [`derive_more`](https://crates.io/crates/derive_more).
#[derive(Clone, Copy, Debug, Add, Mul, PartialEq)]
#[mul(forward)]
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

    #[inline]
    pub fn from_square(x: f32) -> Self {
        Self(x * x)
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

/// Type alias for 3D vector represented as 3-tuple of [f32]
pub type Vec3D = (f32, f32, f32);

/// This trait is a typeclass for all vectors that are normalized via a fast (approximate) inverse
/// square root
pub trait FastNormalize {
    /// Normalization return type
    type NormVec;

    /// Normalize this vector and return [`NormVec`](Self::NormVec)
    fn normalize(&self) -> Self::NormVec;
}

/// This implementation of [FastNormalize] uses a [`fast_rsqrt<1>`](PositiveFloat::fast_rsqrt) to
/// compute compute the invese square root of `x^2 + y^2 + z^2` of the components of the initial
/// vector.
///
/// Because the components must be converted to [PositiveFloat]s, additional checks are
/// necessary to validate the inputs. Moreover, because the validation might fail, the return type
/// must be wrapped into an [Option].
impl FastNormalize for Vec3D {
    type NormVec = Option<Vec3D>;

    fn normalize(&self) -> Self::NormVec {
        let &(x, y, z) = self;

        if !x.is_normal() || !y.is_normal() || !z.is_normal() {
            return None;
        }

        let squares_sum = PositiveFloat::from_square(x)
            + PositiveFloat::from_square(y)
            + PositiveFloat::from_square(z);

        let recip_norm = squares_sum.fast_rsqrt::<1>().inner();

        Some((x * recip_norm, y * recip_norm, z * recip_norm))
    }
}

/// Type that represents *normal* [f32] numbers. This excludes numbers that are
///  - NaN
///  - Infinite
///  - Zero
///  - Subnormal
pub struct Float(f32);

impl Float {
    /// Constructs [Float] only if `v` is *normal*
    #[inline]
    pub fn new(v: f32) -> Option<Self> {
        if v.is_normal() {
            Some(Self(v))
        } else {
            None
        }
    }

    /// Computes the square of the inner value of `self` and returns it as a [PositiveFloat]
    #[inline]
    pub fn square(&self) -> PositiveFloat {
        PositiveFloat::from_square(self.0)
    }
}

/// Optimized implementation of [FastNormalize] for 3D vector of [Float]s.
///
/// In this implementation we know that [Float] is non-zero, not nan and not infinity, so the
/// squares are always [PositiveFloat]s. Therefore we can skip the checks on the components of the
/// input vector.
impl FastNormalize for (Float, Float, Float) {
    type NormVec = Self;

    fn normalize(&self) -> Self::NormVec {
        let (x, y, z) = self;
        let recip_norm = (x.square() + y.square() + z.square())
            .fast_rsqrt::<1>()
            .inner();
        (
            Float(x.0 * recip_norm),
            Float(y.0 * recip_norm),
            Float(z.0 * recip_norm),
        )
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
    #[case::neg_inf(f32::NEG_INFINITY, None)]
    #[case::zero(0.0, None)]
    #[case::neg(-1.0, None)]
    #[case::one(1.0, Some(PositiveFloat(1.0)))]
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

    #[rstest]
    #[case(1.0, 1.0, 1.0, true)]
    #[case(1.0, 2.0, 3.0, true)]
    #[case(4.2, -1.0, -1.0, true)]
    #[case(0.0, 0.0, 0.0, false)]
    #[case(1.0, f32::INFINITY, 2.0, false)]
    #[case(1.0, f32::NEG_INFINITY, 2.0, false)]
    #[case(1.0, f32::NAN, 2.0, false)]
    fn fast_normalization(#[case] x: f32, #[case] y: f32, #[case] z: f32, #[case] some: bool) {
        let v_norm = (x, y, z).normalize();

        assert_eq!(v_norm.is_some(), some);

        if let Some((x, y, z)) = v_norm {
            // Compute and check conventional norm
            let norm = (x * x + y * y + z * z).sqrt();

            assert!(
                approx!(norm, 1.0; EPS),
                "Norm should be approx. one, got: {}",
                norm
            );
        }
    }

    #[rstest]
    #[case(Float(1.0), Float(1.0), Float(1.0))]
    #[case(Float(1.0), Float(2.0), Float(3.0))]
    #[case(Float(4.2), Float(-1.0), Float(-1.0))]
    fn fast_safe_normalization(#[case] x: Float, #[case] y: Float, #[case] z: Float) {
        let (x, y, z) = (x, y, z).normalize();

        // Compute and check conventional norm
        let norm = (x.square() + y.square() + z.square()).inner().sqrt();

        assert!(
            approx!(norm, 1.0; EPS),
            "Norm should be approx. one, got: {}",
            norm
        );
    }
}
