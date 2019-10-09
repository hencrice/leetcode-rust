pub struct Solution {}

impl Solution {
    // Correct but too slow with time complexity to be O(n).
    // pub fn climb_stairs(n: i32) -> i32 {
    //     if n == 2 {
    //         2
    //     } else if n == 1 {
    //         1
    //     } else {
    //         Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2)
    //     }
    // }
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 2 {
            2
        } else if n == 1 {
            1
        } else {
            // the DP solution is actually quite obvious
            // given the recursive formula on ln11
            let (mut prev_m1, mut prev_m2) = (2, 1);
            for _ in 3..=n {
                let next = prev_m2 + prev_m1;
                prev_m2 = prev_m1;
                prev_m1 = next;
            }
            prev_m1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_basic_4() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }

    #[test]
    fn test_basic_5() {
        // 221, 1121, 2111
        // 212, 1112,
        // 122, 1211
        // 1111
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
