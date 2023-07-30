use std::collections::HashMap;
impl Solution {
  pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut map:HashMap<String, usize> = HashMap::new();
    let mut count:usize = usize::MAX;
    let mut res:Vec<String> = vec![];
    for (idx, s) in list1.iter().enumerate() {
      map.insert(s.clone(), idx);
    }
    for (idx, s) in list2.iter().enumerate() {
      match map.get(s) {
        Some(idx_1) => {
          if idx+idx_1 < count {
            count = idx+idx_1;
            res.drain(..);
            res.push(s.to_owned());
          }
          else if idx+idx_1 == count {
            res.push(s.to_owned());
          }
        },
        _ => (),
        None => ()
      }
    }
    return res;
  }
}