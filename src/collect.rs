pub fn first(input: &str) -> Option<char> {
    input.chars().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Traverse a collection of input values and applying an effect (taking the first character
    /// from each string slice) that may fail (resulting in an `Option<char>`).
    ///
    /// Here the 'magic' is the sequencing of these effects via `collect`. This is possible
    /// because [std::iter::FromIterator] is implemented for the `Option` type, allowing for
    /// the following transformations:
    ///  1. Applying an effect on each value of `Vec<&str>`, turning it into an
    ///     `Iterator<Item = Option<char>>`
    ///  2. Turning these effects "inside out" and collecting the items into `Option<Vec<char>`
    ///
    /// Second example demonstrates how the effects are sequenced. If the traversal finds single
    /// value to be `None`, the whole result is `None`, otherwise all values are valid and can be
    /// extracted and the result is `Some`.
    ///
    /// This efectively realizes [Traverse](https://typelevel.org/cats/typeclasses/traverse.html),
    /// in this case for a [Vec] which is a 'Functor' (has a `map`) and an [Option] which is
    /// 'Applicative Monad' (by the realization in Rust std lib).
    #[test]
    fn traversing_options() {
        let success = vec!["one", "key"]
            .into_iter()
            .map(first)
            .collect::<Option<Vec<_>>>();

        assert_eq!(success, Some(vec!['o', 'k']));

        let failure = vec!["fail", ""]
            .into_iter()
            .map(first)
            .collect::<Option<Vec<_>>>();

        assert_eq!(failure, None);
    }

    // TODO: Result<T, E>
}
