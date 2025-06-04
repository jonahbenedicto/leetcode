// 106. Construct Binary Tree from Inorder and Postorder Traversal
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        
        let mut inorder_map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i);
        }
        
        Self::build_helper(&inorder, &postorder, &inorder_map, 
                          0, inorder.len() - 1, 0, postorder.len() - 1)
    }
    
    fn build_helper(
        inorder: &[i32],
        postorder: &[i32],
        inorder_map: &HashMap<i32, usize>,
        in_start: usize,
        in_end: usize,
        post_start: usize,
        post_end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_start > in_end || post_start > post_end {
            return None;
        }
        
        let root_val = postorder[post_end];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        
        let root_idx = *inorder_map.get(&root_val).unwrap();
        
        let left_size = if root_idx > in_start { root_idx - in_start } else { 0 };
        
        if root_idx > in_start {
            root.borrow_mut().left = Self::build_helper(
                inorder,
                postorder,
                inorder_map,
                in_start,
                root_idx - 1,
                post_start,
                post_start + left_size - 1,
            );
        }
        
        if root_idx < in_end {
            root.borrow_mut().right = Self::build_helper(
                inorder,
                postorder,
                inorder_map,
                root_idx + 1,
                in_end,
                post_start + left_size,
                post_end - 1,
            );
        }
        
        Some(root)
    }
}