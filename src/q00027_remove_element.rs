pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut to_move_index = nums.len() - 1;
        let mut to_remove_index = 0;

        nums.sort_unstable();

        // Made a mistake in the following condition. Did not consider
        // the fact that if there's no `val` to remove in `nums`, then
        // to_remove_index may go over the boundary.
        while to_remove_index < nums.len() {
            let v = nums[to_remove_index];
            if v == val {
                break;
            }
            to_remove_index += 1;
        }

        // start replacing
        // Made a mistake in the following condition. Did not consider
        // the fact that `val` to remove might be at the end of the `nums`.
        while to_remove_index < nums.len() {
            let v = nums[to_remove_index];
            if v != val {
                break;
            }

            // https://stackoverflow.com/questions/49547861/how-to-mutate-a-vectors-elements-while-iterating-through-it
            nums[to_remove_index] = nums[to_move_index];
            to_move_index -= 1;
            to_remove_index += 1;
        }

        nums.resize(to_move_index + 1, 0);

        (to_move_index + 1) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_elements_to_removed_at_the_end() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    }

    #[test]
    fn test_result_is_5() {
        assert_eq!(
            Solution::remove_element(&mut vec![3, 2, 2, 3, 1, 4, 7], 3),
            5
        );
    }

    #[test]
    fn test_nothing_to_remove() {
        assert_eq!(
            Solution::remove_element(&mut vec![4, 2, 2, 1, 1, 4, 7], 3),
            7
        );
    }
}
