// time  : O(n)
// space : O(1)
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&n| n != 1).map(|x| x.len()).max().unwrap_or(0) as i32
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
}
