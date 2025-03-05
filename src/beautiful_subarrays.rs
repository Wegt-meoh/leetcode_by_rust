use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut res = 0;
        let mut remain = nums[0];
        let mut count = HashMap::<i32, i64>::new();

        count.insert(nums[0], 1);

        for i in 0..n {
            remain ^= nums[i];

            count
                .entry(remain)
                .and_modify(|e| {
                    res += *e;
                    *e += 1
                })
                .or_insert(1);
        }

        res
    }
}
