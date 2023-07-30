// brute force
// use std::collections::HashMap;
// impl Solution {
//     pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//       let mut map = HashMap::new();
//       let mut res:Vec<i32> = vec![-1; nums1.len()];
//       for i in 0..nums2.len() {
//         map.insert(nums2[i], i);
//       }
//       'loop1: for i in 0..nums1.len() {
//         if let Some(idx) = map.get(&nums1[i]) {
//           for j in (*idx+1)..nums2.len() {
//             if nums2[j] > nums1[i] {
//               res[i] = nums2[j];
//               continue 'loop1;
//             }
//           }
//         }
//       }
//       return res;
//     }
// }

// hashmap + stack
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
      let mut map:HashMap<i32,i32> = HashMap::new();
      let mut stack = vec![];
      let mut res:Vec<i32> = vec![-1; nums1.len()];
      for i in 0..nums2.len() {
        while !stack.is_empty() && nums2[i] > *stack.last().unwrap() {
          map.insert(stack.pop().unwrap(), nums2[i]);
        }
        stack.push(nums2[i]);
      }
      // println!("{:?}", stack);
      // println!("{:?}", map);
      for i in 0..nums1.len() {
        if let Some(next_greater) = map.get(&nums1[i]) {
          res[i] = *next_greater;
        }
      }
      return res;
    }
}