impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut head = ListNode::new(0);
        let mut p = &mut head;
        let mut carry = 0;
        // this overhead is acceptable because of the list costs
        while p1.is_some() || p2.is_some() || carry != 0 {
            if let Some(node) = p1 {
                carry += node.val;
                p1 = node.next.as_ref();
            }
            if let Some(node) = p2 {
                carry += node.val;
                p2 = node.next.as_ref();
            }

            p = p.next.insert(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
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
