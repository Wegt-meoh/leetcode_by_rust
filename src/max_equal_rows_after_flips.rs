use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = 0;
        let mut flag = vec![false; m];

        for i in 0..m - 1 {
            if flag[i] {
                continue;
            }
            flag[i] = true;
            let mut sum = 1;
            for j in i + 1..m {
                if !flag[j] && Self::is_equal_or_mirror(&matrix[i], &matrix[j]) {
                    sum += 1;
                    flag[j] = false;
                }
            }
            result = result.max(sum);
        }

        result
    }

    fn is_equal_or_mirror(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        if a.len() != b.len() {
            return false;
        }
        if a[0] == b[0] {
            for index in 0..a.len() {
                if a[index] != b[index] {
                    return false;
                }
            }
            true
        } else {
            for index in 0..a.len() {
                if a[index] == b[index] {
                    return false;
                }
            }
            true
        }
    }

    pub fn use_hash_map(mut matrix: Vec<Vec<i32>>) -> i32 {
        matrix.iter_mut().for_each(|x| {
            if x[0] == 0 {
                Self::reverse_vec(x);
            }
        });
        let mut hash_map = matrix.iter().fold(HashMap::new(), |mut a, b| {
            a.entry((*b).clone()).and_modify(|x| *x += 1).or_insert(1);
            a
        });

        let mut result = 0;
        for (_, v) in hash_map {
            result = result.max(v);
        }
        result
    }

    pub fn reverse_vec(a: &mut Vec<i32>) {
        for i in a {
            if *i == 0 {
                *i = 1;
            } else {
                *i = 0;
            }
        }
    }
}

// 给定 m x n 矩阵 matrix 。

// 你可以从中选出任意数量的列并翻转其上的 每个 单元格。（即翻转后，单元格的值从 0 变成 1，或者从 1 变为 0 。）

// 返回 经过一些翻转后，行与行之间所有值都相等的最大行数 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/flip-columns-for-maximum-number-of-equal-rows
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let matrix = vec![vec![0, 1], vec![1, 1]];
    let ans = 1;
    let result = Solution::max_equal_rows_after_flips(matrix);
    assert!(ans == result);

    let matrix = vec![vec![0, 1], vec![1, 0]];
    let ans = 2;
    let result = Solution::max_equal_rows_after_flips(matrix);
    assert!(ans == result);

    let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
    let ans = 2;
    let result = Solution::max_equal_rows_after_flips(matrix);
    assert!(ans == result);
}

#[test]
fn test1() {
    let matrix = vec![vec![0, 1], vec![1, 1]];
    let ans = 1;
    let result = Solution::use_hash_map(matrix);
    assert!(ans == result);

    let matrix = vec![vec![0, 1], vec![1, 0]];
    let ans = 2;
    let result = Solution::use_hash_map(matrix);
    assert!(ans == result);

    let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
    let ans = 2;
    let result = Solution::use_hash_map(matrix);
    assert!(ans == result);
}

#[test]
fn test_fn() {
    let a = vec![1, 0, 1, 0, 1];
    let b = vec![1, 0, 1, 0, 1];
    let c = vec![0, 1, 0, 1, 0];
    let d = vec![1, 1, 1, 1, 1];
    let result1 = Solution::is_equal_or_mirror(&a, &b);
    assert!(result1 == true);
    let result1 = Solution::is_equal_or_mirror(&a, &c);
    assert!(result1 == true);
    let result1 = Solution::is_equal_or_mirror(&a, &d);
    assert!(result1 == false);
}

#[test]
fn test_reverse() {
    let mut a = vec![1, 0, 1, 0, 1];
    let mut b = vec![0, 0, 1, 0, 1];
    let mut c = vec![0, 1, 0, 1, 0];
    Solution::reverse_vec(&mut a);
    assert!(vec![0, 1, 0, 1, 0] == a);
    Solution::reverse_vec(&mut b);
    assert!(vec![1, 1, 0, 1, 0] == b);
    Solution::reverse_vec(&mut c);
    assert!(vec![1, 0, 1, 0, 1] == c);
}
