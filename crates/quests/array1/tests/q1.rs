// time  : O(n)
// space : O(n)
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.repeat(2)
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::get_concatenation(vec![1, 2, 1]), vec![1, 2, 1, 1, 2, 1]);
    assert_eq!(Solution::get_concatenation(vec![1, 3, 2, 1]), vec![1, 3, 2, 1, 1, 3, 2, 1]);
}
