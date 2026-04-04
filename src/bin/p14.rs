impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.first().map_or(String::new(), |first| {
            first
                .char_indices()
                .find(|&(i, ch)| strs.iter().any(|s| s.get(i..).map_or(true, |s| !s.starts_with(ch))))
                .map_or(first.as_str(), |(i, _)| &first[..i])
                .to_string()
        })
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]),
        "fl"
    );
    assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
}
