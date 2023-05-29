pub struct Solution {}

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = vec![];
        let mut sum = 0;

        let mut count = vec![0; mat.len()];

        'a: loop {
            // println!("count={:?}", count);

            for (index, i) in count.iter().enumerate() {
                sum += mat[index][*i];
            }
            result.push(sum);
            sum = 0;

            let mut curr = count.len() - 1;
            count[curr] += 1;

            while count[curr] == mat[0].len() {
                if curr == 0 {
                    break 'a;
                }
                count[curr] = 0;
                count[curr - 1] += 1;
                curr -= 1;
            }
        }
        // println!("count={:?}", count);

        result.sort();

        result[k as usize - 1]
    }
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
