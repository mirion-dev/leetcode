impl Solution {
    // time  : O(n)
    // space : O(1)
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut out = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[out] = nums[i];
                out += 1;
            }
        }
        out as i32
    }
}

struct Solution;

fn test(mut nums: Vec<i32>, val: i32, expected: Vec<i32>, ret_expected: i32) {
    let ret = Solution::remove_element(&mut nums, val);
    assert_eq!(nums[..ret as usize], expected);
    assert_eq!(ret, ret_expected);
}

fn main() {
    test(vec![3, 2, 2, 3], 3, vec![2, 2], 2);
    test(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 3, 0, 4], 5);
}
