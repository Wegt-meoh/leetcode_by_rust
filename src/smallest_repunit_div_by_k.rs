use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut set = HashSet::new();
        let result = -1;

        let mut resid = 1 % k;

        if resid == 0 {
            return 1;
        }

        set.insert(resid);

        while resid != 0 {
            resid = (resid * 10 + 1) % k;
            if set.contains(&resid) {
                return -1;
            } else {
                set.insert(resid);
            }
        }

        set.len() as i32
    }
}

// 给定正整数 k ，你需要找出可以被 k 整除的、仅包含数字 1 的最 小 正整数 n 的长度。

// 返回 n 的长度。如果不存在这样的 n ，就返回-1。

// 注意： n 不符合 64 位带符号整数。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/smallest-integer-divisible-by-k
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let k = 1;
    let ans = 1;
    let result = Solution::smallest_repunit_div_by_k(k);
    assert!(ans == result);

    let k = 2;
    let ans = -1;
    let result = Solution::smallest_repunit_div_by_k(k);
    assert!(ans == result);

    let k = 3;
    let ans = 3;
    let result = Solution::smallest_repunit_div_by_k(k);
    assert!(ans == result);
}
