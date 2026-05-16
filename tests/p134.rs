// time  : O(n)
// space : O(1)
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut debt = 0;
        let mut start = 0;
        let mut balance = 0;
        for (i, (&g, c)) in gas.iter().zip(cost).enumerate() {
            balance += g - c;
            if balance < 0 {
                debt += balance;
                start = i + 1;
                balance = 0;
            }
        }
        if debt + balance >= 0 { start as i32 } else { -1 }
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}

#[test]
fn extra() {
    assert_eq!(Solution::can_complete_circuit(vec![1], vec![0]), 0);
    assert_eq!(Solution::can_complete_circuit(vec![0], vec![1]), -1);
    assert_eq!(Solution::can_complete_circuit(vec![3, 0, 0, 8, 0, 0, 4, 0, 0], vec![0, 2, 2, 0, 4, 5, 0, 1, 1]), 6);
}
