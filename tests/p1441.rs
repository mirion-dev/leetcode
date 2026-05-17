// time  : O(n)
// space : O(n)
impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut i = 1;
        let mut res = vec![];
        for v in target {
            while i < v {
                res.push("Push".to_string());
                res.push("Pop".to_string());
                i += 1;
            }
            res.push("Push".to_string());
            i += 1;
        }
        res
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::build_array(vec![1, 3], 3), vec!["Push", "Push", "Pop", "Push"]);
    assert_eq!(Solution::build_array(vec![1, 2, 3], 3), vec!["Push", "Push", "Push"]);
    assert_eq!(Solution::build_array(vec![1, 2], 4), vec!["Push", "Push"]);
}
