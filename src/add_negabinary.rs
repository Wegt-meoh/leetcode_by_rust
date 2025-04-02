pub struct Solution {}

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(arr1.len().max(arr2.len()));
        let mut arr1_index = (arr1.len() - 1) as i32;
        let mut arr2_index = (arr2.len() - 1) as i32;
        let mut temp = 0;
        let mut flag = 1;

        while arr1_index >= 0 || arr2_index >= 0 {
            let mut sum = {
                let a = {
                    if arr1_index >= 0 {
                        arr1[arr1_index as usize]
                    } else {
                        0
                    }
                };
                let b = {
                    if arr2_index >= 0 {
                        arr2[arr2_index as usize]
                    } else {
                        0
                    }
                };

                temp + flag * (a + b)
            };

            if sum * flag < 0 {
                let mut times = 0;
                while sum * flag < 0 {
                    sum += flag * 2;
                    times += 1;
                }

                temp = -flag * times;
                result.push(sum & 1);
            } else {
                temp = sum / 2;
                result.push(sum & 1);
            }

            arr1_index -= 1;
            arr2_index -= 1;
            flag = -flag;
        }

        while temp != 0 {
            if temp * flag < 0 {
                let mut times = 0;
                while temp * flag < 0 {
                    temp += flag * 2;
                    times += 1;
                }

                temp = -flag * times;
                result.push(temp & 1);
            } else {
                result.push(temp & 1);
                temp /= 2;
            }
            flag = -flag;
        }

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        result.reverse();

        result
    }
}

// 给出基数为 -2 的两个数 arr1 和 arr2，返回两数相加的结果。

// 数字以 数组形式 给出：数组由若干 0 和 1 组成，按最高有效位到最低有效位的顺序排列。例如，arr = [1,1,0,1] 表示数字 (-2)^3 + (-2)^2 + (-2)^0 = -3。数组形式 中的数字 arr 也同样不含前导零：即 arr == [0] 或 arr[0] == 1。

// 返回相同表示形式的 arr1 和 arr2 相加的结果。两数的表示形式为：不含前导零、由若干 0 和 1 组成的数组。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/adding-two-negabinary-numbers
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let arr1 = vec![1, 1, 1, 1, 1];
    let arr2 = vec![1, 0, 1];
    let ans = vec![1, 0, 0, 0, 0];
    let result = Solution::add_negabinary(arr1, arr2);
    assert!(ans == result, "ans={:?}, result={:?}", ans, result);

    let arr1 = vec![0];
    let arr2 = vec![0];
    let ans = vec![0];
    let result = Solution::add_negabinary(arr1, arr2);
    assert!(ans == result, "ans={:?}, result={:?}", ans, result);

    let arr1 = vec![0];
    let arr2 = vec![1];
    let ans = vec![1];
    let result = Solution::add_negabinary(arr1, arr2);
    assert!(ans == result, "ans={:?}, result={:?}", ans, result);

    let arr1 = vec![1];
    let arr2 = vec![1];
    let ans = vec![1, 1, 0];
    let result = Solution::add_negabinary(arr1, arr2);
    assert!(ans == result, "ans={:?}, result={:?}", ans, result);

    let arr1 = vec![0];
    let arr2 = vec![1, 0];
    let ans = vec![1, 0];
    let result = Solution::add_negabinary(arr1, arr2);
    assert!(ans == result, "ans={:?}, result={:?}", ans, result);
}
