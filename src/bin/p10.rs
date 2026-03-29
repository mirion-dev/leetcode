impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        enum Node {
            Next(char),
            Loop(char),
        }

        let mut nfa = Vec::with_capacity(p.len());
        let mut fmt = p.chars().peekable();
        while let Some(ch) = fmt.next() {
            nfa.push(match fmt.next_if_eq(&'*') {
                Some(_) => Node::Loop(ch),
                None => Node::Next(ch),
            })
        }

        let m = nfa.len();
        let mut epsilon = vec![0; m];
        let mut last = m;
        for (i, node) in nfa.iter().enumerate().rev() {
            if let Node::Next(_) = node {
                last = i;
            }
            epsilon[i] = last + 1;
        }

        let mut reach = vec![false; m + 1];
        let mut next = vec![false; m + 1];
        reach[0..epsilon[0]].fill(true);
        for ch in s.chars() {
            next.fill(false);
            for i in (0..m).filter(|&i| reach[i]) {
                match nfa[i] {
                    Node::Next(fmt_ch) if fmt_ch == '.' || ch == fmt_ch => next[i + 1] = true,
                    Node::Loop(fmt_ch) if fmt_ch == '.' || ch == fmt_ch => next[i] = true,
                    _ => {}
                }
            }

            reach.fill(false);
            reach[m] = next[m];
            for i in (0..m).filter(|&i| next[i]) {
                if !reach[i] {
                    reach[i..epsilon[i]].fill(true);
                }
            }
        }
        reach[m]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);

    assert_eq!(Solution::is_match("aa".to_string(), "a*a".to_string()), true);
}
