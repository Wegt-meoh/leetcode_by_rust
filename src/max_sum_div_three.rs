// 给你一个整数数组 nums，请你找出并返回能被三整除的元素最大和。

//

// 示例 1：

// 输入：nums = [3,6,5,1,8]
// 输出：18
// 解释：选出数字 3, 6, 1 和 8，它们的和是 18（可被 3 整除的最大和）。
// 示例 2：

// 输入：nums = [4]
// 输出：0
// 解释：4 不能被 3 整除，所以无法选出数字，返回 0。
// 示例 3：

// 输入：nums = [1,2,3,4,4]
// 输出：12
// 解释：选出数字 1, 3, 4 以及 4，它们的和是 12（可被 3 整除的最大和）。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/greatest-sum-divisible-by-three
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub struct Solution {}

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut base = 0;
        let mut handled = [vec![], vec![]];
        for num in nums.iter() {
            let r = *num % 3;

            if r == 0 {
                base += *num;
            } else if r == 1 {
                handled[0].push(*num);
            } else {
                handled[1].push(*num)
            }
        }

        handled[1].sort_by(|x, y| {
            return y.cmp(x);
        });

        handled[0].sort_by(|x, y| {
            return y.cmp(x);
        });

        for i in 0..3 {
            if handled[0].len() < i {
                break;
            }
            for j in 0..3 {
                if handled[1].len() < j {
                    break;
                }
                let count1 = handled[0].len() - i;
                let count2 = handled[1].len() - j;
                if (count1 + 2 * count2) % 3 == 0 {
                    let mut sum = 0;
                    for index in 0..count1 {
                        sum += handled[0][index];
                    }
                    for index in 0..count2 {
                        sum += handled[1][index];
                    }
                    result = result.max(sum);
                }
            }
        }

        result + base
    }
}

#[test]
fn test() {}
