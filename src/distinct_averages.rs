use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut hash_set = HashSet::new();

        while left < right {
            let a = nums[left];
            let b = nums[right];
            let ave = (a + b) / 2;
            let res = (a + b) & 1;
            hash_set.insert((ave, res));
            left += 1;
            right -= 1;
        }

        hash_set.len() as i32
    }
}

// 给你一个下标从 0 开始长度为 偶数 的整数数组 nums 。

// 只要 nums 不是 空数组，你就重复执行以下步骤：

// 找到 nums 中的最小值，并删除它。
// 找到 nums 中的最大值，并删除它。
// 计算删除两数的平均值。
// 两数 a 和 b 的 平均值 为 (a + b) / 2 。

// 比方说，2 和 3 的平均值是 (2 + 3) / 2 = 2.5 。
// 返回上述过程能得到的 不同 平均值的数目。

// 注意 ，如果最小值或者最大值有重复元素，可以删除任意一个。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/number-of-distinct-averages
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}
