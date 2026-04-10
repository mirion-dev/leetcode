impl Solution {
    // time  : O(log n)
    // space : O(1)
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut count = 0;
        while n != 0 {
            n /= 5;
            count += n;
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(0), 0);
}
