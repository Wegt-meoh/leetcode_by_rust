pub struct Solution {}

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let max = Self::find_max(&nums) as usize;
        let mut result = 0;
        let num_occur_vec = {
            let mut result = vec![false; max + 1];
            for value in nums.iter() {
                result[*value as usize] = true;
            }
            result
        };

        let mut could_div_vec = Vec::new();

        for gcd in 1..=max {
            for num in 1..=max {
                let t = gcd * num;
                if t > max {
                    break;
                }
                if num_occur_vec[t] {
                    could_div_vec.push(t);
                }
            }

            if could_div_vec.is_empty() {
                continue;
            }

            let mut temp_gcd = could_div_vec[0];
            for div_num in could_div_vec.iter() {
                temp_gcd = Self::gcd(temp_gcd, *div_num);
                if temp_gcd == 1 {
                    break;
                }
            }

            if temp_gcd == gcd {
                result += 1;
            }

            could_div_vec.clear();
        }

        result
    }

    fn find_max(nums: &Vec<i32>) -> i32 {
        let mut max = 0;
        for i in nums {
            if *i > max {
                max = *i;
            }
        }
        max
    }

    fn gcd(a: usize, b: usize) -> usize {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }

        if a > b {
            Self::gcd(a % b, b)
        } else {
            Self::gcd(b % a, a)
        }
    }
}
