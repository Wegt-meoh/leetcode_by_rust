pub struct Solution{}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n=nums.len();
        let mut dp=vec![1;n];

        for i in 1..n{
            for j in 0..i{
                if nums[i]>nums[j]{
                    dp[i]=dp[i].max(dp[j]+1);
                }                
            }
        }


        let mut max_item=0;
        for i in dp{
            max_item=max_item.max(i)
        }

        max_item
    }
}

#[test]
fn test(){
    // 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

    // 子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/longest-increasing-subsequence
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let nums=vec![10,9,2,5,3,7,101,18];
    let ans=4;
    let result=Solution::length_of_lis(nums);
    assert!(result==ans);

    let nums=vec![0,1,0,3,2,3];
    let ans=4;
    let result=Solution::length_of_lis(nums);
    assert!(result==ans);

    let nums=vec![7,7,7,7,7,7,7];
    let ans=1;
    let result=Solution::length_of_lis(nums);
    assert!(result==ans);

    let nums=vec![1,3,6,7,9,4,10,5,6];
    let ans=6;
    let result=Solution::length_of_lis(nums);
    assert!(result==ans);
}