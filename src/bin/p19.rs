// time  : O(n)
// space : O(1)
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let head = ListNode { next: head, val: 0 };
        let mut seeker = &head;
        for _ in 0..n {
            seeker = seeker.next.as_ref()?;
        }

        let mut follower = &head;
        while let Some(node) = seeker.next.as_ref() {
            follower = follower.next.as_ref()?;
            seeker = node;
        }

        // INTENTIONAL: The definition of ListNode is bad. They should use NonNull instead of Box.
        #[allow(mutable_transmutes)]
        let follower: &mut ListNode = unsafe { std::mem::transmute(follower) };
        follower.next = follower.next.take()?.next;

        head.next
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

macro_rules! list {
    () => { None };
    ($v: expr $(, $args: expr)*) => { Some(Box::new(ListNode { val: $v, next: list![$($args), *] })) };
}

fn main() {
    assert_eq!(Solution::remove_nth_from_end(list![1, 2, 3, 4, 5], 2), list![1, 2, 3, 5]);
    assert_eq!(Solution::remove_nth_from_end(list![1], 1), list![]);
    assert_eq!(Solution::remove_nth_from_end(list![1, 2], 1), list![1]);
}
