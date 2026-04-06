impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = 0;
        for (i, &v) in nums.iter().enumerate() {
            if i > farthest {
                return false;
            }
            farthest = farthest.max(v as usize + i);
            if farthest >= nums.len() - 1 {
                return true;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
}
