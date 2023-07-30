// impl Solution {
//   pub fn third_max(nums: Vec<i32>) -> i32 {
//     let mut first = None;
//     let mut second = None;
//     let mut third = None;

//     for n in nums {
//       let n = Some(n);
//       if n > first {
//           third = second;
//           second = first;
//           first = n;
//       } else if n < first && n > second {
//           third = second;
//           second = n;
//       } else if n < second && n >= third {
//           third = n;
//       }
//     }
//     return third.unwrap_or(first.unwrap());
//   }
// }


use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
  pub fn third_max(nums: Vec<i32>) -> i32 {
    // https://en.wikipedia.org/wiki/Median_of_medians
    // find the nth smallest;
    // 1 is the smallest
    fn select(nums:&mut Vec<i32>, left:usize, right:usize, k:usize) -> usize {
      if k > nums.len() {
        panic!("k out of bound");
      }
      let mut right = right;
      let mut k = k-1;
      let mut left = left;
      loop {
        if left == right {
          return left;
        }
        let p = find_pivot(nums, left, right);
        let pivot_index = partition(nums, left, right, p, k);
        if k == pivot_index {
          return k;
        }
        else if k < pivot_index {
          right = pivot_index - 1;
        }
        else {
          left = pivot_index + 1;
        }
      }
    }

    fn find_pivot(nums: &mut Vec<i32>, left:usize, right:usize) -> usize{
      if right - left < 5 {
        return insertion_sort(nums, left, right);
      } 
      for i in (left..=right).step_by(5) {
        let mut sub_right = i+4;
        if sub_right > right {
          sub_right = right
        }
        let median_5 = insertion_sort(nums, i, sub_right);
        nums.swap(median_5, left+(i-left)/5);
      }
      let mid = (right-left)/10 + left+1;
      return select(nums, left, left+((right-left)/5), mid);
    }

    fn insertion_sort(nums: &mut Vec<i32>, left:usize, right:usize) -> usize {
      for i in (left+1)..=right {
        if nums[i] < nums[i-1] {
          let key = nums[i];
          let mut j = i;
          while j > left && key < nums[j-1] {
            nums.swap(j, j-1);
            j -= 1;
          }
          nums.swap(i, j)
        }
      }
      return (left+right)/2;
    }

    fn partition(nums:&mut Vec<i32>, left:usize, right:usize, pivot:usize, k:usize) -> usize {
      let pivot_value = nums[pivot];
      nums.swap(pivot, right);
      let mut store_index = left;
      for i in left..=(right-1) {
        if nums[i] < pivot_value {
          nums.swap(i, store_index);
          store_index += 1;
        }
      }
      let mut store_index_eq = store_index;
      for i in store_index..=(right-1) {
        if nums[i] == pivot_value {
          nums.swap(i, store_index_eq);
          store_index_eq += 1;
        }
      }
      nums.swap(right, store_index_eq);
      if k < store_index {
        return store_index;
      }
      if k <= store_index_eq {
        return k;
      }
      return store_index_eq;
    }

    let mut set:HashSet<i32> = HashSet::from_iter(nums);
    let mut nums: Vec<i32> = set.into_iter().collect();
    let k:usize = if nums.len() >= 3 {nums.len()-2} else {nums.len()};
    let n = nums.len()-1;
    let res = select(&mut nums, 0, n, k);
    return nums[res];
  }
}