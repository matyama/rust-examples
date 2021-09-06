//! This module includes Rust's implementation of *traversable* types.
//!
//! [Traverse](https://typelevel.org/cats/typeclasses/traverse.html) is a *type class* which
//! threats an effect through a collection (or another wrapping context) and "turns the effect
//! inside out".
//!
//! Rust implements pattern via combination of `map` (to apply the effect) followed by `collect`
//! which is available for [std::iter::FromIterator] instances (in Scala Cats this is referred to
//! as `sequence`).
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};
use std::path::Path;

/// Traverse a collection of input values and apply an effect to item that may fail (take the
/// first character from each string slice, resulting in an [`Option<char>`](Option)).
///
/// Here the *magic* is the sequencing of these effects via `collect`. This is possible
/// because [std::iter::FromIterator] is implemented for the `Option` type, allowing for
/// the following transformations:
///  1. Applying an effect on each value of `Vec<&str>`, turning it into an
///     `Iterator<Item = Option<char>>`
///  2. Turning these effects "inside out" and collecting the items into `Option<Vec<char>>`
///
/// This efectively realizes [Traverse](https://typelevel.org/cats/typeclasses/traverse.html),
/// in this case for a [Vec] which is a 'Functor' (has a `map`) and an [Option] which is
/// 'Applicative Monad' (by the realization in Rust std lib).
pub fn collect_initials(names: Vec<&str>) -> Option<Vec<char>> {
    names.into_iter().map(first).collect()
}

fn first(input: &str) -> Option<char> {
    input.chars().next()
}

/// Traversing a [Result] works analogously to an [Option] since a result is basically an option
/// where the `None` case is some more specific type.
///
/// Other languages typically implement a general `.traverse(f)` method, so it might seem that
/// `collect` is not as expressive (it's equivalent to `.sequence`). However, `.traverse(f)` is
/// equivalent to `.map(f).sequence`. See the reference to *Typelevel Cats* for more.
///
/// Note that [std::io::Result] is just ordinary result with io error `Result<T, std::io::Error>`.
pub fn read_files<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<String>> {
    paths.iter().map(read_file).collect()
}
fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Result::Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use std::borrow::Borrow;
    use std::env;
    use std::path::PathBuf;

    struct TempFile(PathBuf);

    impl Drop for TempFile {
        fn drop(&mut self) {
            std::fs::remove_file(&self.0).unwrap_or(());
        }
    }

    impl AsRef<Path> for TempFile {
        fn as_ref(&self) -> &Path {
            &self.0
        }
    }

    // Based on https://github.com/la10736/rstest/blob/master/notes.md
    #[fixture]
    fn temp_file(#[default("test")] name: &str, #[default("")] text: &str) -> TempFile {
        let mut path = env::temp_dir();
        path.push(name);
        File::create(&path)
            .and_then(|mut fd| fd.write(text.as_bytes()))
            .expect("Failed to create temp file");
        TempFile(path)
    }

    #[rstest]
    fn traverse_options() {
        let success = collect_initials(vec!["Alice", "Bob", "Charlie"]);
        assert_eq!(success, Some(vec!['A', 'B', 'C']));

        // Second example demonstrates how the effects are sequenced. If the traversal finds single
        // value to be `None`, the whole result is `None`, otherwise all values are valid and can be
        // extracted and the result is `Some`.
        let failure = collect_initials(vec!["Martin", ""]);
        assert_eq!(failure, None);
    }

    #[rstest]
    fn traverse_results(
        #[from(temp_file)]
        #[with("test1", "some text")]
        tmp1: TempFile,
        #[from(temp_file)]
        #[with("test2", "other text")]
        tmp2: TempFile,
    ) {
        let success =
            read_files(&[tmp1.borrow(), tmp2.borrow()]).expect("This case should return Ok");
        assert_eq!(
            success,
            vec!["some text".to_string(), "other text".to_string()]
        );

        let non_existing = TempFile(PathBuf::from("non_existing_file"));
        let failure = read_files(&[tmp1, tmp2, non_existing]);
        assert!(failure.is_err());
    }

    #[rstest]
    fn build_non_linear_structure() {
        use std::collections::BinaryHeap;

        // As mentioned above, `collect` works for any implementation of the `FromIterator` trait.
        // This allows us to transform sequential data (an `Iterator`) into any data structure -
        // event non-linear one such as `BinaryHeap` (`BinaryHeap` implements `FromIterator`).
        let heap = vec![1, -2, 5, 4].into_iter().collect::<BinaryHeap<i32>>();
        assert_eq!(heap.into_iter().collect::<Vec<_>>(), vec![5, 4, 1, -2]);
    }
}
