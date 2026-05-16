// time  : O(n)
// space : O(1)
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim_start().chars().peekable();
        let sign = match s.next_if(|&ch| ch == '+' || ch == '-') {
            Some('-') => -1,
            _ => 1,
        };
        s.map_while(|ch| ch.to_digit(10))
            .try_fold(0i32, |v, d| v.checked_mul(10).and_then(|v| v.checked_add(d as i32 * sign)))
            .unwrap_or(if sign == 1 { i32::MAX } else { i32::MIN })
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi(" -042".to_string()), -42);
    assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
}

#[test]
fn extra() {
    assert_eq!(Solution::my_atoi("".to_string()), 0);
    assert_eq!(Solution::my_atoi(i32::MAX.to_string()), i32::MAX);
    assert_eq!(Solution::my_atoi(i32::MIN.to_string()), i32::MIN);
    assert_eq!(Solution::my_atoi(i64::MAX.to_string()), i32::MAX);
    assert_eq!(Solution::my_atoi(i64::MIN.to_string()), i32::MIN);
}
