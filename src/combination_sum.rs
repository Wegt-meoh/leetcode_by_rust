pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            c: &Vec<i32>,
            target: i32,
            curr: &mut Vec<i32>,
            index: usize,
            result: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                result.push(curr.clone());
                return;
            }

            if index >= c.len() {
                return;
            }

            dfs(c, target, curr, index + 1, result);
            if target >= c[index] {
                curr.push(c[index]);
                dfs(c, target - c[index], curr, index, result);
                curr.pop();
            }
        }

        let mut result = vec![];
        let mut curr = vec![];

        dfs(&candidates, target, &mut curr, 0, &mut result);
        result
    }
}

#[test]
fn test() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let ans = vec![vec![7], vec![2, 2, 3]];
    let result = Solution::combination_sum(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(ans == result);

    let candidates = vec![2, 3, 5];
    let target = 8;
    let ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
    let result = Solution::combination_sum(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(ans == result);

    let candidates = vec![2];
    let target = 1;
    let ans: Vec<Vec<i32>> = vec![];
    let result = Solution::combination_sum(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(ans == result);

    let candidates = vec![1];
    let target = 1;
    let ans = vec![vec![1]];
    let result = Solution::combination_sum(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(ans == result);

    let candidates = vec![1];
    let target = 2;
    let ans = vec![vec![1, 1]];
    let result = Solution::combination_sum(candidates, target);
    println!("ans={:?}, result={:?}", ans, result);
    assert!(ans == result);
}
