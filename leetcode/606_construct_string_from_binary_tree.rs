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
impl Solution {
  pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> String {
      match node {
        Some(n) => {
          let n = n.borrow();
          let left = helper(&n.left);
          let right = helper(&n.right);
          if left.is_empty() && right.is_empty() {
            return format!("{}", n.val);
          }
          else if right.is_empty() {
            return format!("{}({})", n.val, left);
          }
          else {
            return format!("{}({})({})", n.val, left, right);
          }
          
        },
        None => {
          return "".to_owned();
        }
      }
    }
    return helper(&root);
  }
}