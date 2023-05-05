use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let hash_map: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut map, ele| {
            map.entry(*ele).and_modify(|x| *x += 1).or_insert(1);
            map
        });

        for (k, v) in hash_map {
            if v == 1 {
                return k;
            }
        }

        -1
    }

    pub fn single_number1(nums: Vec<i32>) -> i32 {        
        let mut ans=0;

        for i in 0..32{
            let mut total=0;
            nums.iter().for_each(|item|{
                total+=(*item)>>i&1;
            });
            if total%3==1{
                ans|=1<<i;
            }
        }

        ans
    }
}

#[test]
fn test11(){
    let t=-1;    
    for i in 0..32{
        println!("{}", t>>i&1);
    }
}

// 给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。请你找出并返回那个只出现了一次的元素。

//

// 示例 1：

// 输入：nums = [2,2,3,2]
// 输出：3
// 示例 2：

// 输入：nums = [0,1,0,1,0,1,100]
// 输出：100

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/WGki4K
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let nums = vec![2, 2, 3, 2];
    let ans = 3;
    let result = Solution::single_number(nums);
    assert!(ans == result);

    let nums = vec![0, 1, 0, 1, 0, 1, 100];
    let ans = 100;
    let result = Solution::single_number(nums);
    assert!(ans == result);
}

#[test]
fn test1() {
    let nums = vec![2, 2, 3, 2];
    let ans = 3;
    let result = Solution::single_number1(nums);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);

    let nums = vec![0, 1, 0, 1, 0, 1, 100];
    let ans = 100;
    let result = Solution::single_number1(nums);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);

    let nums = vec![-2, -2, 1, 1, 4, 1, 4, 4, -4, -2];
    let ans = -4;
    let result = Solution::single_number1(nums);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);

    let nums = vec![
        43,
        16,
        45,
        89,
        45,
        -2147483648,
        45,
        2147483646,
        -2147483647,
        -2147483648,
        43,
        2147483647,
        -2147483646,
        -2147483648,
        89,
        -2147483646,
        89,
        -2147483646,
        -2147483647,
        2147483646,
        -2147483647,
        16,
        16,
        2147483646,
        43,
    ];
    let ans = 2147483647;
    let result = Solution::single_number1(nums);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);

    let nums = vec![
        -401451,
        -177656,
        -2147483646,
        -473874,
        -814645,
        -2147483646,
        -852036,
        -457533,
        -401451,
        -473874,
        -401451,
        -216555,
        -917279,
        -457533,
        -852036,
        -457533,
        -177656,
        -2147483646,
        -177656,
        -917279,
        -473874,
        -852036,
        -917279,
        -216555,
        -814645,
        2147483645,
        -2147483648,
        2147483645,
        -814645,
        2147483645,
        -216555,
    ];
    let ans = -2147483648;
    let result = Solution::single_number1(nums);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}
