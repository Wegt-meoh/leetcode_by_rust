use std::{collections::HashSet, mem::swap};

pub struct Solution {}

impl Solution {
    pub fn powerful_integers(mut x: i32, mut y: i32, bound: i32) -> Vec<i32> {
        if bound <= 1 {
            return vec![];
        }

        if x == 1 && y == 1 {
            if bound >= 2 {
                return vec![2];
            }
            return vec![];
        }

        if y == 1 || x == 1 {
            if y == 1 {
                swap(&mut x, &mut y);
            }
            let mut result = vec![];
            let mut curr_y = 1;
            for _ in 0..=20 {
                if curr_y >= bound {
                    break;
                }

                if curr_y + x <= bound {
                    result.push(curr_y + x);
                } else {
                    break;
                }
                curr_y *= y;
            }

            return result;
        }

        let mut result_set: HashSet<i32> = HashSet::new();
        let mut curr_x = 1;
        let mut curr_y = 1;
        for _ in 0..20 {
            curr_y = 1;
            if curr_x >= bound {
                break;
            }
            for _ in 0..20 {
                if curr_y >= bound {
                    break;
                }

                if curr_x + curr_y <= bound {
                    result_set.insert(curr_x + curr_y);
                } else {
                    break;
                }
                curr_y *= y;
            }
            curr_x *= x;
        }

        result_set.into_iter().collect()
    }
}

#[test]
fn test() {
    // 给定三个整数 x 、 y 和 bound ，返回 值小于或等于 bound 的所有 强整数 组成的列表 。

    // 如果某一整数可以表示为 xi + yj ，其中整数 i >= 0 且 j >= 0，那么我们认为该整数是一个 强整数 。

    // 你可以按 任何顺序 返回答案。在你的回答中，每个值 最多 出现一次。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/powerful-integers
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let (x, y, bound) = (2, 3, 10);
    let ans = vec![2, 3, 4, 5, 7, 9, 10];
    let result = Solution::powerful_integers(x, y, bound);
    assert!(ans.clone().sort() == result.clone().sort());

    let (x, y, bound) = (3, 5, 15);
    let ans = vec![2, 4, 6, 8, 10, 14];
    let result = Solution::powerful_integers(x, y, bound);
    assert!(ans.clone().sort() == result.clone().sort());
}
