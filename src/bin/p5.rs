impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let extended: Vec<char> = ['^'].into_iter().chain(s.iter().flat_map(|&c| ['.', c])).chain(['.', '$']).collect();
        let mut radius = vec![0; extended.len()];
        let mut rightmost = 0usize;
        (2..extended.len() - 2)
            .map(|i| {
                let right = rightmost + radius[rightmost];
                let mut r = 0;
                if i < right {
                    r = radius[2 * rightmost - i].min(right - i - 1);
                }
                while extended[i - r - 1] == extended[i + r + 1] {
                    r += 1;
                }
                if i + r > right {
                    rightmost = i;
                }
                radius[i] = r;
                &s[(i - r - 1) / 2..(i + r - 1) / 2]
            })
            .max_by_key(|&s| s.len())
            .unwrap_or(&[])
            .into_iter()
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "aba");
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
}
