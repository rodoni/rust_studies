use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct BinaryTree<T> {
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T>
    where
        T: Copy,
{
    ///
    /// Create a new Binary Tree node.
    ///
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    ///
    /// Create a balanced tree from an array reference.
    ///
    pub fn from(new_values: &[T]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: BinaryTree<T> = BinaryTree::new(*first);

        for value in rest {
            root.insert(*value)
        }
        root
    }

    ///
    /// Insert a tree node in the next available branch with breadth first traversal.
    ///
    pub fn insert(&mut self, new_value: T) {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_front(self);
        loop {
            let BinaryTree {
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
        }
    }

    ///
    /// Insert a left child node.
    ///
    pub fn left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    ///
    /// Insert a right child node.
    ///
    pub fn right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tree() {
        let tree = BinaryTree::new(1);

        assert_eq!(tree.value, 1);
    }

    #[test]
    fn insert_left() {
        let tree = BinaryTree::new(1).left(BinaryTree::new(2));

        if let Some(node) = tree.left {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.right, None);
    }

    #[test]
    fn insert_right() {
        let tree = BinaryTree::new(1).right(BinaryTree::new(2));

        if let Some(node) = tree.right {
            assert_eq!(node.value, 2);
        }

        assert_eq!(tree.left, None);
    }

    #[test]
    fn insert() {
        let mut tree = BinaryTree::new(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);

        assert_eq!(
            tree,
            BinaryTree::new(1)
                .left(
                    BinaryTree::new(2)
                        .left(BinaryTree::new(4))
                        .right(BinaryTree::new(5))
                )
                .right(BinaryTree::new(3))
        );

        tree.insert(6);

        assert_eq!(
            tree,
            BinaryTree::new(1)
                .left(
                    BinaryTree::new(2)
                        .left(BinaryTree::new(4))
                        .right(BinaryTree::new(5))
                )
                .right(BinaryTree::new(3).left(BinaryTree::new(6)))
        )
    }

    #[test]
    fn create_new_tree_from() {
        let tree = BinaryTree::from(&[1, 2, 3, 4, 5, 6]);

        assert_eq!(
            tree,
            BinaryTree::new(1)
                .left(
                    BinaryTree::new(2)
                        .left(BinaryTree::new(4))
                        .right(BinaryTree::new(5))
                )
                .right(BinaryTree::new(3).left(BinaryTree::new(6)))
        )
    }
}
