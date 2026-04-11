// time  : O(log n)
// space : O(1)
impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        let neg = n < 0;
        let mut res = 1.;
        while n != 0 {
            if n % 2 != 0 {
                res *= x;
            }
            x *= x;
            n /= 2;
        }
        if neg { 1. / res } else { res }
    }
}

struct Solution;

fn assert_eq(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() <= f64::EPSILON
}

fn main() {
    assert_eq(Solution::my_pow(2., 10), 1024.);
    assert_eq(Solution::my_pow(2.1, 3), 9.261);
    assert_eq(Solution::my_pow(2., -2), 0.25);
}
