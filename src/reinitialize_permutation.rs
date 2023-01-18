pub struct Solution{}

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let n=n as usize;
        let mut perm=Vec::new();    
        let mut result=0;    
        let mut arr=Vec::new();
        let mut flag=false;
        for i in 0..n{
            perm.push(i);
            arr.push(0);
        }        

        while flag==false{            
            result+=1;
            flag=true;
            for i in 0..n{
                if i%2==0{
                    arr[i]=perm[(i/2)];
                }else{
                    arr[i]=perm[(n/2+(i-1)/2)];
                }
                if arr[i]!=i{
                    flag=false;
                }
            }
            for i in 0..n{
                perm[i]=arr[i]
            }
        }

        result
        
    }
}