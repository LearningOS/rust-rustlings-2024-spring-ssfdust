/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug, Clone)]
struct TreeNode<T>
where
    T: Ord + Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone)]
struct BinarySearchTree<T>
where
    T: Ord + Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
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
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let new_node = Box::new(TreeNode::new(value.clone()));
        let mut cur_node = &mut self.root;
        loop {
            match cur_node {
                Some(ref mut cur_node_inner) => {
                    if value < cur_node_inner.value {
                        cur_node = &mut cur_node_inner.left;
                    } else if value > cur_node_inner.value {
                        cur_node = &mut cur_node_inner.right;
                    } else {
                        break;
                    }
                },
                None => {
                    std::mem::replace(cur_node, Some(new_node));
                    break;
                },
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if let Some(ref cur_node) = self.root {
            let mut cur_node = Some(cur_node);
            while let Some(cur_node_inner) = cur_node {
                if value < cur_node_inner.value {
                    cur_node = cur_node_inner.left.as_ref();
                } else if value > cur_node_inner.value {
                    cur_node = cur_node_inner.right.as_ref();
                } else {
                    return true;
                }
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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


