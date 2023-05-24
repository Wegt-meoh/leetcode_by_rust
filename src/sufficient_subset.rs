pub struct Solution {}
// // Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(tree_node: &Option<Rc<RefCell<TreeNode>>>, limit: i32, sum: i32) -> i32 {
            if let Some(node) = tree_node {
                let node_val = node.borrow().val;

                if node.borrow().left == None && node.borrow().right == None {
                    return sum + node_val;
                }

                let left_value = {
                    if node.borrow().left == None {
                        i32::MIN
                    } else {
                        dfs(&node.borrow().left, limit, sum + node_val)
                    }
                };
                let right_value = {
                    if node.borrow().right == None {
                        i32::MIN
                    } else {
                        dfs(&node.borrow().right, limit, sum + node_val)
                    }
                };

                println!(
                    "node_val={}, left_value={}, right_val={}, limit={}",
                    node_val, left_value, right_value, limit
                );

                if left_value < limit {
                    node.borrow_mut().left.take();
                }

                if right_value < limit {
                    node.borrow_mut().right.take();
                }

                return left_value.max(right_value);
            }
            sum
        }

        let root_value = dfs(&root, limit, 0);
        if root_value < limit {
            return None;
        }

        root
    }
}

#[test]
fn test() {}

// pub struct Solution;

// use std::rc::Rc;
// use std::cell::RefCell;
// impl Solution {
//     pub fn sufficient_subset(
//         root: Option<Rc<RefCell<TreeNode>>>,
//         limit: i32,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         fn dfs(
//             root: Option<Rc<RefCell<TreeNode>>>,
//             limit: i32,
//             mut sum: i32,
//         ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
//             if let Some(root) = root {
//                 let mut child_sum = root.borrow().val;
//                 let mut left = dfs(root.borrow_mut().left.take(), limit, sum + child_sum);
//                 let mut right = dfs(root.borrow_mut().right.take(), limit, sum + child_sum);
//                 child_sum += if left.1 == i32::MIN && right.1 == i32::MIN {
//                     0
//                 } else {
//                     left.1.max(right.1)
//                 };
//                 if sum + child_sum < limit {
//                     (None, child_sum)
//                 } else {
//                     root.borrow_mut().left = left.0.take();
//                     root.borrow_mut().right = right.0.take();
//                     (Some(root), child_sum)
//                 }
//             } else {
//                 (None, i32::MIN)
//             }
//         }
//         dfs(root, limit, 0).0
//     }
// }
