use std::collections::HashMap;

pub struct Solution {}

const NUM: i32 = 10_i32.pow(9) + 7;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut pair_count_map = HashMap::new();

        for i in nums {
            let key = Self::calcu(i);
            let origin_value = pair_count_map.get(&key).unwrap_or(&0);
            // println!("----\n
            //             t = {}\n
            //             origin_value = {}\n", key, origin_value);
            result = result + origin_value;
            result %= NUM;
            pair_count_map.insert(key, origin_value + 1);
        }

        result
    }

    fn calcu(num1: i32) -> i32 {
        num1 - Self::rev(num1)
    }

    fn rev(mut num: i32) -> i32 {
        let mut result = 0;
        while num > 0 {
            result = result * 10 + num % 10;
            num /= 10;
        }
        result
    }
}

#[test]
fn test_rev() {
    assert!(Solution::rev(123) == 321);
    assert!(Solution::rev(100) == 1);
    assert!(Solution::rev(120) == 21);
    assert!(Solution::rev(1) == 1);
    assert!(Solution::rev(0) == 0);
}

#[test]
fn test_calcu() {
    assert!(Solution::calcu(12) == 12 - 21);
    assert!(Solution::calcu(1000000000) == 1000000000 - 1);
}

#[test]
fn test() {
    let reuslt1 = Solution::count_nice_pairs(vec![42, 11, 1, 97]);
    let result2 = Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]);
    println!("{} {}", reuslt1, result2);
    assert!(Solution::count_nice_pairs(vec![42, 11, 1, 97]) == 2);
    assert!(Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]) == 4);
}

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {}

    fn isPrime(num: i32) -> bool {}
}
