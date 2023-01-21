use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len() - 1;
        //(0->n,1->3,step)
        let mut visited = vec![[false;4]; n];
        let mut stack: VecDeque<(usize, i32, i32)> =
            VecDeque::from([(0, 2, 0)]);    
        visited[0][2]=true;    

        loop {
            // println!("{}", stack.len());
            // println!("{:?}", stack);
            let curr = stack.pop_front().unwrap();
            if curr.0 == n-1 {
                return curr.2;
            }
            
            let curr_block = obstacles[curr.0];
            let next_block = obstacles[curr.0 + 1];

            if next_block != curr.1 {
                let visit_item=&mut visited[curr.0+1][curr.1 as usize];
                if !*visit_item{
                    stack.push_back((curr.0 + 1, curr.1, curr.2));  
                    *visit_item=true;  
                }                
            }

            for floor in 1..=3 {
                if floor == curr.1 {
                    continue;
                }

                let visit_item=&mut visited[curr.0][floor as usize];
                if floor != curr_block&&!*visit_item {
                    stack.push_back((curr.0,floor,curr.2+1));
                    *visit_item=true;
                }
            }
        }
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

    println!("{res1}-{res2}-{res3}-{res4}-{res5}");

    assert!(res1 == 2);
    assert!(res2 == 0);
    assert!(res3 == 2);
    assert!(res4 == 1);
    assert!(res5 == 14);
}

#[test]
fn doing(){
    let mut v=vec![false;4];
    let f=&mut v[0];
    *f=true;
    println!("{:?}",  v);
}