use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        let mut history = vec![vec![false; grid[0].len()]; grid.len()];
        let mut vec_dequeue: VecDeque<(i32, i32)> = VecDeque::from([(0, 0)]);
        let mut result = 0;
        let dirs: [(i32, i32); 8] = [
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
        ];
        while !vec_dequeue.is_empty() {
            let n = vec_dequeue.len();
            result += 1;
            for _ in 0..n {
                let item = vec_dequeue.pop_front().unwrap();
                if item.0 as usize == grid.len() - 1 && item.1 as usize == grid[0].len() - 1 {
                    return result;
                }
                for dir in dirs.iter() {
                    let next: (i32, i32) = (item.0 + dir.0, item.1 + dir.1);
                    if next.0 >= 0
                        && next.0 < grid.len() as i32
                        && next.1 >= 0
                        && next.1 < grid[0].len() as i32
                        && grid[next.0 as usize][next.1 as usize] == 0
                        && history[next.0 as usize][next.1 as usize] == false
                    {
                        history[next.0 as usize][next.1 as usize] = true;
                        vec_dequeue.push_back(next);
                    }
                }
            }
        }

        -1
    }
}

// 给你一个 n x n 的二进制矩阵 grid 中，返回矩阵中最短 畅通路径 的长度。如果不存在这样的路径，返回 -1 。

// 二进制矩阵中的 畅通路径 是一条从 左上角 单元格（即，(0, 0)）到 右下角 单元格（即，(n - 1, n - 1)）的路径，该路径同时满足下述要求：

// 路径途经的所有单元格都的值都是 0 。
// 路径中所有相邻的单元格应当在 8 个方向之一 上连通（即，相邻两单元之间彼此不同且共享一条边或者一个角）。
// 畅通路径的长度 是该路径途经的单元格总数。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/shortest-path-in-binary-matrix
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}
