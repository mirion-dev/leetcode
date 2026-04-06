impl Solution {
    pub fn is_perfect_square(n: i32) -> bool {
        if n <= 1 {
            return true;
        }

        let qr25 = [
            true, true, false, false, true, false, true, false, false, true, false, true, false, false, true, false,
            true, false, false, true, false, true, false, false, true,
        ];
        let qr32 = [
            true, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false,
            true, true, false, false, false, false, false, false, false, true, false, false, false, false, false,
            false,
        ];
        let qr77 = [
            true, true, false, false, true, false, false, false, false, true, false, true, false, false, true, true,
            true, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false,
            false, false, false, false, true, true, false, false, false, false, true, false, true, false, false, false,
            false, true, false, false, false, true, false, false, true, false, true, false, true, false, false, false,
            true, false, false, true, false, false, true, true, false, false, false, false, false,
        ];
        let qr117 = [
            true, true, false, false, true, false, false, false, false, true, true, false, false, true, false, false,
            true, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false,
            false, false, false, false, true, false, false, false, true, false, false, true, false, false, false,
            false, false, true, false, false, true, false, false, true, false, false, false, false, false, true, false,
            false, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
            false, true, false, true, true, false, false, false, false, false, true, false, true, true, false, false,
            true, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false,
            false, false, false, false, false, false, false,
        ];
        if !qr25[n as usize % 25] || !qr32[n as usize % 32] || !qr77[n as usize % 77] || !qr117[n as usize % 117] {
            return false;
        }

        let mut x = 1 << (33 - n.leading_zeros()) / 2;
        loop {
            let y = (x + n / x) / 2;
            if y >= x {
                return x * x == n;
            }
            x = y;
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
}
