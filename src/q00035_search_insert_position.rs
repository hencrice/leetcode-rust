pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &v) in nums.iter().enumerate() {
            if v < target {
                // v: 3, [4], 5
                continue;
            } else if v >= target {
                // 3, [4], v: 5
                return i as i32;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_existing_value() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_insert_in_the_middle() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_insert_at_the_end() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_insert_at_the_start() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
