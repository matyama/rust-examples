/// Declarative macro that **substitutes** call side with one of the expressions the head of which
/// matches the call pattern.
///
/// Contrary to passing arguments to a function, `$x` is here directly substituted for the calling
/// expression. Moreover, the *types* are not actualt Rust types but rather tokens in Rust syntax -
/// in the case of `$x:expr` it's an arbitrary *expression*.
///
/// Note that the output must be a single expression, so if one wants to use multiple statements,
/// there has to be additional enclosing scope `{}`.
#[macro_export]
macro_rules! approx {
    // One can define any pattern that conforms with Rust's grammar (in this case expression)
    ($x:expr, $y:expr; $eps:expr) => {
        ($x as f64 - $y as f64).abs() < $eps
    };
    ($x:expr, $y:expr) => {
        // It's ok to call other macros (or even the same in this case)
        approx!($x, $y; f64::EPSILON)
    };
}

pub trait MaxValue {
    fn max_value() -> Self;
}

/// One very useful and common use case is to automate trait implementations. In this example we
/// create a *template* implementation for [MaxValue] for an arbitrary type `$t`.
///
/// Note that macros are compile-time safe because they just substitute actual pieces of code at
/// compile time and because they are hygienic (don't allow to use data from the call side scope).
///
/// In this case, any call of this macro with a type that does not define `::MAX` would be rejected
/// at compile time.
#[macro_export]
macro_rules! impl_max_value {
    // One can use regex-like patterns to match variadic arguments. Here we match on one or more
    // types separated by comma. The body can then use `$(...)+` to loop over each item.
    ($($t:ty),+) => {
        $(
            impl $crate::macros::MaxValue for $t {
                fn max_value() -> Self {
                    <$t>::MAX
                }
            }
        )+
    };
}

// This is the where we actually create all the `impl`s
impl_max_value!(u32, i32, u64, i64);

/// Macro that counts any input tokens at compilation time (i.e. resulting in a `const` value) with
/// no residual memory footprint.
/// ```
/// use rust_examples::{count, substitute};
///
/// const COUNT: usize = count!(1, 2, 3);
/// ```
///
/// See the [book about macros](https://danielkeep.github.io/tlborm/book/blk-counting.html) for
/// more and [this video](https://www.youtube.com/watch?v=q6paRBbLgNw&t=4380s) for implementation
/// details.
#[macro_export]
macro_rules! count {
    ($($item:tt),*) => {
        <[()]>::len(&[$(substitute!($item ())),*])
    };
}

#[macro_export]
macro_rules! substitute {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_use]
#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    #[case(0.0, 0)]
    #[case(1.0, 1)]
    fn approx_eq(#[case] x: f64, #[case] y: u32) {
        assert!(approx!(x, y))
    }

    #[rstest]
    fn max_values() {
        assert_eq!(u32::max_value(), u32::MAX);
        assert_eq!(i32::max_value(), i32::MAX);
        assert_eq!(u64::max_value(), u64::MAX);
        assert_eq!(i64::max_value(), i64::MAX);
    }

    #[rstest]
    fn count_items() {
        assert_eq!(count!(), 0);
        assert_eq!(count!(1), 1);
        assert_eq!(count!([1, 2], [], [0, 1, 3]), 3);
    }
}
