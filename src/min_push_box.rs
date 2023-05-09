use std::collections::VecDeque;

pub struct Solution {}

const DIRECTION: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
impl Solution {
    pub fn min_push_box(mut grid: Vec<Vec<char>>) -> i32 {
        let mut result = -1;
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        enum MapCell {
            BOX,
            WALL,
            GROUND,
            TARGET,
        }

        fn add(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
            (a.0 + b.0, a.1 + b.1)
        }

        fn sub(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
            (a.0 - b.0, a.1 - b.1)
        }

        fn out_area(pos: &(i32, i32), m: &i32, n: &i32) -> bool {
            if pos.0 >= 0 && pos.0 < *m && pos.1 >= 0 && pos.1 < *n {
                return false;
            }
            true
        }

        fn pos_is_what(pos: &(i32, i32), map: &Vec<Vec<char>>) -> MapCell {
            match map[pos.0 as usize][pos.1 as usize] {
                'B' => MapCell::BOX,
                '#' => MapCell::WALL,
                'T' => MapCell::TARGET,
                '.' => MapCell::GROUND,
                _ => MapCell::GROUND,
            }
        }

        fn swap_pos(pos1: (i32, i32), pos2: (i32, i32), map: &mut Vec<Vec<char>>) {
            let c1 = map[pos1.0 as usize][pos1.1 as usize];
            map[pos1.0 as usize][pos1.1 as usize] = map[pos2.0 as usize][pos2.1 as usize];
            map[pos2.0 as usize][pos2.1 as usize] = c1;
        }

        fn get_a_to_b_min_step(map: &Vec<Vec<char>>, pos_a: (i32, i32), pos_b: (i32, i32)) -> i32 {
            let mut result = -1;
            let m = map.len();
            let n = map[0].len();
            let mut history = vec![vec![i32::MAX; n]; m];

            fn bfs(
                map: &Vec<Vec<char>>,
                history: &mut Vec<Vec<i32>>,
                curr_pos: (i32, i32),
                target_pos: (i32, i32),
                result: &mut i32,
                m: &i32,
                n: &i32,
            ) {
                let mut two_queue = [VecDeque::new(), VecDeque::new()];
                let mut count = 0;
                two_queue[0].push_back(curr_pos);

                'a: loop {
                    if two_queue[count % 2].is_empty() {
                        break;
                    }

                    while !two_queue[count % 2].is_empty() {
                        let pos = two_queue[count % 2].pop_front().unwrap();
                        // println!("pos={:?}, count={}", pos, count);

                        if pos == target_pos {
                            *result = count as i32;
                            break 'a;
                        }

                        for dir in DIRECTION {
                            let next_pos = add(&dir, &pos);
                            if out_area(&next_pos, m, n) {
                                continue;
                            }

                            match pos_is_what(&next_pos, map) {
                                MapCell::BOX => (),
                                MapCell::WALL => (),
                                MapCell::TARGET | MapCell::GROUND => {
                                    if count as i32 + 1
                                        >= history[next_pos.0 as usize][next_pos.1 as usize]
                                    {
                                        continue;
                                    }

                                    history[next_pos.0 as usize][next_pos.1 as usize] =
                                        count as i32 + 1;
                                    two_queue[(count + 1) % 2].push_back(next_pos);
                                }
                            }
                        }
                    }

                    count += 1;
                }
            }

            fn dfs(
                map: &Vec<Vec<char>>,
                history: &mut Vec<Vec<i32>>,
                step: i32,
                curr_pos: (i32, i32),
                target_pos: (i32, i32),
                result: &mut i32,
                m: &i32,
                n: &i32,
            ) {
                // println!("get min step: curr_pos={:?},target_post={:?}", curr_pos, target_pos);
                if curr_pos == target_pos {
                    if *result == -1 {
                        *result = step;
                    } else {
                        *result = (*result).min(step);
                    }
                    return;
                }

                for dir in DIRECTION {
                    let next_pos = add(&curr_pos, &dir);
                    if out_area(&next_pos, m, n) {
                        continue;
                    }

                    match pos_is_what(&next_pos, map) {
                        MapCell::BOX => (),
                        MapCell::WALL => (),
                        MapCell::TARGET | MapCell::GROUND => {
                            if step + 1 >= history[next_pos.0 as usize][next_pos.1 as usize] {
                                continue;
                            }
                            history[next_pos.0 as usize][next_pos.1 as usize] = step + 1;
                            dfs(map, history, step + 1, next_pos, target_pos, result, m, n);
                        }
                    }
                }
            }
            history[pos_a.0 as usize][pos_a.1 as usize] = 0;

            bfs(
                map,
                &mut history,
                pos_a,
                pos_b,
                &mut result,
                &(m as i32),
                &(n as i32),
            );

            result
        }

