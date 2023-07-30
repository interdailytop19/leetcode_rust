impl Solution {
  pub fn check_record(s: String) -> bool {
    let mut absent = 0;
    let mut late = 0;
    for char in s.chars() {
      if char == 'A' {
        absent += 1;
        if absent >= 2 || late >= 3 {
          return false;
        }
      }
      if char == 'L' {
        late += 1;
        if absent >= 2 || late >= 3 {
          return false;
        }
      }
      else {
        late = 0;
      }
    }
    return absent < 2 && late < 3;
  }
}