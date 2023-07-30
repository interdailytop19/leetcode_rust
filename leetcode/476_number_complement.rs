impl Solution {
  pub fn find_complement(num: i32) -> i32 {
    let mut num = num;
    let mut res:i32 = 0;
    let mask = 1;
    let mut count = 0;
    while num != 0 {
      if num&mask == 0 {
        res |= 1 << count;
      }
      num >>= 1;
      count += 1;
    }
    return res;
  }
}