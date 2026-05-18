impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        match arr.windows(2).position(|w| w[0] >= w[1]) {
            Some(pos) if pos != 0 => arr[pos..].is_sorted_by(|&a, &b| a > b),
            _ => false,
        }
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
    assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
}

#[test]
fn extra() {
    assert_eq!(Solution::valid_mountain_array(vec![4, 4, 3, 2, 1]), false);
}
