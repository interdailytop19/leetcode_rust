// https://www.cnblogs.com/grandyang/p/6828687.html

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
  pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return Solution::helper(&root, &sub_root);
  }
  
  fn helper(root:&Option<Rc<RefCell<TreeNode>>>, sub_root:&Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
      return false;
    }
    if Solution::is_same(&root, &sub_root) {
      return true;
    }
    if let Some(n) = root {
      let n = n.borrow();
      return Solution::helper(&n.left, &sub_root) || Solution::helper(&n.right, &sub_root)
    }
    return false;
  }
  
  fn is_same(node_1:&Option<Rc<RefCell<TreeNode>>>, node_2:&Option<Rc<RefCell<TreeNode>>>) -> bool {
    if node_1.is_none() && node_2.is_none() {
      return true;
    }
    if let Some(n1) = node_1 {
      if let Some(n2) = node_2 {
        let n1 = n1.borrow();
        let n2 = n2.borrow();
        if n1.val != n2.val {
          return false;
        }
        return Solution::is_same(&n1.left, &n2.left) && Solution::is_same(&n1.right, &n2.right);
      }
      return false
    }
    return false;
  }
}