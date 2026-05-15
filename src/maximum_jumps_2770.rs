pub struct Solution;
impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut graph = vec![vec![]; n];
        let mut count_graph = vec![-1; n];
        for (i, i_num) in nums.iter().enumerate() {
            for (j, j_num) in nums.iter().enumerate() {
                if j <= i {
                    continue;
                }

                let value = j_num - i_num;
                if -target <= value && value <= target {
                    graph[i].push(j);
                }
            }
        }

        count_graph[0] = 0;

        Solution::dfs(&graph, &mut count_graph, 0, 0, n - 1)
    }

    pub fn dfs(
        graph: &Vec<Vec<usize>>,
        count_graph: &mut Vec<i32>,
        step: i32,
        curr: usize,
        end: usize,
    ) -> i32 {
        if curr == end {
            return step;
        }

        let mut result = vec![-1];

        for &next in graph[curr].iter() {
            if count_graph[next] < step + 1 {
                count_graph[next] = step + 1;
                result.push(Solution::dfs(graph, count_graph, step + 1, next, end));
                count_graph[next] = -1;
            }
        }

        *result.iter().max().unwrap()
    }
}
