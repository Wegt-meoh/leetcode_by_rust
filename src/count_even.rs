pub struct Solution{}

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut result=0;
        
        for i in 1..=num{
            if Solution::judge(i){
                result+=1;
            }
        }

        result
    }

    fn judge(mut x:i32)->bool{
        let mut odd_count=0;
        while x>0{
            if x%2==1{
                odd_count+=1;
            }
            x/=10
        }
        if odd_count%2==0{
            return true;
        }
        false
    }
}