pub struct Solution {}

struct Node {
    this: i32,
    edges: Vec<i32>,
}

impl Node {
    fn new(this: i32, edges: Vec<i32>) -> Self {
        Self { this, edges }
    }
}

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut node_vec = Vec::with_capacity(n as usize);
        for i in 1..=n {
            node_vec.push(Node::new(i, vec![]));
        }
        for edge in edges {
            let a = edge[0];
            let b = edge[1];

            node_vec[(a - 1) as usize].edges.push(b);
            node_vec[(b - 1) as usize].edges.push(a);
        }

        let mut sum = 0.0;

        fn dfs(
            node_vec: &Vec<Node>,
            curr: i32,
            from: i32,
            t: i32,
            target: i32,
            curr_sum: f64,
            sum: &mut f64,
        ) {
            if t == 0 {
                if curr == target {
                    *sum += curr_sum;
                }
                return;
            }

            let mut count = 0;
            for to in node_vec[(curr - 1) as usize].edges.iter() {
                if *to != from {
                    count += 1;
                }
            }

            if count == 0 {
                if curr == target {
                    *sum += curr_sum;
                }
                return;
            }

            let count = 1 as f64 / count as f64 * curr_sum;

            for to in node_vec[(curr - 1) as usize].edges.iter() {
                if *to != from {
                    dfs(node_vec, *to, curr, t - 1, target, count, sum)
                }
            }
        }

        dfs(&node_vec, 1, -1, t, target, 1.0, &mut sum);

        sum
    }
}

// 给你一棵由 n 个顶点组成的无向树，顶点编号从 1 到 n。青蛙从 顶点 1 开始起跳。规则如下：

// 在一秒内，青蛙从它所在的当前顶点跳到另一个 未访问 过的顶点（如果它们直接相连）。
// 青蛙无法跳回已经访问过的顶点。
// 如果青蛙可以跳到多个不同顶点，那么它跳到其中任意一个顶点上的机率都相同。
// 如果青蛙不能跳到任何未访问过的顶点上，那么它每次跳跃都会停留在原地。
// 无向树的边用数组 edges 描述，其中 edges[i] = [ai, bi] 意味着存在一条直接连通 ai 和 bi 两个顶点的边。

// 返回青蛙在 t 秒后位于目标顶点 target 上的概率。与实际答案相差不超过 10-5 的结果将被视为正确答案。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/frog-position-after-t-seconds
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let n = 7;
    let edges = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 7],
        vec![2, 4],
        vec![2, 6],
        vec![3, 5],
    ];
    let t = 2;
    let target = 4;
    let result = Solution::frog_position(n, edges, t, target);
    let ans = 0.16666666666666666;
    assert!(result == ans, "result={result}, ans={ans}");
}
