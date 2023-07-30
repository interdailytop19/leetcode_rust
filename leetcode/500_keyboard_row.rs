impl Solution {
  pub fn find_words(words: Vec<String>) -> Vec<String> {
    let mut count = 0;
    let first = "qwertyuiop".to_owned();
    let second = "asdfghjkl".to_owned();
    let third = "zxcvbnm".to_owned();
    let mut res:Vec<String> = vec![];
    'loop1: for i in 0..words.len() {
      let word = words[i].to_lowercase();
      count = 0;
      for char in word.chars() {
        if count == 0 {
          if first.contains(char) {
            count = 1;
          }
          if second.contains(char) {
            count = 2;
          }
          if third.contains(char) {
            count = 3;
          }
        }
        else {
          if first.contains(char) && count != 1 {
            continue 'loop1;
          }
          if second.contains(char) && count != 2 {
            continue 'loop1;
          }
          if third.contains(char) && count != 3 {
            continue 'loop1;
          }
        }
      }
      res.push(words[i].clone());
    }
    return res;
  }
}