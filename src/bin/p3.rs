impl Solution {
    // time  : O(n)
    // space : O(1)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut last = [0; 128];
        s.chars()
            .enumerate()
            .map(|(r, ch)| {
                l = l.max(last[ch as usize]);
                last[ch as usize] = r + 1;
                (r - l + 1) as i32
            })
            .max()
            .unwrap_or(0)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
}
