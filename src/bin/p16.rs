// time  : O(n^2)
// space : O(log n)
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = i32::MAX;
        let mut min_distance = i32::MAX;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                let diff = sum - target;
                match diff.signum() {
                    -1 => j += 1,
                    1 => k -= 1,
                    _ => return target,
                }
                if diff.abs() < min_distance {
                    res = sum;
                    min_distance = diff.abs();
                }
            }
            i = nums[i + 1..].iter().position(|&v| v > nums[i]).map_or(n, |offset| i + 1 + offset);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);

    assert_eq!(Solution::three_sum_closest(vec![0, 1, 2], 3), 3);
}
