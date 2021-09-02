/// An enum representing an Binary Tree Algebraic Data Type (ADT)
///
/// This enum defined two distinct types (variants), each of different shape and size:
///   1. The `Leaf` representing a leaf node that wraps `(key, ref data)`
///   2. Struct `Node` representing an inner node with key and reference to underlying data.
///      Additionally, inner nodes contain references to two heap-allocated child trees.
///
/// Notice that the reference to the data must live at least as long as an instance of a tree.
/// This ensures that nodes of any tree will always point to a valid memory section.
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

// Note: By not requiring `K` to be `PartialEq` on the `Tree` type itself, we
// allow users to create tree instances (which is totally valid) but without
// the option to lookup data by keys.
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
