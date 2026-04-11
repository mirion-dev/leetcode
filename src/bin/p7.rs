// time  : O(log n)
// space : O(1)
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res = 0i32;
        while x != 0 {
            match res.checked_mul(10).and_then(|v| v.checked_add(x % 10)) {
                Some(v) => res = v,
                None => return 0,
            }
            x /= 10;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);

    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(i32::MAX), 0);
    assert_eq!(Solution::reverse(i32::MIN), 0);
}
