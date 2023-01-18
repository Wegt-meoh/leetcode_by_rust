pub struct Solution{}

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {        
        let mut nums_len=nums.len();        

        while nums_len>1{
            for i in 0..nums_len/2{
                let t=i*2;
                if i%2==0{                    
                    nums[i]=nums[t].min(nums[t+1]);
                }else{                    
                    nums[i]=nums[t].max(nums[t+1]);
                }
            }
            nums_len/=2;
        }

        nums[0]
    }
}