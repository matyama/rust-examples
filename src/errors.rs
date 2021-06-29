pub fn naive_div(num: i32, d: i32) -> i32 {
    num / d
}

pub fn maybe_div(num: i32, d: i32) -> Option<i32> {
    // Note that std already contians `i32::checked_div`
    if d != 0 {
        Some(num / d)
    } else {
        None
    }
}

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

    #[test]
    #[should_panic]
    fn naive_div_by_zero() {
        // This call will panic because we divide by zero!
        naive_div(42, 0);
    }

    #[test]
    fn maybe_div_works() {
        // In normal situations we get some result
        assert_eq!(maybe_div(42, 2), Some(21));

        // However, division by zero yields a None value.
        // Note that this is not a 'null' pointer, rather a regular value of the `Option` type.
        //
        // `Option` is an example of a recoverable error. It is a *context* in which a value might
        // be missing.
        assert_eq!(maybe_div(42, 0), None);
    }

    #[test]
    fn explained_div_works() {
        // In standard case we get an Ok result
        assert_eq!(explained_div(42, &"2"), Ok(21));

        // In other cases (parting error or division by zero) we get an Err.
        // Built-in `Result` is another example of a type for handling recoverable errors. It can
        // be viewd as an `Option` with an information `Err` about the failure.

        assert_eq!(
            explained_div(42, &"0"),
            Err("Division by zero!".to_string())
        );

        assert_eq!(
            explained_div(42, &"abcdefg"),
            Err("invalid digit found in string".to_string())
        );
    }
}
