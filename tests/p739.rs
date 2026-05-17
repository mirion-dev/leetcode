// time  : O(n)
// space : O(n)
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res = vec![0; temperatures.len()];
        for current in 0..temperatures.len() {
            while let Some(&top) = stack.last()
                && temperatures[current] > temperatures[top]
            {
                res[top] = (current - top) as i32;
                stack.pop();
            }
            stack.push(current);
        }
        res
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]), vec![1, 1, 4, 2, 1, 1, 0, 0]);
    assert_eq!(Solution::daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(Solution::daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
}
