pub struct Solution;

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n == 1 {
            return 0;
        }
        // dp[index][prev][curr] =  [0..=index] max score that is not compelted
        let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n];
        // prev_dp[prev][j]= max{dp[i-1][k=0..=j][prev]-max{prev_sum[i-1][k]-prev_sum[i-1][prev],0}}
        let mut prev_dp = vec![vec![0; n + 1]; n + 1];
        // prev_max[prev][j]=max{dp[i-1][k=j..=n][prev]}
        let mut prev_max = vec![vec![0; n + 1]; n + 1];
        let mut pre_sum = vec![vec![0; n + 1]; n];

        // init prefix sum
        for i in 0..n {
            for j in 0..=n {
                if j > 0 {
                    pre_sum[i][j] = pre_sum[i][j - 1] + grid[j - 1][i] as i64;
                }
            }
        }

        //  println!("{:#?}", pre_sum);

        for i in 1..n {
            // update prev_dp and prev_max for dp[i]
            for prev in 0..=n {
                prev_dp[prev][0] =
                    0.max(dp[i - 1][0][prev] - 0.max(pre_sum[i - 1][0] - pre_sum[i - 1][prev]));
                prev_max[prev][n] = dp[i - 1][n][prev];
                for k in 1..=n {
                    prev_dp[prev][k] = prev_dp[prev][k]
                        .max(prev_dp[prev][k - 1])
                        .max(dp[i - 1][k][prev] - 0.max(pre_sum[i - 1][k] - pre_sum[i - 1][prev]));

                    prev_max[prev][n - k] = prev_max[prev][n - k]
                        .max(prev_max[prev][n - k + 1])
                        .max(dp[i - 1][n - k][prev]);
                }
            }
            // calc for dp[i]
            for curr in 0..=n {
                for prev in 0..=n {
                    // update dp[i-1] first
                    if curr <= prev {
                        dp[i][prev][curr] = prev_max[prev][0] + pre_sum[i][prev] - pre_sum[i][curr];
                    } else {
                        dp[i][prev][curr] = dp[i][prev][curr]
                            .max(prev_dp[prev][curr] + pre_sum[i - 1][curr] - pre_sum[i - 1][prev])
                            .max(prev_max[prev][curr]);
                    }
                }
            }
        }

        println!("{:#?}", dp);

        let mut ans = 0;
        // for last column
        for prev in 0..=n {
            ans = ans.max(dp[n - 1][prev][0]);
            ans = ans.max(dp[n - 1][prev][n]);
        }
        ans
    }
}

#[test]
fn test() {
    use crate::slice_to_vec_recursive;
    let g = [
        [0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0],
        [0, 1, 0, 0, 0],
        [5, 0, 0, 3, 0],
        [0, 0, 0, 0, 2],
    ];
    let vg = slice_to_vec_recursive!(g;nested);
    let result = Solution::maximum_score(vg);
    let ans = 11;
    assert_eq!(result, ans, "test result={}, ans={}", result, ans);
}
