use itertools::Itertools;

// time  : O(n)
// space : O(n)
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (left, right) = nums.split_at(n as usize);
        left.iter().interleave(right).copied().collect()
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
    assert_eq!(Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4), vec![1, 4, 2, 3, 3, 2, 4, 1]);
    assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}
