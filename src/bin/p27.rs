impl Solution {
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

fn test(mut nums: Vec<i32>, val: i32, expected: i32) {
    let k = Solution::remove_element(&mut nums, val);
    assert_eq!(k, expected);
}

fn main() {
    test(vec![3, 2, 2, 3], 3, 2);
    test(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5);
}
