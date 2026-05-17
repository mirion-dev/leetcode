use std::sync::LazyLock;

use regex::Regex;

static REGEX_LOG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+):(start|end):(\d+)$").expect("Failed to compile regex."));

// time  : O(m)
// space : O(m)
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut res = vec![0; n as usize];
        let mut stack = vec![];
        let mut last_time = 0;
        for log in logs {
            if let Some(cap) = REGEX_LOG.captures(&log)
                && let Ok(id) = cap[1].parse::<usize>()
                && let start = &cap[2] == "start"
                && let Ok(mut time) = cap[3].parse::<usize>()
            {
                if start {
                    if let Some(&last_id) = stack.last() {
                        res[last_id] += (time - last_time) as i32;
                    }
                    stack.push(id);
                } else {
                    time += 1;
                    if let Some(last_id) = stack.pop() {
                        res[last_id] += (time - last_time) as i32;
                    }
                }
                last_time = time;
            }
        }

        res
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(
        Solution::exclusive_time(
            2,
            vec!["0:start:0".to_string(), "1:start:2".to_string(), "1:end:5".to_string(), "0:end:6".to_string()]
        ),
        vec![3, 4]
    );
    assert_eq!(
        Solution::exclusive_time(
            1,
            vec![
                "0:start:0".to_string(),
                "0:start:2".to_string(),
                "0:end:5".to_string(),
                "0:start:6".to_string(),
                "0:end:6".to_string(),
                "0:end:7".to_string()
            ]
        ),
        vec![8]
    );
    assert_eq!(
        Solution::exclusive_time(
            2,
            vec![
                "0:start:0".to_string(),
                "0:start:2".to_string(),
                "0:end:5".to_string(),
                "1:start:6".to_string(),
                "1:end:6".to_string(),
                "0:end:7".to_string()
            ]
        ),
        vec![7, 1]
    );
}
