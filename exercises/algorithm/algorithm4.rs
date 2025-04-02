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
    // 创建一个新的空二叉搜索树
    fn new() -> Self {
        BinarySearchTree { root: None } // 初始化根节点为空
    }

    // 插入一个值到二叉搜索树中
    fn insert(&mut self, value: T) {
        // 如果根节点为空，则创建一个新的 TreeNode 并设置为根节点
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            // 否则，调用 TreeNode 的 insert 方法来插入节点
            self.root.as_mut().unwrap().insert(value);
        }
    }

    // 在二叉搜索树中查找一个值
    fn search(&self, value: T) -> bool {
        // 从根节点开始搜索
        let mut current = &self.root;
        while let Some(node) = current {
            // 比较当前节点的值与目标值
            match value.cmp(&node.value) {
                Ordering::Less => current = &node.left, // 如果目标值小于当前节点值，则搜索左子树
                Ordering::Greater => current = &node.right, // 如果目标值大于当前节点值，则搜索右子树
                Ordering::Equal => return true, // 如果找到目标值，则返回 true
            }
        }
        false // 如果未找到目标值，则返回 false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // 比较当前节点的值与目标值
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 如果目标值小于当前节点值，则插入左子树
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            Ordering::Greater => {
                // 如果目标值大于当前节点值，则插入右子树
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.right.as_mut().unwrap().insert(value);
                }
            }
            Ordering::Equal => {
                // 如果目标值等于当前节点值，则不插入（避免重复）
            }
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


