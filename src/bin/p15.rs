// time  : O(n^2)
// space : O(log n)
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = vec![];
        let mut i = 0;
        while i < n {
            if nums[i] > 0 {
                break;
            }

            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                match (nums[i] + nums[j] + nums[k]).signum() {
                    -1 => j += 1,
                    1 => k -= 1,
                    _ => {
                        res.push(vec![nums[i], nums[j], nums[k]]);
                        (j, k) = (
                            nums[j + 1..k].iter().position(|&v| v > nums[j]).map_or(k, |offset| j + 1 + offset),
                            nums[j + 1..k].iter().rposition(|&v| v < nums[k]).map_or(j, |offset| j + 1 + offset),
                        );
                    }
                }
            }
            i = nums[i + 1..].iter().position(|&v| v > nums[i]).map_or(n, |offset| i + 1 + offset);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);

    assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
}
