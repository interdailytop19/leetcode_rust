use std::collections::HashSet;
impl Solution {
  pub fn longest_palindrome(s: String) -> i32 {
    let mut res:i32 = 0;
    let mut hashset:HashSet<char> = HashSet::new();
    for char in s.chars() {
      match hashset.get(&char) {
        None => {
          hashset.insert(char);
        },
        Some(e) => {
          res += 2;
          hashset.remove(&char);
        }
      }
    }
    if !hashset.is_empty() {
      res += 1;
    }
    return res;
  }
}