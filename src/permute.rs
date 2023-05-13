pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        fn dfs(nums: Vec<i32>, curr: Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if nums.len() == 0 {
                result.push(curr);
                return;
            }

            for i in 0..nums.len() {
                let mut _c = curr.clone();
                _c.push(nums[i]);
                let mut _nums = nums.clone();
                _nums.remove(i);
                dfs(_nums, _c, result);
            }
        }

        dfs(nums, vec![], &mut result);

        result
    }
}

// 给定一个不含重复数字的整数数组 nums ，返回其 所有可能的全排列 。可以 按任意顺序 返回答案。

#[test]
fn test() {}
