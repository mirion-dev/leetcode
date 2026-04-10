impl Solution {
    // time  : O(n)
    // space : O(1)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
