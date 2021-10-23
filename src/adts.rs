//! This module contains an example of
//! [Algebraic Data Type (ADT)](https://en.wikipedia.org/wiki/Algebraic_data_type) and the
//! concept of [pattern matching](https://en.wikipedia.org/wiki/Pattern_matching) which is commonly
//! used to work with ADTs.

/// An enum representing an Binary Tree Algebraic Data Type (ADT)
///
/// This enum defined two distinct types (variants), each of different shape and size:
///   1. The [Tree::Leaf] representing a leaf node that wraps `(key, ref data)`
///   2. Variant [Tree::Node] representing an inner node with key and reference to underlying data.
///      Additionally, inner nodes contain references to two heap-allocated child trees.
///
/// Notice that the reference to the data must live at least as long as an instance of a tree.
/// This ensures that nodes of any tree will always point to a valid memory section.
///
/// An enum is Rust's version of what in Haskell is called a *type constructor* while individual
/// variants would be respective *data constructors*. For instance the definition of [Tree] below
/// would roughly translate to the following Haskell code (Haskell is a GC language where all
/// values are allocated on the heap so all the reference jugglinlg is hidden away and infinite
/// data structures are possible and common):
/// ```haskell
/// data Tree k v = Leaf k v | Node { key :: k, data :: v, left :: (Tree k v), right :: (Tree k v) }
/// ```
#[derive(Debug)]
pub enum Tree<'a, K, V> {
    Leaf(K, &'a V),
    Node {
        key: K,
        data: &'a V,
        left: Box<Self>,
        right: Box<Self>,
    },
}

/// Note: By not requiring `K` to be [PartialEq] on the [Tree] type itself, we
/// allow users to create tree instances (which is totally valid) but without
/// the option to lookup data by keys.
impl<'a, K: PartialEq + Eq, V> Tree<'a, K, V> {
    /// Lookup method that returns either:
    ///   - reference to the underlying data if the `lookup_key` was found
    ///   - `None` if this BST does not contain node or leaf with `lookup_key`
    pub fn search(&self, lookup_key: &K) -> Option<&'a V> {
        // Pattern matching is ideal for working with ADTs. Also notice that `match` is an
        // expression that returns the last expression from the matched case.
        match self {
            // First pattern checks for any node (`Leaf` or inner `Node`) that has matching key and
            // unwraps the type to its components. As it's shown here, we can match on multiple
            // types in single pattern using `|`.
            //
            // It's also possible to match a component on particular value and/or add guards which
            // are boolean conditions on binded variables - which we use here.
            Self::Leaf(key, data)
            | Self::Node {
                key,
                data,
                left: _,
                right: _,
            } if key == lookup_key => Some(*data),

            // Patterns are checked sequentially, so any other leaf node can't have matching key
            Self::Leaf(_, _) => None,

            // If the key was not found in an inner node, we check left and right sub-trees.
            //
            // Since the compiler knows we've exausted all possibilities for the `Tree` ADT,
            // we don't need a default branch.
            Self::Node {
                key: _,
                data: _,
                left,
                right,
            } => {
                // Here we use the `if let` matching to check the result of the left sub-tree.
                // We can name a pattern (in this case `r`) and since we don't care about the
                // contents of the `Some` option, we can ignore it with a placeholder `_`.
                // One could say that we're only interested in the structure, not the data.
                if let data @ Some(_) = left.search(lookup_key) {
                    data
                } else {
                    right.search(lookup_key)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_tree() {
        // Stack allocated data that the tree nodes will point to.
        // The lifetime will be bound to the tree so that references remain valid.
        let data = vec![
            "root node",
            "inner node",
            "1st leaf",
            "2nd leaf",
            "3rd leaf",
        ];

        // Build small binary tree with with nodes allocated on the heap via `Box`.
        let tree = Tree::Node {
            key: 42,
            data: &data[0],
            left: Box::new(Tree::Node {
                key: 13,
                data: &data[1],
                left: Box::new(Tree::Leaf(1, &data[2])),
                right: Box::new(Tree::Leaf(2, &data[3])),
            }),
            right: Box::new(Tree::Leaf(3, &data[4])),
        };

        // Check that our implementation works
        assert_eq!(Some(&"inner node"), tree.search(&13));
        assert_eq!(Some(&"2nd leaf"), tree.search(&2));
        assert_eq!(None, tree.search(&7));
    }
}

/// This test demonstrates that in Rust all *self-referential* structures must have size known at
/// compile time. This means that such structures *cannot own* data of type `Self` but rather have
/// to indirectly refence these via some sort of a pointer.
///
/// # Example
/// ```compile_fail
/// enum PList<T> {
///     Nil,
///     Cons(T, Self),
/// }
/// ```
///
/// In order to make the example above compile, one would have to use some sort of indirection for
/// the `Self` owned by the `Cons` variant. This indirection can be realized by either
///  * a refence to stack-allocated data (`&`)
///  * a refence to heap-allocated data ([Box])
///  * a refence counting pointer ([std::rc::Rc])
///
/// or similar pointer-like structure which has *defined size* - i.e. is known not to be
/// infinite (of unbounded memory).
pub struct SelfReferentialStructureTest;
