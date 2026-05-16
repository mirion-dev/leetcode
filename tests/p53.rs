// time  : O(n)
// space : O(1)
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut last = 0;
        nums.iter()
            .map(|&i| {
                last = (last + i).max(i);
                last
            })
            .max()
            .unwrap_or(0)
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}
