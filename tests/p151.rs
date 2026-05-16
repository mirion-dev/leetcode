// time  : O(n)
// space : O(n)
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::reverse_words("the sky is blue".to_string()), "blue is sky the");
    assert_eq!(Solution::reverse_words("  hello world  ".to_string()), "world hello");
    assert_eq!(Solution::reverse_words("a good   example".to_string()), "example good a");
}
