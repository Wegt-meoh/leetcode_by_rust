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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut fifo = std::collections::VecDeque::with_capacity(1024);
        let mut ans = Vec::with_capacity(1024);
        if root.is_some() {
            fifo.push_back(root.unwrap())
        }
        while let Some(t) = fifo.pop_front() {
            let mut t = t.borrow_mut();
            ans.push(t.val);
            if let Some(x) = t.left.take() {
                fifo.push_back(x);
            }
            if let Some(x) = t.right.take() {
                fifo.push_back(x);
            }
        }
        ans
    }
}

#[test]
fn test() {
    // 从上到下打印出二叉树的每个节点，同一层的节点按照从左到右的顺序打印。

    //

    // 例如:
    // 给定二叉树: [3,9,20,null,null,15,7],

    //     3
    // / \
    // 9  20
    //     /  \
    // 15   7
    // 返回：

    // [3,9,20,15,7]

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
}
