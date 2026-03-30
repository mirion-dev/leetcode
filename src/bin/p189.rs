impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.is_empty() {
            return;
        }

        let mut l = 0;
        let mut m = nums.len() - k as usize % nums.len();
        loop {
            let (left, right) = nums[l..].split_at_mut(m - l);
            let n = left.len().min(right.len());
            if n == 0 {
                return;
            }

            left[..n].swap_with_slice(&mut right[..n]);
            l += n;
            if n == left.len() {
                m += n;
            }
        }
    }
}

struct Solution;

fn test(mut nums: Vec<i32>, k: i32, expected: Vec<i32>) {
    Solution::rotate(&mut nums, k);
    assert_eq!(nums, expected);
}

fn main() {
    test(vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]);
    test(vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]);

    test(vec![], 1, vec![]);
    test(vec![1], 0, vec![1]);
    test(vec![1], 1, vec![1]);
}
