// 105. Construct Binary Tree from Preorder and Inorder Traversal
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        
        let mut inorder_map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i);
        }
        
        Self::build_tree_helper(&preorder, &inorder_map, 0, preorder.len() - 1, 0)
    }
    
    fn build_tree_helper(
        preorder: &Vec<i32>,
        inorder_map: &HashMap<i32, usize>,
        preorder_start: usize,
        preorder_end: usize,
        inorder_start: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_start > preorder_end {
            return None;
        }
        
        let root_val = preorder[preorder_start];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        
        let root_inorder_idx = *inorder_map.get(&root_val).unwrap();
        
        let left_subtree_size = root_inorder_idx - inorder_start;
        
        if left_subtree_size > 0 {
            root.borrow_mut().left = Self::build_tree_helper(
                preorder,
                inorder_map,
                preorder_start + 1,
                preorder_start + left_subtree_size,
                inorder_start,
            );
        }
        
        if root_inorder_idx + 1 <= inorder_start + (preorder_end - preorder_start) {
            root.borrow_mut().right = Self::build_tree_helper(
                preorder,
                inorder_map,
                preorder_start + left_subtree_size + 1,
                preorder_end,
                root_inorder_idx + 1,
            );
        }
        
        Some(root)
    }
}