pub struct Solution {}

impl Solution {
    pub fn mct_from_leaf_values(mut arr: Vec<i32>) -> i32 {
        let mut result = 0;

        while arr.len() >= 2 {
            let mut min_sum = i32::MAX;
            let mut min_index = 0;
            for i in 0..arr.len() - 1 {
                let item = arr[i] * arr[i + 1];
                if item < min_sum {
                    min_sum = item;
                    min_index = i;
                }
            }
            // println!("arr={:?}", arr);
            result += min_sum;
            arr.splice(
                min_index..=min_index + 1,
                [arr[min_index].max(arr[min_index + 1])],
            );
        }
        // println!("arr={:?}", arr);

        result
    }
}

// 给你一个正整数数组 arr，考虑所有满足以下条件的二叉树：

// 每个节点都有 0 个或是 2 个子节点。
// 数组 arr 中的值与树的中序遍历中每个叶节点的值一一对应。
// 每个非叶节点的值等于其左子树和右子树中叶节点的最大值的乘积。
// 在所有这样的二叉树中，返回每个非叶节点的值的最小可能总和。这个和的值是一个 32 位整数。

// 如果一个节点有 0 个子节点，那么该节点为叶节点。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/minimum-cost-tree-from-leaf-values
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let arr = vec![
        2, 3, 4, 1, 2, 4, 5, 2, 3, 2, 1, 2, 4, 2, 3, 2, 3, 1, 2, 3, 3, 2, 1, 2, 3, 3, 2, 1, 2, 3,
        12, 2, 3, 3,
    ];
    let ans = 335;
    let result = Solution::mct_from_leaf_values(arr);
    assert!(ans == result, "ans={}, result={}", ans, result);
}
