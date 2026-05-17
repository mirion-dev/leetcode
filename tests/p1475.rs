// time  : O(n)
// space : O(n)
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res = prices.clone();
        for i in 0..prices.len() {
            while let Some(&top) = stack.last()
                && prices[i] <= prices[top]
            {
                res[top] -= prices[i];
                stack.pop();
            }
            stack.push(i);
        }
        res
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::final_prices(vec![8, 4, 6, 2, 3]), vec![4, 2, 4, 2, 3]);
    assert_eq!(Solution::final_prices(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
}

#[test]
fn extra() {
    assert_eq!(Solution::final_prices(vec![10, 2, 8, 3, 7]), vec![8, 2, 5, 3, 7]);
}
