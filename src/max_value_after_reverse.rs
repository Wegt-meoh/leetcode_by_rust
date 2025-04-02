use std::{fs::File, io::Read};

pub struct Solution {}

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let value1 = {
            let mut t = 0;
            for i in 0..nums.len() - 1 {
                t += Self::abs(nums[i], nums[i + 1]);
            }
            t
        };

        let mut result = value1;

        for i in 1..nums.len() - 1 {
            result = result
                .max(value1 - Self::abs(nums[i], nums[i + 1]) + Self::abs(nums[0], nums[i + 1]));
        }

        for i in 1..nums.len() - 1 {
            result = result.max(
                value1 - Self::abs(nums[i], nums[i - 1])
                    + Self::abs(nums[nums.len() - 1], nums[i - 1]),
            );
        }

        let mut min_item = i32::MAX;
        let mut max_item = i32::MIN;
        for i in 0..nums.len() - 1 {
            let x = nums[i];
            let y = nums[i + 1];
            max_item = max_item.max(x.min(y));
            min_item = min_item.min(x.max(y));
        }

        result = result.max(value1 + 2 * (max_item - min_item));

        result
    }

    fn abs(a: i32, b: i32) -> i32 {
        if a >= b {
            a - b
        } else {
            b - a
        }
    }
}

// 给你一个整数数组 nums 。「数组值」定义为所有满足 0 <= i < nums.length-1 的 |nums[i]-nums[i+1]| 的和。

// 你可以选择给定数组的任意子数组，并将该子数组翻转。但你只能执行这个操作 一次 。

// 请你找到可行的最大 数组值 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/reverse-subarray-to-maximize-array-value
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let nums = [2, 3, 1, 5, 4].to_vec();
    let ans = 10;
    let result = Solution::max_value_after_reverse(nums);
    assert!(ans == result, "ans = {ans}, result = {result}");

    let nums = [2, 4, 9, 24, 2, 1, 10].to_vec();
    let ans = 68;
    let result = Solution::max_value_after_reverse(nums);
    assert!(ans == result, "ans = {ans}, result = {result}");
}

#[test]
fn test1() -> std::io::Result<()> {
    let mut file = File::open("./src/input_data/max_value_after_reverse.input.data")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let nums = buf
        .split(",")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect();
    let ans = 1988832659;
    let result = Solution::max_value_after_reverse(nums);
    assert!(ans == result, "ans = {ans}, result = {result}");
    Ok(())
}
