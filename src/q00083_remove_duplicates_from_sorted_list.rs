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
        // if head.is_none() {
        //     return None;
        // }
        // let mut h = head;
        // let mut p1 = h.as_mut().unwrap();
        // while let Some(p2) = p1.next.as_mut() {
        //     if p1.val == p2.val {
        //         p1.next = p2.next.take();
        //     } else {
        //         p1 = p1.next.as_mut().unwrap();
        //     }
        // }
        // h

        let mut cur_val = match &head {
            Some(n) => n.val,
            _ => return None,
        };

        let mut h = head;
        // study ln 40, 41, 43, 46. Especially the usage of
        // as_mut()
        let mut node = h.as_mut().unwrap();
        while let Some(next_n) = node.next.as_mut() {
            if next_n.val == cur_val {
                node.next = next_n.next.take();
            } else {
                cur_val = next_n.val;
                node = node.next.as_mut().unwrap();
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
