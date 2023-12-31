#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_ref = &l1;
        let mut l2_ref = &l2;
        let mut head = Box::new(ListNode::new(0));
        let mut head_ref = &mut head;
        let mut carry = false;
        loop {
            match (l1_ref, l2_ref) {
                (Some(n1), Some(n2)) => {
                    let result = n1.val + n2.val + carry as i32;
                    head_ref.next = Some(Box::new(ListNode::new(result % 10)));
                    head_ref = head_ref.next.as_mut().unwrap();
                    carry = result > 9;
                    l1_ref = &n1.next;
                    l2_ref = &n2.next;
                }
                (Some(n1), None) => {
                    let result = n1.val + carry as i32;
                    head_ref.next = Some(Box::new(ListNode::new(result % 10)));
                    head_ref = head_ref.next.as_mut().unwrap();
                    carry = result > 9;
                    l1_ref = &n1.next;
                }
                (None, Some(n2)) => {
                    let result = n2.val + carry as i32;
                    head_ref.next = Some(Box::new(ListNode::new(result % 10)));
                    head_ref = head_ref.next.as_mut().unwrap();
                    carry = result > 9;
                    l2_ref = &n2.next;
                }
                (None, None) => {
                    if carry {
                        head_ref.next = Some(Box::new(ListNode::new(1)));
                    }
                    break;
                }
            }
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            })),
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1))),
                })),
                Some(Box::new(ListNode::new(0))),
            ),
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1))),
            })),
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::new(9))),
                Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode::new(9))),
                })),
            ),
            Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1))),
                })),
            })),
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode::new(9))),
                })),
                Some(Box::new(ListNode::new(9))),
            ),
            Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1))),
                })),
            })),
        );
    }
}
