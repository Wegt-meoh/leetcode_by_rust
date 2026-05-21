use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut hash_set1 = HashSet::new();
        let mut hash_set2 = HashSet::new();

        for item in arr1.iter() {
            let prefix_vec = Self::get_prefix(*item);
            for item in prefix_vec.iter() {
                hash_set1.insert(*item);
            }
        }
        for item in arr2.iter() {
            let prefix_vec = Self::get_prefix(*item);
            for item in prefix_vec.iter() {
                hash_set2.insert(*item);
            }
        }
        let intersection: Vec<_> = hash_set1.intersection(&hash_set2).collect();
        let mut max_len = 0;
        for item in intersection.iter() {
            let len = Self::get_prefix(**item).len();
            if len > max_len {
                max_len = len;
            }
        }
        max_len as i32
    }

    fn get_prefix(num: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut bit_vec = vec![];
        let mut temp = num;

        while temp > 0 {
            bit_vec.push(temp % 10);
            temp /= 10;
        }

        let mut prev = 0;
        for item in bit_vec.iter().rev() {
            let _res = prev * 10 + *item;
            res.push(_res);
            prev = _res;
        }

        res
    }
}
