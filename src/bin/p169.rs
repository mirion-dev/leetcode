impl Solution {
    // time  : O(n)
    // space : O(1)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        nums.iter().fold(0, |x, &v| {
            if count == 0 || x == v {
                count += 1;
                v
            } else {
                count -= 1;
                x
            }
        })
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
