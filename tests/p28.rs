// time  : O(n + m)
// space : O(m)
impl Solution {
    pub fn str_str(s: String, p: String) -> i32 {
        if p.is_empty() {
            return 0;
        }

        let p = p.as_bytes();
        let m = p.len();
        let mut fail = vec![0; m + 1];
        for i in 1..m {
            let mut j = fail[i];
            while j != 0 && p[i] != p[j] {
                j = fail[j];
            }
            if p[i] == p[j] {
                j += 1;
            }
            fail[i + 1] = j;
        }

        let mut j = 0;
        for (i, &ch) in s.as_bytes().iter().enumerate() {
            while j != 0 && ch != p[j] {
                j = fail[j];
            }
            if ch == p[j] {
                j += 1;
            }
            if j == m {
                return (i + 1 - m) as i32;
            }
        }
        -1
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
}

#[test]
fn extra() {
    assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
}
