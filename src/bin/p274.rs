// time  : O(n)
// space : O(n)
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut count = vec![0; n + 1];
        for c in citations {
            count[(c as usize).min(n)] += 1;
        }

        let mut sum = 0;
        for (i, &val) in count.iter().enumerate().rev() {
            sum += val;
            if sum >= i {
                return i as i32;
            }
        }
        0
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
}
