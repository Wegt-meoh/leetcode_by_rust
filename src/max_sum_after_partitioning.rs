pub struct Solution {}

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let k = k as usize;
        let mut dp = vec![0; n];

        let mut max_value = 0;
        for i in 0..k {
            max_value = max_value.max(arr[i]);
            dp[i] = max_value * (i as i32 + 1);
        }

        for i in k..n {
            let mut max_value = 0;
            for j in 1..=k {
                max_value = max_value.max(arr[i - j + 1]);
                dp[i] = dp[i].max(dp[i - j] + max_value * j as i32)
            }
        }

        // println!("dp={:?}",dp);

        dp[n - 1]
    }
}

#[test]
fn test() {
    let input = vec![1, 15, 7, 9, 2, 5, 10];
    let k = 3;
    let ans = 84;
    let result = Solution::max_sum_after_partitioning(input, k);
    println!("ans={}, result={}", ans, result);
    assert!(ans == result);

    let input = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];
    let k = 4;
    let ans = 83;
    let result = Solution::max_sum_after_partitioning(input, k);
    println!("ans={}, result={}", ans, result);
    assert!(ans == result);

    let input = vec![1];
    let k = 1;
    let ans = 1;
    let result = Solution::max_sum_after_partitioning(input, k);
    println!("ans={}, result={}", ans, result);
    assert!(ans == result);
}
