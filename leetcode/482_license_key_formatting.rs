impl Solution {
  pub fn license_key_formatting(s: String, k: i32) -> String {
    let mut a = s.replace("-", "").to_uppercase();
    let mut remain = a.len()%k as usize;
    let mut res:String = "".to_owned();
    let k = k as usize;
    if remain != 0 {
      res.push_str(&a[0..remain]);
    }
    for i in (remain..a.len()).step_by(k) {
      let mut right = i+k;
      if right > a.len() {
        right = a.len();
      }
      if i != 0 {
        res.push('-');
      }
      res.push_str(&a[i..right]);
    }
    return res;
  }
}