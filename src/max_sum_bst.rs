use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

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

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (bool, i32, i32, i32) {
            if let Some(node) = node {
                let node_val = node.borrow().val;
                let left_node = node.borrow_mut().left.take();
                let right_node = node.borrow_mut().right.take();

                let (left_is_search_tree, left_result, left_min_val, left_max_val) =
                    dfs(left_node, ans);
                let (right_is_search_tree, right_result, right_min_val, right_max_val) =
                    dfs(right_node, ans);
                if left_is_search_tree
                    && right_is_search_tree
                    && left_max_val < node_val
                    && node_val < right_min_val
                {
                    let sum = left_result + right_result + node_val;
                    *ans = (*ans).max(sum);

                    // why need left_min_val.min(node_val) and right_max_val.max(node_val)
                    return (
                        true,
                        sum,
                        left_min_val.min(node_val),
                        right_max_val.max(node_val),
                    );
                }

                return (false, 0, 0, 0);
            }

            //why i32::MAX and i32::MIN
            (true, 0, i32::MAX, i32::MIN)
        }

        let mut ans = 0;
        dfs(root, &mut ans);
        ans
    }
}

// 给你一棵以 root 为根的 二叉树 ，请你返回 任意 二叉搜索子树的最大键值和。

// 二叉搜索树的定义如下：

// 任意节点的左子树中的键值都 小于 此节点的键值。
// 任意节点的右子树中的键值都 大于 此节点的键值。
// 任意节点的左子树和右子树都是二叉搜索树。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/maximum-sum-bst-in-binary-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
use std::collections::VecDeque;
pub fn build_tree_from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if v.len() == 0 {
        return None;
    }
    let mut queue = VecDeque::new();
    let root = Rc::new(RefCell::new(TreeNode::new(v[0].unwrap())));
    queue.push_back(Rc::clone(&root));
    for i in (1..v.len()).step_by(2) {
        let a = v[i];
        let b = v[i + 1];
        let node = queue.pop_front().unwrap();
        if let Some(a) = a {
            let left_node = Rc::new(RefCell::new(TreeNode::new(a)));
            node.borrow_mut().left = Some(Rc::clone(&left_node));
            queue.push_back(left_node);
        }

        if let Some(b) = b {
            let right_node = Rc::new(RefCell::new(TreeNode::new(b)));
            node.borrow_mut().right = Some(Rc::clone(&right_node));
            queue.push_back(right_node);
        }
    }
    Some(root)
}
#[test]
fn test() {
    let root = build_tree_from_vec(vec![
        Some(1),
        Some(4),
        Some(3),
        Some(2),
        Some(4),
        Some(2),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(4),
        Some(6),
    ]);
    let ans = 20;
    let result = Solution::max_sum_bst(root);
    assert!(ans == result, "ans={ans}, result={result}");

    let root = build_tree_from_vec(vec![Some(1), None, Some(10), Some(-5), Some(20)]);
    let ans = 25;
    let result = Solution::max_sum_bst(root);
    assert!(ans == result, "ans={ans}, result={result}");
}
