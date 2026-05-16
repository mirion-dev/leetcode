use std::cmp::Ordering;

// time  : O(n)
// space : O(1)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max = 0;
        while l < r {
            max = max.max((r - l) as i32 * height[l].min(height[r]));
            match height[l].cmp(&height[r]) {
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
                Ordering::Equal => {
                    l += 1;
                    r -= 1
                }
            }
        }
        max
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
