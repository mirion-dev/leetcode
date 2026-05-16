// time  : O(n)
// space : O(1)
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        const APPEARED: i32 = 1 << 31;

        for i in 0..nums.len() {
            let index = (nums[i] & !APPEARED) as usize - 1;
            nums[index] |= APPEARED;
        }

        for (i, v) in nums.iter_mut().enumerate() {
            *v = *v & APPEARED | i as i32 + 1;
        }

        nums.retain(|&v| v & APPEARED == 0);
        nums
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![5, 6]);
    assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
}
