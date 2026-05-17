// time  : O(n)
// space : O(n)
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack = vec![];
        let mut max = 0;
        for i in 0..=n {
            while let Some(&top) = stack.last()
                && (i == n || heights[top] > heights[i])
            {
                stack.pop();
                max = max.max((i - stack.last().map_or(0, |&v| v + 1)) as i32 * heights[top]);
            }
            stack.push(i);
        }
        max
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
}
