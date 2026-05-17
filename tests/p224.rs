// time  : O(n)
// space : O(n)
impl Solution {
    pub fn calculate(s: String) -> i32 {
        #[derive(Debug)]
        enum Token {
            Num(i32),
            LeftPar,
            RightPar,
            Add,
            Sub,
            Mul,
            Div,
            Neg,
        }

        impl Token {
            fn outer_prec(&self) -> u8 {
                match self {
                    Self::RightPar => 1,
                    Self::Add | Self::Sub => 10,
                    Self::Mul | Self::Div => 11,
                    Self::Neg => 20,
                    _ => u8::MAX,
                }
            }

            fn inner_prec(&self) -> u8 {
                match self {
                    Self::LeftPar | Self::RightPar => 0,
                    Self::Add | Self::Sub | Self::Neg => 10,
                    Self::Mul | Self::Div => 11,
                    _ => u8::MAX,
                }
            }

            fn arg_count(&self) -> u8 {
                match self {
                    Self::Add | Self::Sub | Self::Mul | Self::Div => 2,
                    Self::Neg => 1,
                    _ => 0,
                }
            }
        }

        let mut s = s.chars().peekable();
        let mut tokens = vec![];
        while let Some(&ch) = s.peek() {
            if ch.is_whitespace() {
                s.next();
                continue;
            }

            if ch.is_digit(10) {
                let mut value = 0;
                while let Some(digit) = s.peek().and_then(|&ch| ch.to_digit(10)) {
                    value = value * 10 + digit;
                    s.next();
                }
                tokens.push(Token::Num(value as i32));
                continue;
            }

            tokens.push(match ch {
                '(' => Token::LeftPar,
                ')' => Token::RightPar,
                '+' => Token::Add,
                '-' => match tokens.last() {
                    Some(Token::Num(_) | Token::RightPar) => Token::Sub,
                    _ => Token::Neg,
                },
                '*' => Token::Mul,
                '/' => Token::Div,
                _ => panic!("Unexpected character."),
            });
            s.next();
        }
        tokens.push(Token::RightPar);

        let mut op_stack = vec![Token::LeftPar];
        let mut num_stack = vec![];
        'a: for current in tokens {
            if let Token::Num(value) = current {
                num_stack.push(value);
                continue;
            }

            while let Some(top) = op_stack.pop() {
                if current.outer_prec() > top.inner_prec() {
                    if !matches!(current, Token::RightPar) {
                        op_stack.push(top);
                        op_stack.push(current);
                    }
                    continue 'a;
                }

                if top.arg_count() == 1
                    && let Some(a) = num_stack.pop()
                {
                    num_stack.push(-a)
                } else if top.arg_count() == 2
                    && let Some(b) = num_stack.pop()
                    && let Some(a) = num_stack.pop()
                {
                    num_stack.push(match top {
                        Token::Add => a + b,
                        Token::Sub => a - b,
                        Token::Mul => a * b,
                        Token::Div => a / b,
                        _ => unreachable!(),
                    })
                } else {
                    panic!("Missing operand.")
                }
            }
            assert!(!op_stack.is_empty(), "Missing left parenthesis.");
        }
        assert!(op_stack.is_empty(), "Missing right parenthesis.");
        assert!(num_stack.len() <= 1, "Missing operator.");
        num_stack.pop().expect("Missing operand.")
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
}
