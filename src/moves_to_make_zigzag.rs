use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut result_1 = 0;
        let nums_length = nums.len();

        for index in (1..nums_length).step_by(2) {
            let prev_even_num = nums[index - 1];
            let next_even_num = {
                if index + 1 >= nums_length {
                    2000
                } else {
                    nums[index + 1]
                }
            };

            let min_num = min(prev_even_num, next_even_num);
            let difference = nums[index] - min_num;
            if difference >= 0 {
                result_1 += difference + 1;
            }
        }

        let mut result_2 = 0;
        for index in (0..nums_length).step_by(2) {
            let prev_odd_num = {
                if (index as i32 - 1) < 0 {
                    2000
                } else {
                    nums[index - 1]
                }
            };

            let next_odd_num = {
                if index + 1 >= nums_length {
                    2000
                } else {
                    nums[index + 1]
                }
            };

            let min_num = min(prev_odd_num, next_odd_num);
            let difference = nums[index] - min_num;
            if difference >= 0 {
                result_2 += difference + 1;
            }
        }

        min(result_1, result_2)
    }
}

#[test]
fn test() {
    assert!(Solution::moves_to_make_zigzag(vec![1, 2, 3]) == 2);
    assert!(Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]) == 4);
}
