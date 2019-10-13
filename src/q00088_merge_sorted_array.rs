pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // idea:
        // Since nums1 contains contiguous empty slots at the end of the vector,
        // we'll start merging by picking values from the end of 2 vectors. That is,
        // given vector:
        // [1, 2, 3, 0, 0, 0]
        // [2, 5, 6]
        // we'll first compare the last element of nums1, 3, with the last element of
        // nums2, 6. And decided to place 6 at the end of nums1.

        // the following solution won't work because the iter() borrows immutably
        // but we also want to modify nums1 while looping through the iterator.
        // let nums1_rev = nums1[0..m as usize].iter().rev();
        // let nums2_rev = nums2[0..n as usize].iter().rev();
        // let mut to_fill_i: usize = (m+n-1).try_into().unwrap();
        // for (v1, v2) in nums1_rev.zip(nums2_rev) {
        //     if v1 > v2 {
        //         nums1[to_fill_i] = *v1;
        //         to_fill_i -= 1;
        //         continue
        //     }
        //     nums1[to_fill_i] = *v2;
        // }

        let total_len = (m + n) as usize;
        let mut nums1_i = m as usize;
        let mut nums2_i = n as usize;

        let mut placed = 0;
        while nums1_i > 0 && nums2_i > 0 {
            if nums1[nums1_i - 1] > nums2[nums2_i - 1] {
                nums1[total_len - placed - 1] = nums1[nums1_i - 1];
                nums1_i -= 1;
                placed += 1;
            } else {
                nums1[total_len - placed - 1] = nums2[nums2_i - 1];
                nums2_i -= 1;
                placed += 1;
            }
        }

        while nums2_i > 0 {
            // copy the rest of nums2 into nums1
            nums1[total_len - placed - 1] = nums2[nums2_i - 1];
            placed += 1;
            nums2_i -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_m_larger_than_n() {
        let nums_1 = &mut vec![1, 2, 3, 4, 0, 0, 0];
        Solution::merge(nums_1, 4, &mut vec![1, 2, 3], 3);
        assert_eq!(*nums_1, vec![1, 1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_n_larger_than_m() {
        let nums_1 = &mut vec![1, 2, 0, 0, 0, 0, 0];
        Solution::merge(nums_1, 2, &mut vec![1, 2, 3, 3, 4], 5);
        assert_eq!(*nums_1, vec![1, 1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_m_eq_n() {
        let nums_1 = &mut vec![1, 2, 3, 0, 0, 0];
        Solution::merge(nums_1, 3, &mut vec![1, 2, 4], 3);
        assert_eq!(*nums_1, vec![1, 1, 2, 2, 3, 4]);
    }

    #[test]
    fn test_all_nums1_values_larger_than_nums2() {
        let nums_1 = &mut vec![4, 5, 6, 0, 0, 0];
        Solution::merge(nums_1, 3, &mut vec![1, 2, 3], 3);
        assert_eq!(*nums_1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_same_values() {
        let nums_1 = &mut vec![1, 1, 1, 0, 0, 0];
        Solution::merge(nums_1, 3, &mut vec![1, 1, 1], 3);
        assert_eq!(*nums_1, vec![1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_n_larger_than_m_but_nums1_value_in_the_middle() {
        let nums_1 = &mut vec![4, 0, 0, 0, 0, 0];
        Solution::merge(nums_1, 1, &mut vec![1, 2, 3, 5, 6], 5);
        assert_eq!(*nums_1, vec![1, 2, 3, 4, 5, 6]);
    }
}
