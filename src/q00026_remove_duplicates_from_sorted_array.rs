pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut shift_to_index = 0;
            let mut next_index = 0;

            while next_index < nums.len() {
                let cur_val = nums.get(shift_to_index).unwrap();
                let next_val = nums.get(next_index).unwrap();
                if *cur_val != *next_val {
                    nums[shift_to_index + 1] = *next_val;
                    shift_to_index += 1;
                }
                next_index += 1
            }
            nums.resize(shift_to_index + 1, 0);
            (shift_to_index + 1) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_vector() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
    }

    #[test]
    fn test_sorted_vector() {
        let mut v = vec![1, 1, 2, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut v), 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_sorted_vector_all_ones() {
        let mut v = vec![1, 1, 1];
        assert_eq!(Solution::remove_duplicates(&mut v), 1);
        assert_eq!(v, vec![1]);
    }
}
