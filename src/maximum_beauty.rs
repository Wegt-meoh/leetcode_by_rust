use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn maximum_beauty(
        flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        // 舍既成之本，逐未竟之业
        let mut result = 0;
        let partial = partial as i64;
        let full = full as i64;
        let mut old_pertfect_count = 0;
        let mut sorted_flowers: Vec<_> = flowers
            .iter()
            .fold(vec![], |mut acc: Vec<i32>, x| {
                if *x < target {
                    acc.push(*x)
                } else {
                    old_pertfect_count += 1;
                };
                acc
            })
            .iter()
            .map(|e| *e as i64)
            .collect();
        sorted_flowers.sort_by(|a, b| b.cmp(a));
        let target = target as i64;
        // (ptr,sum)=suffix_sum
        // sum=sorted_flowers[ptr,ptr+1,...,n-1].sum()
        let mut suffix_sum: (usize, i64) = (0, sorted_flowers.iter().sum());
        let n = sorted_flowers.len() as i64;
        let mut curr_new_flower = new_flowers;
        let calc = |lowest: i64, new_perfect_count: i64| {
            (old_pertfect_count + new_perfect_count) * full + lowest * partial
        };

        // 边界情况，如果可种植的花数量足够多，把所有花园都变成完美的
        if target * n as i64 - suffix_sum.1 <= new_flowers {
            result = calc(0, n);
        }

        // 假定在下标i之前的所有花园都已经是完美的
        // plant flowers in i,i+1,...,n-1.来获取到最小不完善花园的最大值
        // 尝试让下标i这个花园也变成完美的，如果不行则结束循环
        for i in 0..sorted_flowers.len() {
            // the ptr=suffix_sum.0 means
            // 如果你在[ptr-1,ptr,...,n-1]中种植
            // 你会发现花朵数量不够将[ptr,ptr+1,...,n-1]种满成[ptr-1]一样高
            while suffix_sum.0 < i
                || sorted_flowers[suffix_sum.0] * (n - suffix_sum.0 as i64) - suffix_sum.1
                    > curr_new_flower
            {
                suffix_sum.1 -= sorted_flowers[suffix_sum.0];
                suffix_sum.0 += 1;
            }

            let res = curr_new_flower
                - (sorted_flowers[suffix_sum.0] * (n - suffix_sum.0 as i64) - suffix_sum.1);
            result = result.max(calc(
                (sorted_flowers[suffix_sum.0] + res / (n - suffix_sum.0 as i64)).min(target - 1),
                i as i64,
            ));

            let need = target - sorted_flowers[i];
            if need <= curr_new_flower {
                curr_new_flower -= need;
            } else {
                break;
            }
        }

        result
    }

    pub fn maximum_beauty_official(
        flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        let mut flowers = flowers.clone();
        let n = flowers.len();
        for flower in flowers.iter_mut() {
            if *flower > target {
                *flower = target;
            }
        }
        flowers.sort_by(|a, b| b.cmp(a));
        let mut sum: i64 = flowers.iter().map(|&flower| flower as i64).sum();
        let mut ans = 0;
        if (target as i64) * n as i64 - sum <= new_flowers {
            ans = full as i64 * n as i64;
        }
        let mut pre = 0;
        let mut ptr = 0;
        for i in 0..n {
            if i != 0 {
                pre += flowers[i - 1] as i64;
            }
            if flowers[i] == target {
                continue;
            }
            let mut rest = new_flowers - ((target as i64) * i as i64 - pre);
            if rest < 0 {
                break;
            }
            while !(ptr >= i && (flowers[ptr] as i64) * (n - ptr) as i64 - sum <= rest) {
                sum -= flowers[ptr] as i64;
                ptr += 1;
            }
            rest -= (flowers[ptr] as i64) * (n - ptr) as i64 - sum;
            ans = max(
                ans,
                full as i64 * i as i64
                    + partial as i64
                        * min(
                            flowers[ptr] as i64 + rest / (n - ptr) as i64,
                            (target - 1) as i64,
                        ),
            );
        }
        ans
    }
}
