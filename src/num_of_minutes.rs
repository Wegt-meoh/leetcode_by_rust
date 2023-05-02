use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        if n == 1 {
            return 0;
        }
        if n == 2 {
            return inform_time[0];
        }

        let manager_map: HashMap<i32, Vec<i32>> =
            manager
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (index, e)| {
                    if *e == -1 {
                        return acc;
                    }
                    let index = index as i32;
                    acc.entry(*e)
                        .and_modify(|x| x.push(index))
                        .or_insert(vec![index]);
                    return acc;
                });

        fn dfs<'a>(
            curr_id: i32,
            curr_time: i32,
            manager_map: &HashMap<i32, Vec<i32>>,
            inform_time: &Vec<i32>,
        ) -> i32 {
            match manager_map.get(&curr_id) {
                Some(children) => {
                    return children.iter().fold(0, |acc, x| {
                        let result = dfs(
                            *x,
                            curr_time + inform_time[*x as usize],
                            manager_map,
                            inform_time,
                        );
                        return result.max(acc);
                    });
                }
                None => {
                    return curr_time;
                }
            }
        }

        dfs(
            head_id,
            inform_time[head_id as usize],
            &manager_map,
            &inform_time,
        )
    }
}

#[test]
fn test() {
    // 公司里有 n 名员工，每个员工的 ID 都是独一无二的，编号从 0 到 n - 1。公司的总负责人通过 headID 进行标识。

    // 在 manager 数组中，每个员工都有一个直属负责人，其中 manager[i] 是第 i 名员工的直属负责人。对于总负责人，manager[headID] = -1。题目保证从属关系可以用树结构显示。

    // 公司总负责人想要向公司所有员工通告一条紧急消息。他将会首先通知他的直属下属们，然后由这些下属通知他们的下属，直到所有的员工都得知这条紧急消息。

    // 第 i 名员工需要 informTime[i] 分钟来通知它的所有直属下属（也就是说在 informTime[i] 分钟后，他的所有直属下属都可以开始传播这一消息）。

    // 返回通知所有员工这一紧急消息所需要的 分钟数 。

    // 来源：力扣（LeetCode）
    // 链接：https://leetcode.cn/problems/time-needed-to-inform-all-employees
    // 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

    let n = 1;
    let head_id = 0;
    let manager = vec![-1];
    let inform_time = vec![0];
    let ans = 0;
    let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
    assert!(ans == result);

    let n = 6;
    let head_id = 2;
    let manager = vec![2, 2, -1, 2, 2, 2];
    let inform_time = vec![0, 0, 1, 0, 0, 0];
    let ans = 1;
    let result = Solution::num_of_minutes(n, head_id, manager, inform_time);
    println!("ans={}, result={}", ans, result);
    assert!(ans == result);
}
