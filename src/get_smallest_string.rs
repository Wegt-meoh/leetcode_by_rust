use std::result;

pub struct Solution{}

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let n=n as usize;
        let k=k as usize;
        let a_char_code:u32='a'.to_digit(36).unwrap();        
        let mut nums=vec![1;n];              
        let mut index:usize=0;

        for _ in 0..k-n{
            index=0;
            while index<n{
                if nums[index]==26{
                    nums[index]=1;
                    index+=1;
                }else{
                    nums[index]+=1;
                    break;
                }
            }
        }

        let mut result=String::with_capacity(n as usize);
        for i in (0..n).rev(){
            result.push();
        }

        result
    }
}

#[test]
fn test(){    
    let t=char::from_u32(1+'a'.to(36).unwrap()).unwrap();
    println!("this is {t}");
}