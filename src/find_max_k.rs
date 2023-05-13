pub struct Solution {}

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        for i in 0..nums.len() {
            if nums[i] >= 0 {
                continue;
            }
            for j in 0..nums.len() {
                if nums[j] == -nums[i] {
                    result = result.max(nums[j])
                }
            }
        }
        result
    }
}

// 给你一个 不包含 任何零的整数数组 nums ，找出自身与对应的负数都在数组中存在的最大正整数 k 。

// 返回正整数 k ，如果不存在这样的整数，返回 -1 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}
