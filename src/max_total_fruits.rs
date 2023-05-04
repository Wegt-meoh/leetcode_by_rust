pub struct Solution {}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let prefix_sum = Solution::get_prefix_sum(&fruits, k, start_pos);
        let mut result = 0;
        // println!("prefix_sum = {:?}", prefix_sum);

        for i in 0..=k / 2 {
            let right = start_pos + i;
            let left = (right - k + i).max(0);
            if left == 0 {
                result = result.max(prefix_sum[right as usize]);
            } else {
                result = result.max(prefix_sum[right as usize] - prefix_sum[left as usize - 1]);
            }
        }

        for i in 0..=k / 2 {
            let left = (start_pos - i).max(0);
            let right = left + k - i;
            if left == 0 {
                result = result.max(prefix_sum[right as usize]);
            } else {
                result = result.max(prefix_sum[right as usize] - prefix_sum[left as usize - 1]);
            }
        }

        println!("result = {}", result);

        result
    }

    fn get_prefix_sum(a: &Vec<Vec<i32>>, k: i32, start_pos: i32) -> Vec<i32> {
        let max_index = a.last().unwrap()[0].max(start_pos + k);
        let mut prefix_sum = Vec::with_capacity(max_index as usize + 1);
        let mut prev_value = 0;
        let mut curr_index = 0;
        for i in 0..=max_index {
            if curr_index < a.len() && i == a[curr_index][0] {
                prev_value += a[curr_index][1];
                curr_index += 1;
            }
            prefix_sum.push(prev_value);
        }
        prefix_sum
    }
}

#[test]
fn test() {
    // 在一个无限的 x 坐标轴上，有许多水果分布在其中某些位置。给你一个二维整数数组 fruits ，其中 fruits[i] = [positioni, amounti] 表示共有 amounti 个水果放置在 positioni 上。fruits 已经按 positioni 升序排列 ，每个 positioni 互不相同 。

    // 另给你两个整数 startPos 和 k 。最初，你位于 startPos 。从任何位置，你可以选择 向左或者向右 走。在 x 轴上每移动 一个单位 ，就记作 一步 。你总共可以走 最多 k 步。你每达到一个位置，都会摘掉全部的水果，水果也将从该位置消失（不会再生）。

    // 返回你可以摘到水果的 最大总数 。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/maximum-fruits-harvested-after-at-most-k-steps
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let fruits = vec![vec![2, 8], vec![6, 3], vec![8, 6]];
    let start_pos = 5;
    let k = 4;
    let ans = 9;
    let result = Solution::max_total_fruits(fruits, start_pos, k);
    assert!(ans == result);

    let fruits = vec![
        vec![0, 9],
        vec![4, 1],
        vec![5, 7],
        vec![6, 2],
        vec![7, 4],
        vec![10, 9],
    ];
    let start_pos = 5;
    let k = 4;
    let ans = 14;
    let result = Solution::max_total_fruits(fruits, start_pos, k);
    assert!(ans == result);

    let fruits = vec![vec![0, 3], vec![6, 4], vec![8, 5]];
    let start_pos = 3;
    let k = 2;
    let ans = 0;
    let result = Solution::max_total_fruits(fruits, start_pos, k);
    assert!(ans == result);

    let fruits = vec![vec![200000, 10000]];
    let start_pos = 0;
    let k = 200000;
    let ans = 10000;
    let result = Solution::max_total_fruits(fruits, start_pos, k);
    assert!(ans == result);
}
