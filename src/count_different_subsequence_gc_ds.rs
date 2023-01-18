pub struct Solution{}

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let max=Self::find_max(&nums) as usize;
        let mut result=0;
        let num_occur_vec={
            let mut result=Vec::with_capacity(max+1);
            for _ in 0..result.capacity(){
                result.push(false);
            }
            for value in nums.iter(){
                result[*value as usize]=true;
            }            
            result
        };
        
        let mut could_div_vec=Vec::new();

        for i in 1..=max{            
            for j in 1..=max{
                let t=i*j;
                if t>max{
                    break;
                }
                if num_occur_vec[t]{
                    could_div_vec.push(t);
                }
            }

            if could_div_vec.len()==0{
                continue;
            }
            let mut target=could_div_vec[0];
            for j in could_div_vec.iter(){
                target=Self::gcd(target, *j);                
                if target == 1{
                    break;
                }
            }

            if target==i{
                result+=1;
            }

            could_div_vec.clear();
        }

        result
    }

    fn find_max(nums:&Vec<i32>)->i32{
        let mut max=0;
        for i in nums{
            if *i>max{
                max=*i;
            }
        }
        max
    }

    fn gcd(a:usize,b:usize) -> usize {
        if a==0{
            return b;
        }
        if b==0{
            return a;
        }        
        
        if a>b{            
            return Self::gcd(a%b,b);
        }else{
            return Self::gcd(b%a,a);
        }
    }
}