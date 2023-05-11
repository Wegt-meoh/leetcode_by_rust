use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();
        if nums.len() < 3 {
            return result;
        }
        let mut i = 0;
        while i < nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }

            let target = -nums[i];
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            'a: while left < right {
                let curr = nums[left] + nums[right];
                if curr == target {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    while left + 1 < nums.len() && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    left += 1;

                    while right > 0 && right - 1 >= 0 && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    if right == 0 {
                        break 'a;
                    }
                    right -= 1;
                } else if curr < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }

            while i + 1 < nums.len() && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }

        result
    }
}

// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请

// 你返回所有和为 0 且不重复的三元组。

// 注意：答案中不可以包含重复的三元组。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/3sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let mut ans = [[-1, -1, 2].to_vec(), [-1, 0, 1].to_vec()].to_vec();
    let mut result = Solution::three_sum(nums);
    ans.sort();
    result.sort();
    assert!(ans == result, "ans = {:?}, result = {:?}", ans, result);

    let nums = vec![0, 1, 1];
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut result = Solution::three_sum(nums);
    ans.sort();
    result.sort();
    assert!(ans == result, "ans = {:?}, result = {:?}", ans, result);

    let nums = vec![0, 0, 0];
    let mut ans = [vec![0, 0, 0]].to_vec();
    let mut result = Solution::three_sum(nums);
    ans.sort();
    result.sort();
    assert!(ans == result, "ans = {:?}, result = {:?}", ans, result);
}

#[test]
fn test1() -> std::io::Result<()> {
    let mut file = File::open("./src/input_data/three_sum.input.data")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let str_vec: Vec<&str> = buf.split(",").collect();
    let nums = str_vec
        .iter()
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();
    let mut ans = [vec![0, 0, 0]].to_vec();
    let mut result = Solution::three_sum(nums);
    ans.sort();
    result.sort();
    assert!(ans == result, "ans = {:?}, result = {:?}", ans, result);
    Ok(())
}
