use crate::listnode::ListNode;

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut node1 = l1.unwrap();
        let mut node2 = l2.unwrap();
        let mut root = ListNode::new(0);
        let mut result = Solution::make_node(node1.val, node2.val, 0);
        root.next.get_or_insert(Box::new(result.0));
        let mut current = &mut root.next;
        while node1.next.is_some() || node2.next.is_some() {
            match current {
                None => break,
                Some(node) => {
                    node1 = node1.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    node2 = node2.next.or(Some(Box::new(ListNode::new(0)))).unwrap();

                    result = Solution::make_node(node1.val, node2.val, result.1);
                    node.next.get_or_insert(Box::new(result.0));

                    current = &mut node.next;
                }
            }
        }

        if result.1 > 0 {
            if let Some(node) = current {
                node.next.get_or_insert(Box::new(ListNode::new(result.1)));
            }
        }

        root.next
    }

    fn make_node(num1: i32, num2: i32, single: i32) -> (ListNode, i32) {
        let mut sum = num1 + num2 + single;
        (ListNode::new(sum % 10), sum / 10)
    }
}
