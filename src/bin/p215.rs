impl Solution {
    fn medians_of_medians<T: Ord + Clone>(nums: &mut Vec<T>, k: usize) -> T {
        if nums.len() <= 5 {
            nums.sort_unstable();
            return nums[k].clone();
        }

        let mut medians: Vec<T> = nums
            .chunks_mut(5)
            .map(|chunk| {
                chunk.sort_unstable();
                chunk[chunk.len() / 2].clone()
            })
            .collect();
        let half = medians.len() / 2;
        let median = Self::medians_of_medians(&mut medians, half);

        let (mut less, other): (Vec<T>, Vec<T>) = nums.iter().cloned().partition(|x| *x < median);
        let (mut greater, equal): (Vec<T>, Vec<T>) = other.into_iter().partition(|x| *x > median);
        if k < less.len() {
            Self::medians_of_medians(&mut less, k)
        } else if k < less.len() + equal.len() {
            median
        } else {
            Self::medians_of_medians(&mut greater, k - less.len() - equal.len())
        }
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize;
        Self::medians_of_medians(&mut nums, k)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
