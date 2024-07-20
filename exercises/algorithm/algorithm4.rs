/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::new(value));
        self.root=Some(Self::insert_node(self.root.take(), new_node));
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>,  new_node: Box<TreeNode<T>>) -> Box<TreeNode<T>> {
        match node {
            None => new_node,
            Some(mut node) => {
                match new_node.value.cmp(&node.value) {
                    Ordering::Less => {
                        node.left = Some(Self::insert_node(node.left.take(), new_node));
                    },
                    Ordering::Greater => {
                        node.right = Some(Self::insert_node(node.right.take(), new_node));
                    },
                    Ordering::Equal => {
                        // Handle duplicate values as needed, e.g., by ignoring them
                        // or by updating the node's value.
                    },
                }
                node
            },
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        self.search_node(self.root.as_ref(), value)
    }

    fn search_node(&self, node: Option<&Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => self.search_node(node.left.as_ref(), value),
                Ordering::Greater => self.search_node(node.right.as_ref(), value),
                Ordering::Equal => true,
            },
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            },
            Ordering::Equal => {
                // Handle duplicates as needed, e.g., by ignoring them
                // or by updating the node's value.
            },
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


