// 给你一个下标从 0 开始的正整数数组 nums 。请你找出并统计满足下述条件的三元组 (i, j, k) 的数目：

// 0 <= i < j < k < nums.length
// nums[i]、nums[j] 和 nums[k] 两两不同 。
// 换句话说：nums[i] != nums[j]、nums[i] != nums[k] 且 nums[j] != nums[k] 。
// 返回满足上述条件三元组的数目。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/number-of-unequal-triplets-in-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub struct Solution {}

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut temp_nums = nums.clone();

        temp_nums.sort();

        let mut count_vec = vec![];
        let mut prev_num = None;
        let mut temp_count = 0;
        for i in temp_nums.iter() {
            if prev_num == None {
                prev_num = Some(*i);
                temp_count = 1;
            } else if prev_num == Some(*i) {
                temp_count += 1;
            } else {
                count_vec.push(temp_count);
                prev_num = Some(*i);
                temp_count=1;
            }
        }

        if prev_num!=None{
            count_vec.push(temp_count);
        }

        println!("{:?}", count_vec);

        let mut result = 0;

        for i in 0..count_vec.len() {
            for j in i + 1..count_vec.len() {
                for k in j + 1..count_vec.len() {
                    result += count_vec[i] * count_vec[j] * count_vec[k];
                }
            }
        }
        result
    }
}

#[test]
fn test() {}
