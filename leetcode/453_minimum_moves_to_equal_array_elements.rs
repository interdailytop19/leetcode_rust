// https://zxi.mytechroad.com/blog/math/leetcode-453-minimum-moves-to-equal-array-elements/
impl Solution {
  pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mut sum:i32 = 0;
    let mut min:i32 = i32::MAX;
    let n = nums.len() as i32;
    for num in nums {
      sum += num;
      if num < min {
        min = num;
      }
    }
    return sum - n * min;
  }
}