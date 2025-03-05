use std::{collections::VecDeque, i32};

pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut ans = vec![];
        let mut is_over = false;
        let mut dp = vec![i32::MAX; n];
        let mut prev = vec![vec![]; n];

        for i in 0..n - 1 {
            graph[i].push(i + 1);
        }

        dp[0] = 0;

        for i in 1..n {
            prev[i].push(i - 1);
            dp[i] = dp[i - 1] + 1;
        }

        queries.iter().for_each(|q| {
            let x = q[0] as usize;
            let y = q[1] as usize;

            if x == 0 && y == n - 1 {
                is_over = true;
            }

            if is_over {
                ans.push(1);
            } else {
                graph[x].push(y);
                prev[y].push(x);

                for i in y..n {
                    for &v in prev[i].iter() {
                        dp[i] = dp[i].min(dp[v] + 1);
                    }
                }

                ans.push(dp[n - 1]);
            }
        });

        return ans;
    }

    fn dfs(graph: &Vec<Vec<usize>>, curr: usize, sum: i32, n: usize, dist: &mut Vec<i32>) -> i32 {
        if curr == n - 1 {
            return sum;
        }

        if sum >= dist[curr] {
            return i32::MAX;
        }

        dist[curr] = sum;

        let mut ans = i32::MAX;

        graph[curr].iter().for_each(|next| {
            ans = ans.min(Self::dfs(graph, *next, sum + 1, n, dist));
        });

        return ans;
    }

    fn bfs(graph: &Vec<Vec<usize>>, n: usize) -> i32 {
        let mut q = VecDeque::new();
        let mut dist = vec![-1; n];
        q.push_back(0);
        dist[0] = 0;

        while let Some(item) = q.pop_front() {
            for &next in graph[item].iter() {
                if dist[next] > 0 {
                    continue;
                }
                dist[next] = dist[item] + 1;
                q.push_back(next)
            }
        }

        dist[n - 1]
    }
}

impl Solution {
    // shortest_distance_after_queries
    pub fn ss(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![(0, true); n];
        let mut ans = vec![];
        let mut sum = n as i32 - 1;

        for i in 0..n - 1 {
            graph[i] = (i + 1, true);
        }

        queries.iter().for_each(|q| {
            let x = q[0] as usize;
            let y = q[1] as usize;

            if graph[x].1 && graph[y].1 {
                let mut curr = x;
                while graph[curr].0 < y {
                    let next = graph[curr].0;
                    sum -= 1;
                    graph[next].1 = false;
                    curr = next;
                }

                graph[x].0 = y;
            }

            ans.push(sum);
        });

        ans
    }
}
