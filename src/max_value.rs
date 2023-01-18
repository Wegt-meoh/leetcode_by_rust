pub struct Solution{}

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {      
        let max_sum_i64=max_sum as i64;    
        let mut left:i64=1;
        let mut right:i64=max_sum_i64+1;     
        let mut index_value:i64=max_sum_i64; 
        let left_width:i64=index as i64;
        let right_width:i64=(n-index-1) as i64;
        let mut count:i64=0;        
        while left<right{
            index_value=(left+right)/2;
            count=sum(index_value-1,left_width)+sum(index_value-1, right_width)+index_value;
            if count>max_sum as i64{
                right=index_value;
            }else if count<max_sum as i64{
                left=index_value+1;
            }else{
                return index_value as i32;
            }
        }

        (left-1) as i32
    }
}

fn sum(begin_value:i64,x_width:i64)->i64{
    if begin_value>=x_width{
        let min_value=begin_value-x_width+1;
        (begin_value+min_value)*x_width/2
    }else{
        (begin_value+1)*begin_value/2+x_width-begin_value
    }
}