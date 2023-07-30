// dp 
impl Solution {
    pub fn fib(n: i32) -> i32 {
      if n == 0 {
        return 0;
      }
      let mut last_1 = 0;
      let mut last_2 = 1;
      for i in 2..=n {
        let temp = last_1+last_2;
        last_1 = last_2;
        last_2 = temp;
      }
      return last_2;
    }
  }