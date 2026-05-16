// time  : O(n 4^n)
// space : O(n 4^n)
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        digits.chars().fold(vec!["".to_string()], |res, ch| {
            let charset = match ch {
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                _ => "",
            };
            res.iter().flat_map(|s| charset.chars().map(|ch| s.to_owned() + &ch.to_string())).collect()
        })
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
    );
    assert_eq!(Solution::letter_combinations("2".to_string()), vec!["a", "b", "c"]);
}
