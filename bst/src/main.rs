
//Option<T> is an enum with two variants:
//Some(T), which represents a value of type T
//None, which represents the lack of a value
//Box<T> is a smart pointer type, which is a pointer type that is used to manage memory allocation.

use std::collections::VecDeque;

//Box<T> is used to store data on the heap, which is the area of memory that is not stored on the stack.
#[derive(Debug, PartialEq)]

pub struct BinaryTree<T>{
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}
impl<T> BinaryTree<T>
where
    T:Copy,
{
    // Create a new binary tree with a single node
    pub fn new(value: T) -> Self
    {
        BinaryTree
        {
            value,
            left: None,
            right: None,
        }
    }

    // Create a balanced tree from a array reference
    pub fn from(new_values: &[T]) -> Self
    {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root: BinaryTree<T> = BinaryTree::new(*first);
        for value in rest
        {
            root.insert(*value);
        }
        root
    }

    //Insert a tree node in the next available branch with breadth first traversal
    pub fn insert(&mut self, new_value: T)
    {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_back(self);
        loop
        {
            let BinaryTree
            {
                ref mut left,
                ref mut right,
                ..
            } = queue.pop_front().unwrap();

            match left
            {
                Some(node) =>
                    { queue.push_front(node); }
                None =>
                {
                    *left = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
            match right
            {
                Some(node) =>
                    { queue.push_front(node); }
                None =>
                {
                    *right = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
        }
    }
    //Insert a left child node.
    pub fn left(mut self, node: BinaryTree<T>) -> Self
    {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: BinaryTree<T>) -> Self
    {
        self.right = Some(Box::new(node));
        self
    }
}

#[test]
fn create_new_tree_from()
{
    let tree = BinaryTree::from(&[1, 2, 3, 4, 5, 6]);

    assert_eq!(
        tree,
        BinaryTree::new(1).left(
            BinaryTree::new(2).left(
                BinaryTree::new(4).left(
                    BinaryTree::new(5)
                ).right(
                    BinaryTree::new(6)
                )
            ).right(
                BinaryTree::new(3)
            )
        )
    )
}





fn main() {
    println!("Hello, world!");
}
