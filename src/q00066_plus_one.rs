pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        // kinda hope I'm allowed to mutate the input `digits`
        let mut carry = true;
        let mut res = vec![];

        for d in digits.iter().rev() {
            if carry {
                if d + 1 > 9 {
                    res.push(0);
                } else {
                    carry = false;
                    res.push(d + 1);
                }
            } else {
                res.push(*d);
            }
        }

        if carry {
            // carry happens on the most significant digit in the
            // input `digits` vector.
            res.push(1);
        }

        res.reverse();
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_carry_on_most_significant_digit() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_0() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }
}
