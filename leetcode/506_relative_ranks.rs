use std::collections::HashMap;
impl Solution {
  pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut sorted:Vec<i32> = score.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    let mut map:HashMap<i32, usize> = HashMap::new();
    let mut res:Vec<String> = vec![];
    for i in 0..sorted.len() {
      map.insert(sorted[i], i);
    }
    for i in 0..score.len() {
      if let Some(idx) = map.get(&score[i]) {
        match idx {
          0 => res.push("Gold Medal".to_owned()),
          1 => res.push("Silver Medal".to_owned()),
          2 => res.push("Bronze Medal".to_owned()),
          _ => res.push(format!("{}", idx+1)),
        }
      }
    }
    return res;
  }
}