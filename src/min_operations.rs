pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {                
        let mut left: i32 = -1;
        let mut right = 0;
        let mut result: i32 = -1;
        let mut left_sum = 0;
        let mut right_sum = 0;
        let n: i32 = nums.len() as i32;

        for i in nums.iter() {
            right_sum += *i;
        }
        if x>right_sum{
            return -1;
        }else if x==right_sum{
            return n;
        }

        while left < right && right <= n{            
            let sum = left_sum + right_sum;
            if sum == x {
                // println!("left={}, right={}", left, right);
                let current_result = left + 1 + n - right;
                // println!("result={}, current_result={}", result, current_result);                

                if result == -1 {
                    result = current_result;                   
                }else{
                    if result>current_result{
                        result=current_result;
                    } 
                }
                   
                
                if right==n{
                    break;
                }
                right_sum-=nums[right as usize];
                right+=1;
            } else if sum < x {
                left += 1;
                left_sum += nums[left as usize];
                if left==right{
                    if right==n{
                        break;
                    }
                    right_sum-=nums[right as usize];
                    right+=1;
                }
            } else {
                if right==n{
                    break;
                }
                right_sum-=nums[right as usize];
                right+=1;
            }
        }
        result
    }
}
