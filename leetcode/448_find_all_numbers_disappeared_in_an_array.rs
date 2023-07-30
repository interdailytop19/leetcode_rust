// https://www.cnblogs.com/grandyang/p/6222149.html

// impl Solution {
//   pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
//     let mut nums = nums;
//     let mut res:Vec<i32> = vec![0; nums.len()];
//     for i in 0..nums.len() {
//       let idx = (nums[i]-1) as usize;
//       res[idx] = nums[i];
//     }
//     for i in (0..res.len()).rev() {
//       if res[i] != 0 {
//         res.remove(i);
//       } else {
//         res[i] = (i+1) as i32
//       }
//     }
//     return res;
//   }
// }

impl Solution {
  pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for n in 0..nums.len() {
        let idx = (nums[n].abs() - 1) as usize;
        nums[idx] = -nums[idx].abs();
    }

    let mut rtn = vec![];
    for (idx, &n) in nums.iter().enumerate() {
        if n > 0 {
            rtn.push((idx + 1) as i32);
        }
    }
    return rtn;
  }
}