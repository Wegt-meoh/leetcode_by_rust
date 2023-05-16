pub struct Solution {}

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;

        if job_difficulty.len() < d {
            return -1;
        }

        let mut dp = vec![
            vec![0; job_difficulty.len() + 1],
            vec![i32::MAX; job_difficulty.len() + 1],
        ];

        //init dp[0]
        for i in 1..=job_difficulty.len() {
            dp[0][i] = job_difficulty[i - 1].max(dp[0][i - 1]);
        }

        for i in 2..=d {
            for task_num in i..=job_difficulty.len() {
                let mut min_diff = i32::MAX;
                for prev_task_num in i - 1..task_num {
                    min_diff = min_diff.min(
                        dp[i & 1][prev_task_num]
                            + Self::get_max(&job_difficulty, prev_task_num + 1, task_num),
                    );
                }
                dp[(i + 1) & 1][task_num] = min_diff;
            }
        }

        dp[(d + 1) & 1][job_difficulty.len()]
    }

    fn get_max(a: &Vec<i32>, left: usize, right: usize) -> i32 {
        let mut result = 0;
        for i in left - 1..right {
            result = result.max(a[i]);
        }
        result
    }
}

// 你需要制定一份 d 天的工作计划表。工作之间存在依赖，要想执行第 i 项工作，你必须完成全部 j 项工作（ 0 <= j < i）。

// 你每天 至少 需要完成一项任务。工作计划的总难度是这 d 天每一天的难度之和，而一天的工作难度是当天应该完成工作的最大难度。

// 给你一个整数数组 jobDifficulty 和一个整数 d，分别代表工作难度和需要计划的天数。第 i 项工作的难度是 jobDifficulty[i]。

// 返回整个工作计划的 最小难度 。如果无法制定工作计划，则返回 -1 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/minimum-difficulty-of-a-job-schedule
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let job_difficulty = vec![6, 5, 4, 3, 2, 1];
    let d = 2;
    let ans = 7;
    let result = Solution::min_difficulty(job_difficulty, d);
    assert!(ans == result, "ans={ans},result={result}");

    let job_difficulty = vec![9, 9, 9];
    let d = 4;
    let ans = -1;
    let result = Solution::min_difficulty(job_difficulty, d);
    assert!(ans == result, "ans={ans},result={result}");

    let job_difficulty = vec![1, 1, 1];
    let d = 3;
    let ans = 3;
    let result = Solution::min_difficulty(job_difficulty, d);
    assert!(ans == result, "ans={ans},result={result}");

    let job_difficulty = vec![7, 1, 7, 1, 7, 1];
    let d = 3;
    let ans = 15;
    let result = Solution::min_difficulty(job_difficulty, d);
    assert!(ans == result, "ans={ans},result={result}");

    let job_difficulty = vec![11, 111, 22, 222, 33, 333, 44, 444];
    let d = 6;
    let ans = 843;
    let result = Solution::min_difficulty(job_difficulty, d);
    assert!(ans == result, "ans={ans},result={result}");
}
