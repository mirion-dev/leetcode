impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut out = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[out] = nums[i];
                out += 1;
            }
        }
        out as i32
    }
}

struct Solution;

fn test(mut nums: Vec<i32>, expected: Vec<i32>, ret_expected: i32) {
    let ret = Solution::remove_duplicates(&mut nums);
    assert_eq!(nums[..ret as usize], expected);
    assert_eq!(ret, ret_expected);
}

fn main() {
    test(vec![1, 1, 2], vec![1, 2], 2);
    test(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4], 5);
}
