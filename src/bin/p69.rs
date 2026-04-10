impl Solution {
    // time  : O(log log n)
    // space : O(1)
    pub fn my_sqrt(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let mut x = 1 << (33 - n.leading_zeros()) / 2;
        loop {
            let y = (x + n / x) / 2;
            if y >= x {
                return x;
            }
            x = y;
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);

    assert_eq!(Solution::my_sqrt(0), 0);
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(2), 1);
}