        fn dfs(
            map: &mut Vec<Vec<char>>,
            step: i32,
            result: &mut i32,
            player_pos: (i32, i32),
            box_pos: (i32, i32),
            target_pos: &(i32, i32),
            m: &i32,
            n: &i32,
            history: &mut Vec<Vec<Vec<Vec<i32>>>>,
        ) {
            // println!(
            //     "player_pos={:?},box_pos={:?},target_pos={:?} step={}",
            //     player_pos, box_pos, target_pos, step
            // );
            // for i in map.iter() {
            //     println!("{:?}", i);
            // }
            if box_pos == *target_pos {
                if *result == -1 {
                    *result = step;
                } else {
                    *result = (*result).min(step);
                }
                return;
            }

            for dir in DIRECTION {
                let next_pos = add(&box_pos, &dir);
                if out_area(&next_pos, m, n) {
                    continue;
                }
                match pos_is_what(&next_pos, map) {
                    MapCell::BOX => (),
                    MapCell::WALL => (),
                    MapCell::TARGET | MapCell::GROUND => {
                        let could_step = get_a_to_b_min_step(map, player_pos, next_pos);
                        // println!(
                        //     "play_pos={:?}, next_pos={:?}, could_step={}",
                        //     player_pos, next_pos, could_step
                        // );
                        if could_step == -1 {
                            continue;
                        }

                        let box_next_pos = sub(&box_pos, &dir);
                        if out_area(&box_next_pos, m, n) {
                            continue;
                        }

                        match pos_is_what(&box_next_pos, map) {
                            MapCell::BOX => (),
                            MapCell::WALL => (),
                            MapCell::TARGET | MapCell::GROUND => {
                                // push the box
                                if step + 1
                                    >= history[box_pos.0 as usize][box_pos.1 as usize]
                                        [next_pos.0 as usize]
                                        [next_pos.1 as usize]
                                {
                                    continue;
                                }

                                history[box_pos.0 as usize][box_pos.1 as usize]
                                    [next_pos.0 as usize][next_pos.1 as usize] = step + 1;

                                swap_pos(box_pos, box_next_pos, map);
                                dfs(
                                    map,
                                    step + 1,
                                    result,
                                    box_pos,
                                    box_next_pos,
                                    target_pos,
                                    m,
                                    n,
                                    history,
                                );
                                swap_pos(box_pos, box_next_pos, map);
                            }
                        }
                    }
                }
            }
        }

        let (player_pos, box_pos, target_pos) = {
            let mut player_pos = (0, 0);
            let mut box_pos = (0, 0);
            let mut target_pos = (0, 0);
            for i in 0..m {
                for j in 0..n {
                    match grid[i as usize][j as usize] {
                        'S' => player_pos = (i, j),
                        'B' => box_pos = (i, j),
                        'T' => target_pos = (i, j),
                        _ => (),
                    }
                }
            }
            (
                (player_pos.0 as i32, player_pos.1 as i32),
                (box_pos.0 as i32, box_pos.1 as i32),
                (target_pos.0 as i32, target_pos.1 as i32),
            )
        };

        let mut history =
            vec![vec![vec![vec![i32::MAX; n as usize]; m as usize]; n as usize]; m as usize];
        // history[box_pos.0 as usize][box_pos.1 as usize][player_pos.0 as usize]
        //     [player_pos.1 as usize] = 0;
        grid[player_pos.0 as usize][player_pos.1 as usize] = '.';
        dfs(
            &mut grid,
            0,
            &mut result,
            player_pos,
            box_pos,
            &target_pos,
            &m,
            &n,
            &mut history,
        );

        result
    }
}

// 「推箱子」是一款风靡全球的益智小游戏，玩家需要将箱子推到仓库中的目标位置。

