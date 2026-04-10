impl Solution {
    // time  : O(n m)
    // space : O(m)
    pub fn is_match(s: String, p: String) -> bool {
        enum Node {
            Next(char),
            Loop(char),
        }

        let p: Vec<char> = p.chars().collect();
        let mut nfa = Vec::with_capacity(p.len());
        let mut p = p.into_iter().peekable();
        while let Some(ch) = p.next() {
            nfa.push(match p.next_if_eq(&'*') {
                Some(_) => Node::Loop(ch),
                None => Node::Next(ch),
            })
        }

        let m = nfa.len() + 1;
        let mut epsilon = vec![m; m];
        let mut end = m;
        for (i, node) in nfa.iter().enumerate().rev() {
            if let Node::Next(_) = node {
                end = i + 1;
            }
            epsilon[i] = end;
        }

        let mut reach = vec![false; m];
        let mut next = vec![false; m];
        reach[0..epsilon[0]].fill(true);
        for ch in s.chars() {
            next.fill(false);
            for i in (0..m - 1).filter(|&i| reach[i]) {
                match nfa[i] {
                    Node::Next(p_ch) if p_ch == '.' || ch == p_ch => next[i + 1] = true,
                    Node::Loop(p_ch) if p_ch == '.' || ch == p_ch => next[i] = true,
                    _ => {}
                }
            }

            reach.fill(false);
            for i in (0..m).filter(|&i| next[i]) {
                if !reach[i] {
                    reach[i..epsilon[i]].fill(true);
                }
            }
        }
        reach[m - 1]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);

    assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
    assert_eq!(Solution::is_match("".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("a".to_string(), "".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*a".to_string()), true);
}
