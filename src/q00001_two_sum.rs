use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let value_to_index = nums.iter().zip(0..nums.len() as i32).collect::<HashMap<_, _>>();
        for (i, &v) in nums.iter().enumerate() {
            match value_to_index.get(&(target - v)) {
                Some(&index) if (i as i32) != index => return vec![(i as i32), index],
                _ => continue
            };
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 2, 3], 4), vec![0, 1]);
    }
}