pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        let mut h = head;
        let mut p1 = h.as_mut().unwrap();

        while let Some(p2) = p1.next.as_mut() {
            if p1.val == p2.val {
                // skip the current p2 such that after
                // this loop, the Boxed value it refers
                // to will be deallocated
                p1.next = p2.next.take();
            } else {
                // p1 = p2; // borrow check failed. Asked on Stackoverflow:
                // https://stackoverflow.com/questions/58359684/mutable-borrow-starts-here-in-previous-iteration-of-loop-for-leetcode-problem
                p1 = p1.next.as_mut().unwrap();
            }
        }
        h
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0_1() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::delete_duplicates(None), None);
    }

    #[test]
    fn test_0_1_1() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        );
    }
}
