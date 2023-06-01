pub struct Solution {}

impl Solution {
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        price.sort();

        let mut prev_candy;
        let mut count = 0;
        let mut left = 0;
        let mut right = price.last().unwrap() - price.first().unwrap() + 1;
        let mut result = 0;

        while left < right {
            let mid = (left + right) / 2;
            prev_candy = price[0];
            let mut i = 1;
            count = 1;
            while i < price.len() && count < k {
                if price[i] - prev_candy >= mid {
                    prev_candy = price[i];
                    count += 1;
                }
                i += 1;
            }

            if count == k {
                result = mid;
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        result
    }
}

// 给你一个正整数数组 price ，其中 price[i] 表示第 i 类糖果的价格，另给你一个正整数 k 。

// 商店组合 k 类 不同 糖果打包成礼盒出售。礼盒的 甜蜜度 是礼盒中任意两种糖果 价格 绝对差的最小值。

// 返回礼盒的 最大 甜蜜度。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/maximum-tastiness-of-candy-basket
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {}
