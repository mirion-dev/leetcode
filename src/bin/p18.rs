use std::cmp::Ordering;

// time  : O(n^3)
// space : O(log n)
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = vec![];
        let mut i = 0;
        while i < n {
            if nums[i] as i64 * 4 > target as i64 {
                break;
            }

            let mut j = i + 1;
            while j < n {
                if nums[i] as i64 + nums[j] as i64 * 3 > target as i64 {
                    break;
                }

                let mut k = j + 1;
                let mut l = n - 1;
                while k < l {
                    match (nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64).cmp(&(target as i64)) {
                        Ordering::Less => k += 1,
                        Ordering::Greater => l -= 1,
                        Ordering::Equal => {
                            res.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                            (k, l) = (
                                nums[k + 1..l].iter().position(|&v| v > nums[k]).map_or(l, |offset| k + 1 + offset),
                                nums[k + 1..l].iter().rposition(|&v| v < nums[l]).map_or(k, |offset| k + 1 + offset),
                            );
                        }
                    }
                }
                j = nums[j + 1..].iter().position(|&v| v > nums[j]).map_or(n, |offset| j + 1 + offset);
            }
            i = nums[i + 1..].iter().position(|&v| v > nums[i]).map_or(n, |offset| i + 1 + offset);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
    );
    assert_eq!(Solution::four_sum(vec![2, 2, 2, 2, 2], 8), vec![vec![2, 2, 2, 2]]);

    assert_eq!(
        Solution::four_sum(vec![0, 0, 0, -1000000000, -1000000000, -1000000000, -1000000000], -1000000000),
        vec![vec![-1000000000, 0, 0, 0]]
    );
}
