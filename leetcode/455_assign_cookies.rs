// sort + 2-pointer
impl Solution {
  pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;
    let mut i:i32 = (g.len()-1) as i32;
    let mut j:i32 = (s.len()-1) as i32;
    g.sort();
    s.sort();
    let mut count:i32 = 0;
    while i >= 0 && j >= 0 {
      if s[j as usize] >= g[i as usize] {
        j -= 1;
        count += 1;
      }
      i -= 1;
    }
    return count;
  }
}