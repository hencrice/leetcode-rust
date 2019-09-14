pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut root, mut the_other_n) = match (l1, l2) {
            (Some(l1), None) => return Some(l1),
            (None, Some(l2)) => return Some(l2),
            (Some(n1), Some(n2)) => {
                if n1.val > n2.val {
                    // TODO: Figure out a better way instead of unbox and box the value back
                    (Some(n1), &mut Some(Box::new(n2)))
                } else {
                    (Some(n2), &mut Some(Box::new(n1)))
                }
            },
            _ => return None,
        };

        // https://stackoverflow.com/questions/29672373/what-is-difference-between-mut-a-t-and-a-mut-t
        let mut prev_n = &mut root;

        loop {
            let (prev_n, the_other_n) = match (prev_n.unwrap().next, the_other_n) {
                (Some(n), Some(t_n)) => {
                    if n.val > t_n.val {
                        prev_n = &mut prev_n.unwrap().next;
                        (prev_n, the_other_n)
                    } else {
                        prev_n.unwrap().next = the_other_n;
                        the_other_n = &mut prev_n.unwrap().next;
                        prev_n = &mut prev_n.unwrap().next;
                        (prev_n, the_other_n)
                    }
                },
                (Some(_), None) => {
                    // no more connecting needs to be done
                    break
                },
                (None, _) => {
                    prev_n.unwrap().next = *the_other_n;
                    break
                },
            };
            ()
        }

        root
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_empty() {
        assert_eq!(true, true);
    }
}
