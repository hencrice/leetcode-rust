pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x < 0;
        let mut v = x.abs();

        let mut rev_x: i32 = 0;

        while v > 0 {
            let digit = v % 10;

            match rev_x.checked_mul(10) {
                Some(res) => rev_x = res,
                None => return 0,
            }

            match rev_x.checked_add(digit) {
                Some(res) => rev_x = res,
                None => return 0
            }

            v = v / 10;
        }

        if is_negative {-rev_x} else {rev_x}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_positive() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn reverse_negative() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn reverse_overflows() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}