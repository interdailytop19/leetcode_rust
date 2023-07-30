// loop every substring length and check every sub string length is equal
impl Solution {
  pub fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();
    'loop1: for i in 1..=(n/2) {
      if s.len() % i != 0 {
        continue;
      }
      let target = &s[0..i];
      for j in (0+i..s.len()).step_by(i) {
        if (&s[j..j+i] != target) {
          continue 'loop1;
        }
      }
      return true;
    }
    return false;
  }
}

// abcabc

// ababab

// bbbbbb

//   1 2 3 4 5 6
// 1 T F F F F F
// 2   F F T F T
// 3     F F F F
// 4       F F F
// 5         F F
// 6           F

// r(i) = if j == true then substring(0, i-j) = substring(j, i)
// else 