impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (0..m as usize).rev().peekable();
        let mut j = (0..n as usize).rev();
        let mut out = (0..(m + n) as usize).rev();
        while let Some(j) = j.next() {
            while let Some(i) = i.next_if(|&i| nums1[i] > nums2[j]) {
                nums1[out.next().expect("out of range")] = nums1[i];
            }
            nums1[out.next().expect("out of range")] = nums2[j];
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
