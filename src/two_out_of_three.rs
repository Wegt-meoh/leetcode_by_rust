use std::collections::HashMap;

pub struct Solution{}

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut map1=HashMap::new();
        let mut map2=HashMap::new();
        let mut map3=HashMap::new();        
        let mut result=Vec::new();

        for i in nums1{
            map1.insert(i, true);
        }

        for i in nums2{
            map2.insert(i, true);
        }

        for i in nums3{
            map3.insert(i, true);
        }

        for (i,_) in map1{
            if map2.remove(&i)!=None||map3.remove(&i)!=None{
                result.push(i);
            }            
        }

        for (i,_) in map2{
            if map3.remove(&i)!=None{
                result.push(i);
            }
        }
        
        result
    }
}