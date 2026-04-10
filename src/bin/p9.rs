impl Solution {
    // time  : O(log n)
    // space : O(1)
    pub fn is_palindrome(mut x: i32) -> bool {
        if x == 0 {
            return true;
        }
        if x < 0 || x % 10 == 0 {
            return false;
        }

        let mut y = 0;
        while x > y {
            y = y * 10 + x % 10;
            x /= 10;
        }
        return x == y || x == y / 10;
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
}
