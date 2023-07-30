impl Solution {
  pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let is_even:bool = area&2 ==0;
    let mut sqrt = 0;
    let mut left = 0;
    let mut right = area+1;
    while left < right {
      let mid = left+(right-left)/2;
      if mid > area/mid {
        right = mid;
      }
      else {
        left = mid+1;
      }
    }
    for i in (1..=(left-1)).rev() {
      if area%i==0 {
        return vec![area/i, i];
      }
    }
    return vec![area, 1];
  }
}