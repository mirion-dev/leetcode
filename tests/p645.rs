// time  : O(n)
// space : O(1)
impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        const APPEARED: i32 = 1 << 31;

        let mut twice = usize::MAX;
        for i in 0..nums.len() {
            let index = (nums[i] & !APPEARED) as usize - 1;
            if nums[index] & APPEARED != 0 {
                twice = index;
            } else {
                nums[index] |= APPEARED;
            }
        }

        nums.iter()
            .enumerate()
            .find(|&(_, &v)| v & APPEARED == 0)
            .map_or(vec![], |(i, _)| vec![twice as i32 + 1, i as i32 + 1])
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
}
