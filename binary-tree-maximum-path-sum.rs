// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None
//         }
//     }
// }

// -----------------------------------------------------------       DFS Solution     ---------------------------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = i32::MIN;

        Self::dfs(&root, &mut answer);

        answer
    }

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        answer: &mut i32,
    ) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.as_ref().unwrap().borrow();

        let left = Self::dfs(&node.left, answer).max(0);
        let right = Self::dfs(&node.right, answer).max(0);

        let path_through_node = node.val + left + right;
        *answer = (*answer).max(path_through_node);

        node.val + left.max(right)
    }
}

// -----------------------------------------------------------       DP Solution     ---------------------------------------------------------------------
// use std::cell::RefCell;
// use std::rc::Rc;

// impl Solution {
//     pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut map = HashMap::new();

//         Self::helper(root, &mut map, false)
//     }

//     fn helper(
//         root: Option<Rc<RefCell<TreeNode>>>,
//         map: &mut HashMap<(*const RefCell<TreeNode>, bool), i32>,
//         lead_chosen: bool,
//     ) -> i32 {
//         let root = match root {
//             Some(root) => root,
//             None => {
//                 return -30_000_000;
//             }
//         };

//         // our state is (root, lead_chosen)
//         let key = (Rc::as_ptr(&root), lead_chosen);

//         if let Some(&x) = map.get(&key) {
//             return x;
//         }

//         let node = root.borrow();

//         let val = node.val;

//         let left_node = node.left.clone();
//         let right_node = node.right.clone();

//         drop(node);

//         let result;

//         if !lead_chosen {
//             let left = Self::helper(left_node.clone(), map, true).max(0);
//             let right = Self::helper(right_node.clone(), map, true).max(0);

//             let take_as_lead = val + left + right;

//             let leave = Self::helper(left_node, map, false)
//                 .max(Self::helper(right_node, map, false));

//             result = take_as_lead.max(leave);
//         } else {
//             let left = Self::helper(left_node, map, true).max(0);
//             let right = Self::helper(right_node, map, true).max(0);

//             result = val + left.max(right);
//         }

//         map.insert(key, result);

//         result
//     }
// }
