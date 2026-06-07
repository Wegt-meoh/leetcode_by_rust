// Definition for a binary tree node.
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
pub struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut root_map = HashMap::new();
        descriptions.iter().for_each(|description| {
            let p = description[0];
            let c = description[1];
            let is_left = description[2] == 1;
            let p_node = node_map
                .entry(p)
                .or_insert_with(|| {
                    Rc::new(RefCell::new(TreeNode {
                        val: p,
                        left: None,
                        right: None,
                    }))
                })
                .clone();

            let c_node = node_map.entry(c).or_insert(Rc::new(RefCell::new(TreeNode {
                val: c,
                left: None,
                right: None,
            })));

            if is_left {
                p_node.borrow_mut().left = Some(c_node.clone());
            } else {
                p_node.borrow_mut().right = Some(c_node.clone());
            }

            root_map.entry(p).or_insert(true);
            root_map.insert(c, false);
        });

        let r = root_map
            .iter()
            .fold(1, |acc, (key, e)| if *e { *key } else { acc });
        node_map.get(&r).cloned()
    }
}
