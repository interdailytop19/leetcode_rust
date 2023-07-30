impl Solution {
  pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut count = n;
    let n_len = flowerbed.len();
    let mut mem:Vec<i32> = flowerbed.clone();
    for i in 0..n_len {
      if mem[i] == 0 {
        if count == 0 {
          return true;
        }
        if (i == 0 || mem[i-1] == 0) && (i == n_len-1 || mem[i+1] == 0) {
          mem[i] = 1;
          count -= 1;
        }
      }
    }
    return count == 0;
  }
}