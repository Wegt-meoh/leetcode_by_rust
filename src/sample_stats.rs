pub struct Solution {}

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let n = {
            let mut _n: i64 = 0;
            for c in count.iter() {
                _n += *c as i64;
            }

            _n
        };

        let minimum = {
            let mut _m = 0;
            for (index, c) in count.iter().enumerate() {
                if *c > 0 {
                    _m = index;
                    break;
                }
            }
            _m as f64
        };

        let maximum = {
            let mut _m = 0;
            for i in (0..=255).rev() {
                if count[i] > 0 {
                    _m = i;
                    break;
                }
            }
            _m as f64
        };

        let mean = {
            let mut sum: i64 = 0;
            for (index, c) in count.iter().enumerate() {
                if *c > 0 {
                    sum += *c as i64 * index as i64;
                }
            }

            sum as f64 / n as f64
        };

        let median = {
            if n & 1 == 0 {
                let target = n / 2;
                let target_1 = target - 1;
                let mut _m: f64 = 0.0;
                let mut sum: i64 = 0;

                for (index, c) in count.iter().enumerate() {
                    if *c > 0 {
                        sum += *c as i64;

                        if sum >= target_1 + 1 && _m == 0.0 {
                            _m += index as f64;
                        }

                        if sum >= target + 1 {
                            _m += index as f64;
                            break;
                        }
                    }
                }

                _m / 2.0
            } else {
                let mut _m = 0;
                let target = n / 2;
                let mut sum: i64 = 0;
                for (index, c) in count.iter().enumerate() {
                    if *c > 0 {
                        sum += *c as i64;
                        if sum >= target + 1 {
                            _m = index;
                            break;
                        }
                    }
                }
                _m as f64
            }
        };

        let mode = {
            let mut _m: f64 = 0.0;
            let mut max_time = 0;
            for (index, c) in count.iter().enumerate() {
                if *c > max_time {
                    max_time = *c;
                    _m = index as f64;
                }
            }

            _m
        };

        vec![minimum, maximum, mean, median, mode]
    }
}

// 我们对 0 到 255 之间的整数进行采样，并将结果存储在数组 count 中：count[k] 就是整数 k 在样本中出现的次数。

// 计算以下统计数据:

// minimum ：样本中的最小元素。
// maximum ：样品中的最大元素。
// mean ：样本的平均值，计算为所有元素的总和除以元素总数。
// median ：
// 如果样本的元素个数是奇数，那么一旦样本排序后，中位数 median 就是中间的元素。
// 如果样本中有偶数个元素，那么中位数median 就是样本排序后中间两个元素的平均值。
// mode ：样本中出现次数最多的数字。保众数是 唯一 的。
// 以浮点数数组的形式返回样本的统计信息 [minimum, maximum, mean, median, mode] 。与真实答案误差在 10-5 内的答案都可以通过。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/statistics-from-a-large-sample
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    println!("{}", i32::MAX)
}
