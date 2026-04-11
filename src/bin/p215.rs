use std::cmp::Ordering;

// time  : O(n)
// space : O(log n)
impl Solution {
    fn medians_of_medians<T: Ord>(nums: &mut [T], nth: usize) -> (&mut [T], &mut [T], &mut [T]) {
        const CHUNK_LEN: usize = 5;
        let n = nums.len();
        if n < CHUNK_LEN {
            nums.sort_unstable();
            let i = nums[..nth].iter().rposition(|x| x < &nums[nth]).map_or(0, |i| i + 1);
            let j = nums[nth..].iter().position(|x| x > &nums[nth]).map_or(n, |i| i + nth);
            let (less, other) = nums.split_at_mut(i);
            let (equal, greater) = other.split_at_mut(j - i);
            return (less, equal, greater);
        }

        let chunk_num = n.div_ceil(CHUNK_LEN);
        for i in 0..chunk_num {
            let start = i * CHUNK_LEN;
            let end = (start + CHUNK_LEN).min(n);
            nums[start..end].sort_unstable();
            nums.swap(i, start + (end - start) / 2);
        }
        let m = Self::medians_of_medians(&mut nums[..chunk_num], chunk_num / 2).0.len();
        nums.swap(0, m);

        let mut i = 0;
        let mut j = 0;
        let mut k = n;
        while j < k {
            match nums[j].cmp(&nums[i]) {
                Ordering::Less => {
                    nums.swap(i, j);
                    i += 1;
                    j += 1;
                }
                Ordering::Greater => {
                    k -= 1;
                    nums.swap(j, k);
                }
                Ordering::Equal => j += 1,
            }
        }

        if nth < i {
            Self::medians_of_medians(&mut nums[..i], nth)
        } else if nth < j {
            let (less, other) = nums.split_at_mut(i);
            let (equal, greater) = other.split_at_mut(j - i);
            (less, equal, greater)
        } else {
            Self::medians_of_medians(&mut nums[j..], nth - j)
        }
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let nth = nums.len() - k as usize;
        Self::medians_of_medians(&mut nums, nth).1[0]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);

    assert_eq!(Solution::find_kth_largest(vec![0, 1, 2, 3, 4, 5, 6], 1), 6);
}
