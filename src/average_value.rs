pub struct Solution {}

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        for i in nums {
            if i % 3 == 0 && i & 1 == 0 {
                count += 1;
                sum += i;
            }
        }

        if count == 0 {
            return 0;
        } else {
            return sum / count;
        }
    }
}

// 给你一个由正整数组成的整数数组 nums ，返回其中可被 3 整除的所有偶数的平均值。

// 注意：n 个元素的平均值等于 n 个元素 求和 再除以 n ，结果 向下取整 到最接近的整数。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/average-value-of-even-numbers-that-are-divisible-by-three
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}
