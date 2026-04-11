// time  : O(log min(n, m))
// space : O(1)
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let mut l = 0;
        let mut step = m;
        while step != 0 {
            let half = step / 2;
            let i = l + half;
            let j = (m + n) / 2 - i;
            if nums2[j - 1] > nums1[i] {
                l = i + 1;
                step -= half + 1;
            } else {
                step = half;
            }
        }

        let i = l;
        let j = (m + n) / 2 - i;
        let l1 = nums1.get(i.wrapping_sub(1)).copied().unwrap_or(i32::MIN);
        let l2 = nums2.get(j.wrapping_sub(1)).copied().unwrap_or(i32::MIN);
        let r1 = nums1.get(i).copied().unwrap_or(i32::MAX);
        let r2 = nums2.get(j).copied().unwrap_or(i32::MAX);
        let l = l1.max(l2);
        let r = r1.min(r2);
        if (n + m) % 2 == 1 {
            r as f64
        } else {
            (l + r) as f64 / 2.
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);

    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.);
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![1]), 1.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3]), 2.);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4]), 2.5);
}
