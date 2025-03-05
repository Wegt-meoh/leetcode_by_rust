// 给你一棵树，树上有 n 个节点，按从 0 到 n-1 编号。树以父节点数组的形式给出，其中 parent[i] 是节点 i 的父节点。树的根节点是编号为 0 的节点。

// 树节点的第 k 个祖先节点是从该节点到根节点路径上的第 k 个节点。

// 实现 TreeAncestor 类：

// TreeAncestor（int n， int[] parent） 对树和父数组中的节点数初始化对象。
// getKthAncestor(int node, int k) 返回节点 node 的第 k 个祖先节点。如果不存在这样的祖先节点，返回 -1 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/kth-ancestor-of-a-tree-node
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

struct TreeAncestor {
    node_ancestor: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut node_ancestor = vec![vec![-1; 16]; n as usize];

        for (index, par) in parent.iter().enumerate() {
            node_ancestor[index][0] = *par;
        }

        for node in 1..n {
            for j in 1..16 {
                let temp_ancestor = node_ancestor[node as usize][j - 1];
                if temp_ancestor == -1 {
                    break;
                }
                node_ancestor[node as usize][j] = node_ancestor[temp_ancestor as usize][j - 1];
            }
        }

        return Self {
            node_ancestor: node_ancestor,
        };
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        if node == -1 {
            return -1;
        }

        if k == 0 {
            return node;
        }

        let mut current_k = k;
        let mut currrent_node = node;
        let mut step = 0;

        while current_k > 0 && currrent_node != -1 {
            if current_k & 1 == 1 {
                let next_node = self.node_ancestor[currrent_node as usize][step];
                currrent_node = next_node;
            }
            current_k >>= 1;
            step += 1;
        }

        currrent_node
    }
}
