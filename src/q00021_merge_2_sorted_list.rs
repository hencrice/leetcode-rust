use std::mem;

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// Note that if a ListNode is mutable then all the fields are mutable
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// https://doc.rust-lang.org/std/mem/fn.replace.html

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let (mut root, the_other_list) = match (&l1, &l2) {
            (Some(_), None) => return l1,
            (None, Some(_)) => return l2,
            (Some(n1), Some(n2)) => {
                if n1.val > n2.val {
                    (l1, l2)
                } else {
                    (l2, l1)
                }
            },
            _ => return None,
        };

        // // we will bind `prev_n` to another `&mut Option<Box<ListNode>>`
        // // so it needs to be mutable
        let prev_n = &mut root;
        let head_the_other_list = the_other_list;

        loop {
            let prev_n = prev_n.as_mut().unwrap();

            // the following won't work
            // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8a2de1aff8dcfc8c447a8d369c988234
            let (prev_n, head_the_other_list) = match (prev_n.next.as_ref(), head_the_other_list.as_ref()) {
                (Some(next), Some(head_node)) => {
                    if next.val > head_node.val {
                        (Some(next), Some(head_node))
                    } else {
                        (Some(head_node), Some(next))
                    }
                },
                (Some(_), None) => {
                    // no more connecting needs to be done
                    break;
                },
                (None, Some(_)) => {
                    prev_n.next = head_the_other_list;
                    break
                },
                (None, None) => break,
            };
        }

        // loop {
        //     let (prev_n, the_other_node) = match (prev_n.as_mut(), the_other_node.as_mut()) {
        //         (Some(prev_n), Some(the_other_node)) => {
        //             if prev_n.val > the_other_node.val {
        //                 (&mut prev_n.next, &mut Some(*the_other_node))
        //             } else {
        //                 let existing_next = mem::replace(&mut prev_n.next, Some(*the_other_node));
        //                 (&mut prev_n.next, &mut existing_next)
        //             }
        //         },
        //         (Some(_), None) => {
        //             // no more connecting needs to be done
        //             break
        //         },
        //         (None, Some(_)) => {
        //             prev_n.unwrap().next = the_other_node;
        //             break
        //         },
        //         (None, None) => break,
        //     };
        // }

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
