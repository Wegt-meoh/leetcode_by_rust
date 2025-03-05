// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。

// 返回这三个数的和。

// 假定每组输入只存在恰好一个解。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/3sum-closest
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut temp_nums = nums.clone();
        temp_nums.sort();
        let mut result = None;

        for i in 0..nums.len() - 2 {
            let temp_target = target - temp_nums[i];
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = temp_nums[right] + temp_nums[left] + temp_nums[i];
                if result == None {
                    result = Some(sum);
                } else if i32::abs(target - result.unwrap()) > i32::abs(target - sum) {
                    result = Some(sum);
                }
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    return target;
                }
            }
        }

        result.unwrap()
    }
}

#[test]
fn test() {}
