pub struct Solution {}

// #[derive(Debug, PartialEq, Eq)]
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
    pub fn del_nodes(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            to_delete: &Vec<i32>,
            canbe_result: bool,
            result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = node {
                let mut tr = node.borrow_mut();

                if to_delete.contains(&tr.val) {
                    tr.left = dfs(tr.left.take(), to_delete, true, result);
                    tr.right = dfs(tr.right.take(), to_delete, true, result);
                    return None;
                } else {
                    tr.left = dfs(tr.left.take(), to_delete, false, result);
                    tr.right = dfs(tr.right.take(), to_delete, false, result);

                    if canbe_result {
                        result.push(Some(Rc::clone(&node)));
                    }

                    return Some(Rc::clone(&node));
                }
            }
            None
        }

        dfs(root, &to_delete, true, &mut result);

        result
    }
}

// 给出二叉树的根节点 root，树上每个节点都有一个不同的值。

// 如果节点值在 to_delete 中出现，我们就把该节点从树上删去，最后得到一个森林（一些不相交的树构成的集合）。

// 返回森林中的每棵树。你可以按任意顺序组织答案。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/delete-nodes-and-return-forest
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}
