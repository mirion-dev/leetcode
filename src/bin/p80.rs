impl Solution {
    // time  : O(n)
    // space : O(1)
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut out = 2;
        for i in 2..nums.len() {
            if nums[i] != nums[out - 2] {
                nums[out] = nums[i];
                out += 1;
            }
        }
        out.min(nums.len()) as i32
    }
}

struct Solution;

fn test(mut nums: Vec<i32>, expected: Vec<i32>, ret_expected: i32) {
    let ret = Solution::remove_duplicates(&mut nums);
    assert_eq!(nums[..ret as usize], expected);
    assert_eq!(ret, ret_expected);
}

fn main() {
    test(vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3], 5);
    test(vec![0, 0, 1, 1, 1, 1, 2, 3, 3], vec![0, 0, 1, 1, 2, 3, 3], 7);

    test(vec![], vec![], 0);
    test(vec![1], vec![1], 1);
    test(vec![1, 1], vec![1, 1], 2);
}
