impl Solution {
    // time  : O(n)
    // space : O(n)
    pub fn convert(s: String, num_rows: i32) -> String {
        let n = num_rows as usize;
        if n == 1 {
            return s;
        }

        let s: Vec<char> = s.chars().collect();
        let mut rows = vec![String::with_capacity(s.len().div_ceil(n - 1)); n];
        for (&ch, i) in s.iter().zip((0..n).chain((1..n - 1).rev()).cycle()) {
            rows[i].push(ch);
        }
        rows.concat()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
    assert_eq!(Solution::convert("A".to_string(), 1), "A");
}
