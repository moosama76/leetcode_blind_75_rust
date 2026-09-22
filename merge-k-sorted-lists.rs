// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::BTreeMap;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut freq = BTreeMap::new();
        for list in lists {
            let mut current = list;
            while let Some(node) = current {
                *freq.entry(node.val).or_insert(0) += 1;
                current = node.next;
            }
        }


        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        for (key, value) in freq {
            for _ in 0..value {
                tail.next = Some(Box::new(ListNode::new(key)));
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
