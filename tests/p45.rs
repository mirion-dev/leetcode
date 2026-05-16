// time  : O(n)
// space : O(1)
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let last = nums.len() - 1;
        let mut farthest = 0;
        let mut next_farthest = 0;
        let mut step = 0;
        for (i, &v) in nums.iter().enumerate().take(last) {
            if i > farthest {
                return -1;
            }

            next_farthest = next_farthest.max(v as usize + i);
            if next_farthest >= last {
                return step + 1;
            }
            if i == farthest {
                step += 1;
                farthest = next_farthest;
            }
        }
        step
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
}

#[test]
fn extra() {
    assert_eq!(Solution::jump(vec![2, 1]), 1);
}
