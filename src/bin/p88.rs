// time  : O(m + n)
// space : O(1)
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut out = (m + n) as usize;
        for &val in nums2.iter().rev() {
            while i != 0 && nums1[i - 1] > val {
                i -= 1;
                out -= 1;
                nums1[out] = nums1[i];
            }
            out -= 1;
            nums1[out] = val;
        }
    }
}

struct Solution;

fn test(mut nums1: Vec<i32>, m: i32, mut nums2: Vec<i32>, n: i32, expected: Vec<i32>) {
    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, expected);
}

fn main() {
    test(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3, vec![1, 2, 2, 3, 5, 6]);
    test(vec![1], 1, vec![], 0, vec![1]);
    test(vec![0], 0, vec![1], 1, vec![1]);
}
