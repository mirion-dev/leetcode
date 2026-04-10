impl Solution {
    // time  : O(n)
    // space : O(1)
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for d in digits.iter_mut().rev() {
            *d += 1;
            if *d < 10 {
                return digits;
            }
            *d -= 10;
        }
        digits.insert(0, 1);
        digits
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
}
