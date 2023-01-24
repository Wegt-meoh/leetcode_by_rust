use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_side_jumps_dp(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len() - 1;
        let mut dp1: [usize; 4] = [0, 1, 0, 1];
        let mut dp2: [usize; 4] = [0, usize::MAX, usize::MAX, usize::MAX];
        for index in 1..n {
            let curr_block = obstacles[index] as usize;
            let prev_block = obstacles[index - 1] as usize;
            let mut prev_dp;
            let mut curr_dp;

            if index % 2 == 1 {
                prev_dp = &mut dp1;
                curr_dp = &mut dp2;
            } else {
                prev_dp = &mut dp2;
                curr_dp = &mut dp1;
            }

            for floor in 1..=3 {                
                if prev_block != floor &&curr_block!=floor{
                    curr_dp[floor] = prev_dp[floor];
                }else{
                    curr_dp[floor]=usize::MAX;
                }
            }

            for floor in 1..=3{
                if prev_block==floor||curr_block==floor{
                    continue;
                }

                for adjacent in -2..=2{                    
                    let adjacent=floor as i32 + adjacent;
                    if adjacent<1||adjacent>3{
                        continue;
                    }

                    let adjacent=adjacent as usize;
                    if curr_block!=adjacent{
                        curr_dp[adjacent]=curr_dp[adjacent].min(curr_dp[floor]+1);
                    }
                }                
            }
        }

        let mut result = usize::MAX;
        let curr_block = obstacles[n - 1] as usize;
        let flag = n % 2 == 1;
        // println!("dp1 = {:?}", dp1);
        // println!("dp2 = {:?}", dp2);
        for floor in 1..=3 {
            if curr_block == floor {
                continue;
            }

            result = result.min(if flag { dp1[floor] } else { dp2[floor] })
        }
        result as i32
    }

    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len() - 1;
        let mut result = i32::MAX;
        //(0->n,1->3,step)
        let mut visited = vec![[i32::MAX; 4]; n];
        let mut stack: VecDeque<(usize, i32, i32)> = VecDeque::from([(0, 2, 0)]);
        visited[0][2] = 0;

        loop {
            // println!("{}", stack.len());
            // println!("{:?}", stack);
            if stack.is_empty() {
                break;
            }
            let curr = stack.pop_front().unwrap();
            if curr.0 == n - 1 {
                result = result.min(curr.2);
                continue;
            }

            let curr_block = obstacles[curr.0];
            let next_block = obstacles[curr.0 + 1];

            for floor in 1..=3 {
                if curr_block == floor || next_block == floor {
                    continue;
                }
                if floor == curr.1 {
                    let visit_item = &mut visited[curr.0 + 1][floor as usize];
                    if curr.2 < *visit_item {
                        *visit_item = curr.2;
                        stack.push_back((curr.0 + 1, floor, curr.2));
                    }
                } else {
                    let visit_item = &mut visited[curr.0][floor as usize];
                    if curr.2 + 1 < *visit_item {
                        *visit_item = curr.2 + 1;
                        stack.push_back((curr.0, floor, curr.2 + 1));
                    }
                }
            }
        }

        result
    }
}

#[test]
fn test() {
    let res1 = Solution::min_side_jumps(vec![0, 1, 2, 3, 0]);
    let res2 = Solution::min_side_jumps(vec![0, 1, 1, 3, 3, 0]);
    let res3 = Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]);
    let res4 = Solution::min_side_jumps(vec![0, 3, 3, 0, 3, 2, 2, 0, 0, 3, 0]);
    let res5 = Solution::min_side_jumps(vec![
        0, 2, 2, 1, 0, 3, 0, 3, 0, 1, 3, 1, 1, 0, 1, 3, 1, 1, 1, 0, 2, 0, 0, 3, 3, 0, 3, 2, 2, 0,
        0, 3, 3, 3, 0, 0, 2, 0, 0, 3, 3, 0, 3, 3, 0, 0, 3, 1, 0, 1, 0, 2, 3, 1, 1, 0, 3, 3, 0, 3,
        1, 3, 0, 2, 2, 0, 1, 3, 0, 1, 0, 3, 0, 1, 3, 1, 2, 2, 0, 0, 3, 0, 1, 3, 2, 3, 2, 1, 0, 3,
        2, 2, 0, 3, 3, 0, 3, 0, 0, 1, 0,
    ]);
    let res6 = Solution::min_side_jumps(vec![0, 2, 2, 1, 0, 3, 0, 3, 0, 1, 0]);

    println!("{res1}\n{res2}\n{res3}\n{res4}\n{res5}\n{res6}");

    assert!(res1 == 2);
    assert!(res2 == 0);
    assert!(res3 == 2);
    assert!(res4 == 1);
    assert!(res5 == 14);
    assert!(res6 == 2);
}

#[test]
fn test_dp() {
    let res1 = Solution::min_side_jumps_dp(vec![0, 1, 2, 3, 0]);
    let res2 = Solution::min_side_jumps_dp(vec![0, 1, 1, 3, 3, 0]);
    let res3 = Solution::min_side_jumps_dp(vec![0, 2, 1, 0, 3, 0]);
    let res4 = Solution::min_side_jumps_dp(vec![0, 3, 3, 0, 3, 2, 2, 0, 0, 3, 0]);
    let res5 = Solution::min_side_jumps_dp(vec![
        0, 2, 2, 1, 0, 3, 0, 3, 0, 1, 3, 1, 1, 0, 1, 3, 1, 1, 1, 0, 2, 0, 0, 3, 3, 0, 3, 2, 2, 0,
        0, 3, 3, 3, 0, 0, 2, 0, 0, 3, 3, 0, 3, 3, 0, 0, 3, 1, 0, 1, 0, 2, 3, 1, 1, 0, 3, 3, 0, 3,
        1, 3, 0, 2, 2, 0, 1, 3, 0, 1, 0, 3, 0, 1, 3, 1, 2, 2, 0, 0, 3, 0, 1, 3, 2, 3, 2, 1, 0, 3,
        2, 2, 0, 3, 3, 0, 3, 0, 0, 1, 0,
    ]);
    let res6 = Solution::min_side_jumps_dp(vec![0, 2, 2, 1, 0, 3, 0, 3, 0, 1, 0]);

    println!("{res1}\n{res2}\n{res3}\n{res4}\n{res5}\n{res6}");

    assert!(res1 == 2);
    assert!(res2 == 0);
    assert!(res3 == 2);
    assert!(res4 == 1);
    assert!(res5 == 14);
    assert!(res6 == 2);
}

#[test]
fn doing() {
    let mut v = vec![false; 4];
    let f = &mut v[0];
    *f = true;
    println!("{:?}", v);
}
