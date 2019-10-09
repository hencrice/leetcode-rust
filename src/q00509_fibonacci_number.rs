pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 1 {
            1
        } else if n == 0 {
            0
        } else {
            let (mut prev_1, mut prev_2) = (1, 0);
            for _ in 2..=n {
                let next = prev_2 + prev_1;
                prev_2 = prev_1;
                prev_1 = next;
            }
            prev_1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::fib(0), 0);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::fib(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::fib(4), 3);
    }
}
