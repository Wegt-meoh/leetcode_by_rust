pub struct Solution{}

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut result=0;
        let mut count=0;        
        let mut flag=false;
        for i in s.chars(){            
            if i == 'X'{
                flag=true;
            }
            if flag{
                count+=1;                
            }
            if count==3{
                count=0;
                result+=1;
                flag=false;
            }
        }

        if count>0{
            result+=1;
        }

        result
    }
}