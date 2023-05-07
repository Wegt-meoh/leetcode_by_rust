use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let sum_range = (2, 1000);
        let target_sum_vec = {
            let mut res = vec![];
            let mut t = 60;
            while t <= 1000 {
                res.push(t);
                t += 60;
            }

            res
        };
        let mut result = 0;
        let mut time_map = HashMap::new();

        for i in time {
            for target in target_sum_vec.iter() {
                if target <= &i {
                    continue;
                }
                let other = target - i;
                if let Some(value) = time_map.get(&other) {
                    result += value;
                }
            }

            time_map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        result
    }

    pub fn method2(time: Vec<i32>) -> i32 {
        let mut cnt = [0; 60];
        let mut result = 0;

        time.iter().for_each(|item| {
            if item % 60 == 0 {
                result += cnt[0];
            } else {
                result += cnt[(60 - item % 60) as usize];
            }
            cnt[(item % 60) as usize] += 1;
        });

        result
    }
}

// 在歌曲列表中，第 i 首歌曲的持续时间为 time[i] 秒。

// 返回其总持续时间（以秒为单位）可被 60 整除的歌曲对的数量。形式上，我们希望下标数字 i 和 j 满足  i < j 且有 (time[i] + time[j]) % 60 == 0。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/pairs-of-songs-with-total-durations-divisible-by-60
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let time = vec![30, 20, 150, 100, 40];
    let ans = 3;
    let result = Solution::num_pairs_divisible_by60(time);
    assert!(ans == result);

    let time = vec![60; 3];
    let ans = 3;
    let result = Solution::num_pairs_divisible_by60(time);
    assert!(ans == result);
}

#[test]
fn test1() {
    let time = vec![30, 20, 150, 100, 40];
    let ans = 3;
    let result = Solution::method2(time);
    assert!(ans == result);

    let time = vec![60; 3];
    let ans = 3;
    let result = Solution::method2(time);
    assert!(ans == result);
}
