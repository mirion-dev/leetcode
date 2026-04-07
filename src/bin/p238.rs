impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut prod = 1;
        for (i, &x) in nums.iter().enumerate() {
            res[i] = prod;
            if x == 0 {
                break;
            }
            prod *= x;
        }

        let mut prod = 1;
        for (i, &x) in nums.iter().enumerate().rev() {
            res[i] *= prod;
            prod *= x;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(Solution::product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
}
