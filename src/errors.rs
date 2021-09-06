//! This module presents several possibilities of how to handle errors in Rust.
//!
//! In general, there are two
//! [kinds of errors](https://doc.rust-lang.org/book/ch09-00-error-handling.html):
//!  1. *Recoverable errors* - those are contexts in which there is a possibility to react and
//!     correct the situation (e.g. by substituing default value or calling error handler)
//!  1. *Unrecoverable errors* - these are unexpected situations which cannot be handled by an
//!     application or library resulting in a crash of the program
//!
//! Naturally, Rust promotes types like [Option] and [Result] to mitigate the amount of possible
//! non-recoverable situations.

/// This function computes `num / d` in a *naive* way that causes the program to *panic* if `d = 0`
pub fn naive_div(num: i32, d: i32) -> i32 {
    num / d
}

/// This version of division models the error case (`d = 0`) by an [Option].
///
/// An option is a context in which a value might be present ([`Some`](Option::Some) variant) or
/// missing ([`None`](Option::None) variant). Rust std library's API implements an [Option] as a
/// [Monad](https://en.wikipedia.org/wiki/Monad_(functional_programming)).
///
/// Note that std already contians [i32::checked_div] which does the same thing as this function.
pub fn maybe_div(num: i32, d: i32) -> Option<i32> {
    if d != 0 {
        Some(num / d)
    } else {
        None
    }
}

/// This version of division is somewhat artificial but demonstrates another error context, the
/// [Result].
///
/// [Result] is like an [Option] as it represents an error effect but with the error case being an
/// explicit type (i.e. more informative than plain [`None`](Option::None)). Note that it is also
/// modeled as a *Monad*.
///
/// In this example we get the divisor `d` as a string slice so we have to parse it first.
/// Therefore there can be three resulting cases:
///  1. a [`Ok`](Result::Ok) result when parsing succeeds and `d` is non-zer
///  1. an [`Err`](Result::Err) indicating that `d` was zero
///  1. finally a different [`Err`](Result::Err) when parsing fails
///
/// Note that here we use simple [String] to differentiate the error cases but once would typically
/// use custom `enum` with errors as variants.
pub fn explained_div(num: i32, d: &str) -> Result<i32, String> {
    match d.parse::<i32>() {
        Ok(div) if div != 0 => Ok(num / div),
        Ok(_) => Err("Division by zero!".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[should_panic]
    fn naive_div_by_zero() {
        // This call will panic because we divide by zero!
        naive_div(42, 0);
    }

    #[rstest]
    // In normal situations we get some result
    #[case::some(42, 2, Some(21))]
    // However, division by zero yields a None value.
    // Note that this is not a 'null' pointer, rather a regular value of the `Option` type.
    //
    // `Option` is an example of a recoverable error. It is a *context* in which a value might
    // be missing.
    #[case::none(42, 0, None)]
    fn maybe_div_works(#[case] num: i32, #[case] d: i32, #[case] expected: Option<i32>) {
        assert_eq!(maybe_div(num, d), expected);
    }

    #[rstest]
    // In standard case we get an Ok result
    #[case::ok(42, "2", Ok(21))]
    // In other cases (parting error or division by zero) we get an Err.
    // Built-in `Result` is another example of a type for handling recoverable errors. It can
    // be viewd as an `Option` with an information `Err` about the failure.
    #[case::err_zero_div(42, "0", Err("Division by zero!"))]
    #[case::err_parse(42, "abcdefg", Err("invalid digit found in string"))]
    fn explained_div_works(#[case] num: i32, #[case] d: &str, #[case] expected: Result<i32, &str>) {
        assert_eq!(explained_div(num, d), expected.map_err(String::from));
    }
}
