pub struct Solution {}

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pred_odd = [0, 0];
        let mut pred_even = [0, 0];
        let mut suf_odd = [0, 0];
        let mut suf_even = [0, 0];
        let mut succ_count = 0;

        for (index, num) in nums.iter().enumerate() {
            if index & 1 == 0 {
                suf_even[0] += *num;
            } else {
                suf_odd[0] += *num;
            }
        }

        for index in 0..n {
            if index & 1 == 0 {
                suf_odd[1] = suf_odd[0];
                suf_even[1] = suf_even[0] - nums[index];
                pred_odd[1] = pred_odd[0];
                pred_even[1] = pred_even[0] + nums[index];
                if pred_odd[0] + suf_even[1] == pred_even[0] + suf_odd[1] {
                    succ_count += 1;
                }
            } else {
                suf_odd[0] = suf_odd[1] - nums[index];
                suf_even[0] = suf_even[1];
                pred_odd[0] = pred_odd[1] + nums[index];
                pred_even[0] = pred_even[1];
                if pred_odd[1] + suf_even[0] == pred_even[1] + suf_odd[0] {
                    succ_count += 1;
                }
            }
        }

        succ_count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        let res = Solution::ways_to_make_fair(vec![2, 1, 6, 4]);
        println!("{res}");
        assert!(res == 1);

        let res = Solution::ways_to_make_fair(vec![1, 1, 1]);
        assert!(res == 3, "res = {res}, target=3");

        let res = Solution::ways_to_make_fair(vec![1, 2, 3]);
        println!("{res}");
        assert!(res == 0);
    }
}
