// time  : O(log n)
// space : O(1)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..32 - n.leading_zeros())
            .rev()
            .map(|i| n >> i & 1)
            .fold((1, 0), |(a, b), bit| {
                let (a2, b2) = (a * (a + 2 * b), a * a + b * b);
                if bit == 1 {
                    (a2, b2)
                } else {
                    (b2, a2 - b2)
                }
            })
            .0
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
}
