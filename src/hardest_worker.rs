pub struct Solution{}

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut prev_time=0;
        let mut max_time=0;
        let mut best_worker_id=-1;

        logs.iter().for_each(|x|{
            let id=x[0];
            let end_time=x[1];
            let total_time=end_time-prev_time;
            if total_time==max_time&& id<best_worker_id||total_time>max_time{
                best_worker_id=id;
                max_time=total_time;                
            }
            prev_time=end_time;
        });

        best_worker_id
    }
}

// 共有 n 位员工，每位员工都有一个从 0 到 n - 1 的唯一 id 。

// 给你一个二维整数数组 logs ，其中 logs[i] = [idi, leaveTimei] ：

// idi 是处理第 i 个任务的员工的 id ，且
// leaveTimei 是员工完成第 i 个任务的时刻。所有 leaveTimei 的值都是 唯一 的。
// 注意，第 i 个任务在第 (i - 1) 个任务结束后立即开始，且第 0 个任务从时刻 0 开始。

// 返回处理用时最长的那个任务的员工的 id 。如果存在两个或多个员工同时满足，则返回几人中 最小 的 id 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/the-employee-that-worked-on-the-longest-task
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test(){
    let n = 10;
    let logs = vec![vec![0,3],vec![2,5],vec![0,9],vec![1,15]];
}