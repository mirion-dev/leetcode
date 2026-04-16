// time  : O(n)
// space : O(1)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().map_or(0, |w| w.len()) as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
    assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6);

    assert_eq!(Solution::length_of_last_word("".to_string()), 0);
    assert_eq!(Solution::length_of_last_word("word".to_string()), 4);
}
