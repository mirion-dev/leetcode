use std::cmp::Ordering;

// time  : O(n log n)
// space : O(n)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        nums.sort_unstable_by_key(|&(_, v)| v);

        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            match (nums[i].1 + nums[j].1).cmp(&target) {
                Ordering::Equal => return vec![nums[i].0 as i32, nums[j].0 as i32],
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
            }
        }
        vec![]
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
