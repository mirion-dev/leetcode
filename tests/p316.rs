// time  : O(n)
// space : O(n)
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut last = [0; 128];
        for (i, &ch) in s.iter().enumerate() {
            last[ch as usize] = i;
        }

        let mut stack = vec![];
        let mut in_stack = [false; 128];
        for (i, &ch) in s.iter().enumerate() {
            if in_stack[ch as usize] {
                continue;
            }

            while let Some(&top) = stack.last()
                && top > ch
                && last[top as usize] > i
            {
                stack.pop();
                in_stack[top as usize] = false;
            }
            stack.push(ch);
            in_stack[ch as usize] = true;
        }

        stack.iter().map(|&ch| ch as char).collect()
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::remove_duplicate_letters("bcabc".to_string()), "abc");
    assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".to_string()), "acdb");
}

#[test]
fn extra() {
    assert_eq!(Solution::remove_duplicate_letters("cdadabcc".to_string()), "adbc");
    assert_eq!(Solution::remove_duplicate_letters("abacb".to_string()), "abc");
}
