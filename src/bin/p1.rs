use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        nums.into_iter()
            .enumerate()
            .find_map(|(i, v)| {
                map.get(&(target - v)).map(|&j| vec![j as i32, i as i32]).or_else(|| {
                    map.insert(v, i);
                    None
                })
            })
            .unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
