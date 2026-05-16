// time  : O(n)
// space : O(1)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut last = [0; 128];
        s.chars()
            .enumerate()
            .map(|(r, ch)| {
                let ch = ch as usize;
                l = l.max(last[ch]);
                last[ch] = r + 1;
                (r - l + 1) as i32
            })
            .max()
            .unwrap_or(0)
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
}
