pub struct Solution {}

struct Rec {
    max_ends_here: i32,
    max_so_far: i32,
    all_negative: bool,
    largest_neg: Option<i32>
}

impl Solution {
    // clarification questions:
    // 1. What about all negative-value vector?
    // A: Return the largest negative number.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let init = &mut Rec{
            max_ends_here: 0,
            max_so_far: 0,
            all_negative: true,
            largest_neg: None
        };
        let res = nums.iter().fold(init,
        |r, &x| {
            if x >= 0 {
                r.all_negative = false;
            }

            match r.largest_neg {
                None if x < 0 => r.largest_neg = Some(x),
                Some(v) if x < 0 && x > v => r.largest_neg = Some(x),
                _ => () // no-op
            }

            // essentially, if previous consecutive values are positive, we just keep
            // summing them up
            r.max_ends_here += x;
            if r.max_ends_here < 0 {
                // however, whenever it becomes negative, we reset to 0
                // and try to find the next sub array that conssits of contiguous
                // positive values
                r.max_ends_here = 0;
            }

            if r.max_ends_here > r.max_so_far {
                r.max_so_far = r.max_ends_here;
            }
            r
        });

        if res.all_negative {
            res.largest_neg.unwrap()
        } else {
            res.max_so_far
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_negative() {
       assert_eq!(Solution::max_sub_array(vec![-1, -2, -1]), -1); 
    }

    #[test]
    fn test_positive_num_at_the_end() {
        assert_eq!(Solution::max_sub_array(vec![-1,-2,-3,-4,1]), 1);
    }

    #[test]
    fn test_positive_num_at_the_front() {
        assert_eq!(Solution::max_sub_array(vec![1,-2,-3,-4]), 1);
    }

    #[test]
    fn test_base_case() {
        assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }
}
