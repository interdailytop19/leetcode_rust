// Time Limit Exceeded

// impl Solution {
//   pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
//     let m = m as usize;
//     let n = n as usize;
//     let mut mem:Vec<Vec<i32>> = vec![vec![0;n];m];
//     for operation in ops {
//       for i in 0..operation[0]*operation[1] {
//         mem[(i/operation[1]) as usize][(i%operation[1]) as usize] += 1;
//       }
//     }
//     let mut max = -1;
//     let mut res = 0;
//     for i in 0..m*n {
//       if mem[i/n][i%n] > max {
//         max = mem[i/n][i%n];
//         res = 1;
//       }
//       else if mem[i/n][i%n] == max {
//         res += 1;
//       }
//     }
//     return res;
//   }
// }


// https://www.polarxiong.com/archives/LeetCode-598-range-addition-ii.html
use std::cmp;
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
      let mut m = m;
      let mut n = n;
      for op in ops {
        m = cmp::min(m, op[0]);
        n = cmp::min(n, op[1]);
      }
      return m*n;
    }
}