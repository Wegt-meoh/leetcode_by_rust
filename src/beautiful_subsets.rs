use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ordered_nums = nums.clone();
        ordered_nums.sort();

        if n == 1 {
            return 1;
        }

        Self::dfs(&ordered_nums, 0, &mut Vec::new(), k)
    }

    fn dfs(nums: &Vec<i32>, curr: usize, his: &mut Vec<i32>, k: i32) -> i32 {
        if curr == nums.len() {
            return 0;
        }

        let mut cnt = 0;

        let x = nums[curr];
        if his.binary_search(&(x - k)).is_err() {
            cnt += 1;
            his.push(x);
            cnt += Self::dfs(nums, curr + 1, his, k);
            his.pop();
        }
        cnt += Self::dfs(nums, curr + 1, his, k);

        cnt
    }

    pub fn beautiful_subsets_nlogn(nums: Vec<i32>, k: i32) -> i32 {
        let mut arr = Vec::with_capacity(k as usize);

        for _ in 0..k as usize {
            arr.push(HashMap::new())
        }

        for num in nums.iter() {
            arr[(*num % k) as usize]
                .entry(num)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let mut result = 1;

        for a in arr.iter() {
            if a.is_empty() {
                continue;
            }
            let mut order_key_arr: Vec<_> = a.keys().map(|e| **e).collect();
            order_key_arr.sort();
            let n = order_key_arr.len();
            let mut dp = vec![(0, 0); n];

            dp[0].0 = 1;
            dp[0].1 = 2_i32.pow(*a.get(&order_key_arr[0]).unwrap()) - 1;

            for i in 1..n {
                let key = order_key_arr[i];
                let prev_key = order_key_arr[i - 1];
                let cnt = *a.get(&key).unwrap();
                dp[i].0 = dp[i - 1].0 + dp[i - 1].1;
                dp[i].1 = if key - prev_key != k {
                    dp[i].0 * (2_i32.pow(cnt) - 1)
                } else {
                    dp[i - 1].0 * (2_i32.pow(cnt) - 1)
                }
            }

            result *= dp[n - 1].1 + dp[n - 1].0
        }

        println!("{:?}", arr);

        result - 1
    }
}
