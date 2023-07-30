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
use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res:i32 = 0;
    fn helper(node:&Option<Rc<RefCell<TreeNode>>>, res:&mut i32) -> i32 {
      if node.is_none() {
        return -1_i32;
      }
      if let Some(n) = node {
        let n = n.borrow();
        let left = 1+helper(&n.left, res);
        let right = 1+helper(&n.right, res);
        *res = cmp::max(*res, left+right);
        return cmp::max(left, right);
      }
      return -1;
    }
    
    helper(&root, &mut res);
    return res;
  }
}