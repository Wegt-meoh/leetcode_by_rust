pub struct Solution;
const M: i64 = 10_i64.pow(9) + 7;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

        for line in edges.iter() {
            graph[line[0] as usize].push(line[1] as usize);
            graph[line[1] as usize].push(line[0] as usize);
        }

        let depth = Self::dfs(&graph, 1, 1);
        Self::qpow(2, (depth - 1) as i64) as i32
    }

    fn dfs(graph: &[Vec<usize>], curr: usize, prev: usize) -> i32 {
        let mut max_count = 0;

        for next in graph[curr].iter() {
            if *next == prev {
                continue;
            }
            max_count = max_count.max(Self::dfs(graph, *next, curr) + 1);
        }

        max_count
    }

    fn qpow(mut x: i64, mut y: i64) -> i64 {
        let mut res = 1;
        while y > 0 {
            if y & 1 == 1 {
                res = (res * x) % M;
            }
            x = (x * x) % M;
            y >>= 1;
        }
        res
    }
}
