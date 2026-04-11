// time  : O(n)
// space : O(1)
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .rev()
            .fold((0, 0), |(res, last), ch| {
                let val = match ch {
                    'I' => 1,
                    'V' => 5,
                    'X' => 10,
                    'L' => 50,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    _ => 0,
                };
                if val >= last {
                    (res + val, val)
                } else {
                    (res - val, val)
                }
            })
            .0
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
