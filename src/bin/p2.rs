impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        let mut carry = 0;
        while let (Some(node1), Some(node2)) = (p1, p2) {
            carry += node1.val + node2.val;
            tail = tail.next.insert(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            p1 = node1.next.as_ref();
            p2 = node2.next.as_ref();
        }

        let mut p = p1.or(p2);
        while let Some(node) = p {
            carry += node.val;
            tail = tail.next.insert(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            p = node.next.as_ref();
        }

        if carry != 0 {
            tail.next = Some(Box::new(ListNode::new(carry)));
        }

        head.next
    }
}

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

struct Solution;

macro_rules! list {
    () => { None };
    ($v: expr $(, $args: expr)*) => { Some(Box::new(ListNode { val: $v, next: list![$($args), *] })) };
}

fn main() {
    assert_eq!(Solution::add_two_numbers(list![2, 4, 3], list![5, 6, 4]), list![7, 0, 8]);
    assert_eq!(Solution::add_two_numbers(list![0], list![0]), list![0]);
    assert_eq!(Solution::add_two_numbers(list![9, 9, 9, 9, 9, 9, 9], list![9, 9, 9, 9]), list![8, 9, 9, 9, 0, 0, 0, 1]);
}
