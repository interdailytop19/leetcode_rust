use std::char;
impl Solution {
  pub fn convert_to_base7(num: i32) -> String {
    if num == 0 {
      return "0".to_owned();
    }
    let mut num = num;
    let mut res = "".to_owned();
    let is_negative = num.is_negative();
    while num != 0 {
      res = format!("{}{}",(num%7).abs() ,res);
      num/=7;
    }
    if is_negative {
      return format!("-{}", res);
    }
    return res;
  }
}