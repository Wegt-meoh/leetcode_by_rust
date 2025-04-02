use std::collections::{BinaryHeap, HashSet};

pub struct Solution {}

#[derive(PartialEq, Eq, Debug)]
struct Group {
    index_vec: Vec<usize>,
    value_vec: Vec<i32>,
}

impl Group {
    fn new(index_vec: Vec<usize>, value_vec: Vec<i32>) -> Self {
        Self {
            index_vec,
            value_vec,
        }
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut other_sum = 0;
        for i in other.value_vec.iter() {
            other_sum += i;
        }

        let mut self_sum = 0;
        for i in self.value_vec.iter() {
            self_sum += i;
        }

        other_sum.cmp(&self_sum)
    }
}

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut history = HashSet::new();
        let first = {
            let mut index_vec = vec![];
            let mut value_vec = vec![];
            (0..mat.len()).for_each(|i| {
                index_vec.push(0);
                value_vec.push(mat[i][0]);
            });

            Group::new(index_vec, value_vec)
        };
        history.insert(first.index_vec.clone());
        heap.push(first);

        let mut result = 0;

        for _ in 0..k {
            let item = heap.pop();
            if let Some(item) = item {
                // println!("item={:?}", item);
                let index_vec = &item.index_vec;
                let value_vec = &item.value_vec;
                result = value_vec.iter().fold(0, |mut a, b| {
                    a += b;
                    a
                });

                for i in 0..index_vec.len() {
                    let mut next_index_vec = index_vec.clone();
                    next_index_vec[i] += 1;

                    if next_index_vec[i] < mat[0].len() && !history.contains(&next_index_vec) {
                        history.insert(next_index_vec.clone());
                        let mut next_value_vec = value_vec.clone();
                        next_value_vec[i] = mat[i][next_index_vec[i]];
                        heap.push(Group::new(next_index_vec, next_value_vec));
                    }
                }
            } else {
                break;
            }
        }

        result
    }

    pub fn kth_smallest_1(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut h = BinaryHeap::new();
        let mut s = HashSet::new();
        let n = mat.len();
        let m = mat[0].len() - 1;

        h.push((-mat.iter().map(|r| r[0]).sum::<i32>(), vec![0; n]));

        for _ in 1..k {
            let (c, r) = h.pop().unwrap();

            for i in 0..n {
                if r[i] < m {
                    let mut tmp = r.clone();
                    tmp[i] += 1;

                    if !s.contains(&tmp) {
                        s.insert(tmp.clone());
                        h.push((c + mat[i][tmp[i] - 1] - mat[i][tmp[i]], tmp));
                    }
                }
            }
        }

        -h.pop().unwrap().0
    }

    // 作者：DrackRamoray
    // 链接：https://leetcode.cn/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/solution/rust-by-drackramoray-1gtf/
    // 来源：力扣（LeetCode）
    // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
}

// 给你一个 m * n 的矩阵 mat，以及一个整数 k ，矩阵中的每一行都以非递减的顺序排列。

// 你可以从每一行中选出 1 个元素形成一个数组。返回所有可能数组中的第 k 个 最小 数组和。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let mat = vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]];
    let k = 7;
    let ans = 9;
    let result = Solution::kth_smallest(mat, k);
    assert!(ans == result, "ans={ans}, result={result}");
}
