// time  : O(n)
// space : O(n)
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut count = [0; 101];
        for &i in nums.iter() {
            count[i as usize] += 1
        }

        count.iter_mut().fold(0usize, |sum, v| {
            *v += sum;
            *v
        });

        nums.iter().map(|&v| if v == 0 { 0 } else { count[v as usize - 1] } as i32).collect()
    }
}

struct Solution;

#[test]
fn main() {
    assert_eq!(Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]), vec![4, 0, 1, 1, 3]);
    assert_eq!(Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]), vec![2, 1, 0, 3]);
    assert_eq!(Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]), vec![0, 0, 0, 0]);
}
