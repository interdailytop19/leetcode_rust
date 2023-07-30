impl Solution {
  pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut count = 0;
    let mut time_count = 0;
    for time in time_series {
      if time != 0 && time_count-time >= 0 {
        let mut new_time_count = time+duration-1;
        count += new_time_count-time_count;
        time_count = new_time_count;
      }
      else {
        time_count = time+duration-1;
        count += duration;
      }
    }
    return count;
  }
}