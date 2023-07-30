impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
      if num == 1 {
        return false;
      }
      let mut left = 0;
      let mut right = num+1;
      let mut res = 0;
      while left < right {
        let mid = left+(right-left)/2;
        if mid>num/mid {
          right = mid;
        }
        else {
          left = mid+1;
        }
      }
      for i in (2..=left-1).rev() {
        if num%i == 0 {
          res += i + num/i;
        }
      }
      return res+1 == num;
    }
  }