// 游戏地图用大小为 m x n 的网格 grid 表示，其中每个元素可以是墙、地板或者是箱子。

// 现在你将作为玩家参与游戏，按规则将箱子 'B' 移动到目标位置 'T' ：

// 玩家用字符 'S' 表示，只要他在地板上，就可以在网格中向上、下、左、右四个方向移动。
// 地板用字符 '.' 表示，意味着可以自由行走。
// 墙用字符 '#' 表示，意味着障碍物，不能通行。
// 箱子仅有一个，用字符 'B' 表示。相应地，网格上有一个目标位置 'T'。
// 玩家需要站在箱子旁边，然后沿着箱子的方向进行移动，此时箱子会被移动到相邻的地板单元格。记作一次「推动」。
// 玩家无法越过箱子。
// 返回将箱子推到目标位置的最小 推动 次数，如果无法做到，请返回 -1。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/minimum-moves-to-move-a-box-to-their-target-location
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn transform(grid: Vec<Vec<&str>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|x| {
            x.iter()
                .map(|y| y.chars().last().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn test() {
    let grid = transform(vec![
        vec!["#", "#", "#", "#", "#", "#"],
        vec!["#", "T", "#", "#", "#", "#"],
        vec!["#", ".", ".", "B", ".", "#"],
        vec!["#", ".", "#", "#", ".", "#"],
        vec!["#", ".", ".", ".", "S", "#"],
        vec!["#", "#", "#", "#", "#", "#"],
    ]);
    let ans = 3;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test1() {
    let grid = transform(vec![
        vec!["#", "#", "#", "#", "#", "#"],
        vec!["#", "T", "#", "#", "#", "#"],
        vec!["#", ".", ".", "B", ".", "#"],
        vec!["#", "#", "#", "#", ".", "#"],
        vec!["#", ".", ".", ".", "S", "#"],
        vec!["#", "#", "#", "#", "#", "#"],
    ]);
    let ans = -1;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test2() {
    let grid = transform(vec![
        vec!["#", "#", "#", "#", "#", "#"],
        vec!["#", "T", ".", ".", "#", "#"],
        vec!["#", ".", "#", "B", ".", "#"],
        vec!["#", ".", ".", ".", ".", "#"],
        vec!["#", ".", ".", ".", "S", "#"],
        vec!["#", "#", "#", "#", "#", "#"],
    ]);
    let ans = 5;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test3() {
    let grid = transform(vec![
        vec!["#", ".", ".", "#", "#", "#", "#", "#"],
        vec!["#", ".", ".", "T", "#", ".", ".", "#"],
        vec!["#", ".", ".", ".", "#", "B", ".", "#"],
        vec!["#", ".", ".", ".", ".", ".", ".", "#"],
        vec!["#", ".", ".", ".", "#", ".", "S", "#"],
        vec!["#", ".", ".", "#", "#", "#", "#", "#"],
    ]);
    let ans = 7;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test4() {
    let grid = transform(vec![
        vec!["#", ".", ".", "#", "T", "#", "#", "#", "#"],
        vec!["#", ".", ".", "#", ".", "#", ".", ".", "#"],
        vec!["#", ".", ".", "#", ".", "#", "B", ".", "#"],
        vec!["#", ".", ".", ".", ".", ".", ".", ".", "#"],
        vec!["#", ".", ".", ".", ".", "#", ".", "S", "#"],
        vec!["#", ".", ".", "#", ".", "#", "#", "#", "#"],
    ]);
    let ans = 8;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test5() {
    let grid = transform(vec![
        vec![".", ".", "#", ".", ".", ".", ".", "#"],
        vec![".", "B", ".", ".", ".", ".", ".", "#"],
        vec![".", ".", "S", ".", ".", ".", ".", "."],
        vec![".", "#", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", "T", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", "#"],
        vec![".", "#", ".", ".", ".", ".", ".", "."],
    ]);
    let ans = 6;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}

#[test]
fn test6() {
    let grid = transform(vec![vec!["S"], vec!["B"], vec!["T"], vec!["."], vec!["#"]]);
    let ans = 1;
    let result = Solution::min_push_box(grid);
    println!("ans = {}, result = {}", ans, result);
    assert!(ans == result);
}
