pub struct Solution {}

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }
        let mut temp_x = 0;
        let flag = {
            if x < 0 {
                false
            } else {
                true
            }
        };

        if !flag {
            x = -x;
        }

        while x > 0 {
            if temp_x > i32::MAX / 10 {
                return 0;
            }
            let max = i32::MAX - temp_x * 10;
            if x % 10 > max {
                return 0;
            }
            temp_x = temp_x * 10 + x % 10;
            x /= 10;
        }

        if !flag {
            if temp_x == i32::MAX {
                return 0;
            }
            return -temp_x;
        }
        temp_x
    }
}

// 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。

// 如果反转后整数超过 32 位的有符号整数的范围 [−231,  231 − 1] ，就返回 0。

// 假设环境不允许存储 64 位整数（有符号或无符号）。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/reverse-integer
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let x = 123;
    let ans = 321;
    let result = Solution::reverse(x);
    assert!(ans == result, "ans = {},result = {}", ans, result);

    let x = -123;
    let ans = -321;
    let result = Solution::reverse(x);
    assert!(ans == result, "ans = {},result = {}", ans, result);

    let x = 0;
    let ans = 0;
    let result = Solution::reverse(x);
    assert!(ans == result, "ans = {},result = {}", ans, result);
}
