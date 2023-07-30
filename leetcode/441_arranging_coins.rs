impl Solution {
  pub fn arrange_coins(n: i32) -> i32 {
    let mut count:i32 = 0;
    let mut i:i32 = 1;
    let mut n:i32 = n;
    while i == 1 || n - i >= 0 {
      count += 1;
      n -= i;
      i += 1;
    }
    return count;
  }
}