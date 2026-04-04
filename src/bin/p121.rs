impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        prices
            .iter()
            .map(|&i| {
                min = min.min(i);
                i - min
            })
            .max()
            .unwrap_or(0)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
