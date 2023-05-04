use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            c: &Vec<i32>,
            num_map: &HashMap<i32, i32>,
            index: usize,
            curr: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            target: i32,
        ) {
            if target == 0 {
                result.push(curr.clone());
                return;
            }

            if index >= c.len() {
                return;
            }

            dfs(c, num_map, index + 1, curr, result, target);

            let max_time = *num_map.get(&c[index]).unwrap();
            for time in 1..=max_time {
                let new_target = target - c[index] * time;
                if new_target < 0 {
                    break;
                }
                for _ in 0..time {
                    curr.push(c[index]);
                }
                dfs(c, num_map, index + 1, curr, result, new_target);
                for _ in 0..time {
                    curr.pop();
                }
            }
        }

        let num_map: HashMap<i32, i32> = candidates.iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(*x).and_modify(|item| *item += 1).or_insert(1);
            acc
        });

        let mut result = vec![];
        let mut curr = vec![];
        let c = num_map.keys().map(|x| *x).collect();

        println!("num_map = {:?}", num_map);

        dfs(&c, &num_map, 0, &mut curr, &mut result, target);

        result
    }
}

#[test]
fn test() {
    // 给定一个可能有重复数字的整数数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

    // candidates 中的每个数字在每个组合中只能使用一次，解集不能包含重复的组合。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/4sjJUc
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let ans = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
    let result = Solution::combination_sum2(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(result == ans);
}

#[test]
fn test1() {
    let candidates = vec![2, 5, 2, 1, 2];
    let target = 5;
    let ans = vec![vec![1, 2, 2], vec![5]];
    let result = Solution::combination_sum2(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(result == ans);
}

#[test]
fn test2() {
    let candidates = vec![
        14, 6, 25, 9, 30, 20, 33, 34, 28, 30, 16, 12, 31, 9, 9, 12, 34, 16, 25, 32, 8, 7, 30, 12,
        33, 20, 21, 29, 24, 17, 27, 34, 11, 17, 30, 6, 32, 21, 27, 17, 16, 8, 24, 12, 12, 28, 11,
        33, 10, 32, 22, 13, 34, 18, 12,
    ];
    let target = 27;
    let ans = vec![
        vec![6, 6, 7, 8],
        vec![6, 7, 14],
        vec![6, 8, 13],
        vec![6, 9, 12],
        vec![6, 10, 11],
        vec![6, 21],
        vec![7, 8, 12],
        vec![7, 9, 11],
        vec![7, 20],
        vec![8, 8, 11],
        vec![8, 9, 10],
        vec![9, 9, 9],
        vec![9, 18],
        vec![10, 17],
        vec![11, 16],
        vec![13, 14],
        vec![27],
    ];
    let result = Solution::combination_sum2(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(result == ans);
}
