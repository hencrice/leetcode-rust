// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// Note that if a ListNode is mutable then all the fields are mutable
#[cfg(test)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (n @ Some(_), None) => n,
            (None, n @ Some(_)) => n,
            (n1 @ Some(_), n2 @ Some(_)) => {
                let mut v1 = n1.unwrap();
                let mut v2 = n2.unwrap();

                if v1.val < v2.val {
                    v1.next = Solution::merge_two_lists(v1.next, Some(v2));
                    Some(v1)
                } else {
                    v2.next = Solution::merge_two_lists(Some(v1), v2.next);
                    Some(v2)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_empty_lists() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_l1_1_node_list() {
        assert_eq!(
            Solution::merge_two_lists(Some(Box::new(ListNode::new(30))), None),
            Some(Box::new(ListNode::new(30)))
        );
    }

    #[test]
    fn test_l2_1_node_list() {
        assert_eq!(
            Solution::merge_two_lists(None, Some(Box::new(ListNode::new(10)))),
            Some(Box::new(ListNode::new(10)))
        );
    }

    #[test]
    fn test_l1_smaller_than_l2() {
        assert_eq!(
            Solution::merge_two_lists(
                Some(Box::new(ListNode::new(1))),
                Some(Box::new(ListNode::new(2)))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        );
    }

    #[test]
    fn test_l2_smaller_than_l1() {
        assert_eq!(
            Solution::merge_two_lists(
                Some(Box::new(ListNode::new(2))),
                Some(Box::new(ListNode::new(1)))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        );
    }

    #[test]
    fn test_2_node_lists() {
        assert_eq!(
            Solution::merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                })),
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            ),
            // TODO: figure out another way to do this horrendous linked list construction
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            }))
        );
    }
}
