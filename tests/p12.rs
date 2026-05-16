// time  : O(n)
// space : O(n)
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
        .into_iter()
        .fold("".to_string(), |mut res, (val, sym)| {
            while num >= val {
                res.push_str(sym);
                num -= val;
            }
            res
        })
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
    assert_eq!(Solution::int_to_roman(58), "LVIII");
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}
