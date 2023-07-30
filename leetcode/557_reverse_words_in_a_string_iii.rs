impl Solution {
  pub fn reverse_words(s: String) -> String {
    let mut res:String = "".to_owned();
    for (idx, s) in s.split_ascii_whitespace().enumerate() {
      if idx != 0 {
        res.push(' ');
      }
      for char in s.chars().rev() {
        res.push(char);
      }
    }
    return res;
  }
}