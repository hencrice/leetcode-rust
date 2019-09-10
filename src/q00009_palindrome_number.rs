pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else if x == 0 {
            true
        } else if x % 10 == 0 {
            false
        } else {
            let mut v = x;
            let mut digits = vec![];
            while v > 0 {
                let digit = v % 10;
                digits.push(digit);
                v /= 10;
            }

            let middle_index = if digits.len() % 2 == 0 {
                digits.len() / 2
            } else {
                digits.len() / 2 + 1
            };
            let left_half = &digits[0..middle_index];
            let right_half = &digits[middle_index..];
            let it = left_half.iter().zip(right_half.iter().rev());
            for (l, r) in it {
                if l != r {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::is_palindrome(0), true);
    }

    #[test]
    fn test_m100() {
        assert_eq!(Solution::is_palindrome(-100), false);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(1), true);
    }

    #[test]
    fn test_1331() {
        assert_eq!(Solution::is_palindrome(1331), true);
    }
}
