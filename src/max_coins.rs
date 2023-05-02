pub struct Solution {}

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        nums.push(1);
        nums.insert(0, 1);
        let n = nums.len();

        let mut dp = vec![vec![0; n]; n];

        for length in 2..n {
            for begin in 0..n {
                let end = begin + length;
                if end >= n {
                    break;
                }

                for k in begin + 1..end {
                    dp[begin][end] = dp[begin][end]
                        .max(nums[begin] * nums[k] * nums[end] + dp[begin][k] + dp[k][end]);
                }
            }
        }

        return dp[0][n - 1];
    }
}

#[test]
fn test() {
    let input = vec![3, 1, 5, 8];
    let ans = 167;
    let res = Solution::max_coins(input);
    assert!(res == ans);

    let input = vec![1, 5];
    let ans = 10;
    let res = Solution::max_coins(input);
    assert!(res == ans);
}
