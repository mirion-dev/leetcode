// time  : O(n)
// space : O(n)
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut num_stack = vec![];
        for token in tokens {
            match token.parse::<i32>() {
                Ok(value) => num_stack.push(value),
                Err(_) => {
                    if let Some(b) = num_stack.pop()
                        && let Some(a) = num_stack.pop()
                    {
                        num_stack.push(match token.as_str() {
                            "+" => a + b,
                            "-" => a - b,
                            "*" => a * b,
                            "/" => a / b,
                            _ => panic!("Unknown operator."),
                        })
                    } else {
                        panic!("Missing operand.");
                    }
                }
            }
        }
        assert!(num_stack.len() <= 1, "Missing operator.");
        num_stack.pop().expect("Missing operand.")
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(
        Solution::eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]),
        9
    );
    assert_eq!(
        Solution::eval_rpn(vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()]),
        6
    );
    assert_eq!(
        Solution::eval_rpn(vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string()
        ]),
        22
    );
}
