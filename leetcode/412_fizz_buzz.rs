impl Solution {
  pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res:Vec<String> = vec![];
    for i in 1..=n {
      if i % 3 == 0 && i % 5 == 0 {
        res.push("FizzBuzz".to_owned());
        continue;
      }
      if i % 3 == 0 {
        res.push("Fizz".to_owned());
        continue;
      }
      if i % 5 == 0 {
        res.push("Buzz".to_owned());
        continue;
      }
      res.push(format!("{}", i));
    }
    return res;
  }
}