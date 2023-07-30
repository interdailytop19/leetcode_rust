use std::cmp;
impl Solution {
  pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut res:i32 = 0;
    let mut local:i32 = 0;
    while i < nums.len() {
      if nums[i] == 1 {
        local += 1;
      }
      else if nums[i] == 0 && local != 0 {
        res = cmp::max(res, local);
        local = 0;
      }
      i+=1;
    }
    return cmp::max(res, local);
  }
}