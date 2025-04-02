// 在 n*m 大小的棋盘中，有黑白两种棋子，黑棋记作字母 "X", 白棋记作字母 "O"，空余位置记作 "."。
// 当落下的棋子与其他相同颜色的棋子在行、列或对角线完全包围（中间不存在空白位置）另一种颜色的棋子，则可以翻转这些棋子的颜色。
// 「力扣挑战赛」黑白翻转棋项目中，将提供给选手一个未形成可翻转棋子的棋盘残局，其状态记作 chessboard。若下一步可放置一枚黑棋，请问选手最多能翻转多少枚白棋。

pub struct Solution {}

#[derive(PartialEq, Clone, Copy)]
enum Chess {
    black,
    white,
    nothing,
}

impl Solution {
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        let mut chess_map = vec![vec![Chess::nothing; chessboard[0].len()]; chessboard.len()];
        let n = chess_map.len();
        let m = chess_map[0].len();

        for (i, string) in chessboard.iter().enumerate() {
            for (j, c) in string.chars().enumerate() {
                if c == '.' {
                    chess_map[i][j] = Chess::nothing;
                } else if c == 'X' {
                    chess_map[i][j] = Chess::black;
                } else {
                    chess_map[i][j] = Chess::white;
                }
            }
        }

        let mut result = 0;
        for i in 0..n {
            for j in 0..m {
                if chess_map[i][j] == Chess::nothing {
                    chess_map[i][j] = Chess::black;
                    let mut choose_pos = vec![];
                    Self::oper(i as i32, j as i32, &mut chess_map, &mut choose_pos);
                    result = result.max(choose_pos.len() as i32);
                    for pos in choose_pos {
                        chess_map[pos.0 as usize][pos.1 as usize] = Chess::white;
                    }
                    chess_map[i][j] = Chess::nothing;
                }
            }
        }

        result
    }

    fn oper(x: i32, y: i32, chess_map: &mut Vec<Vec<Chess>>, choose_pos: &mut Vec<(i32, i32)>) {
        let mut change_pos = vec![];
        let n = chess_map.len() as i32;
        let m = chess_map[0].len() as i32;
        let directions = [
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for dir in directions {
            let mut white_chess = vec![];
            let mut curr_x = x;
            let mut curr_y = y;
            loop {
                let next_x = curr_x + dir.0;
                let next_y = curr_y + dir.1;
                if next_x >= n || next_y >= m || next_x < 0 || next_y < 0 {
                    break;
                }

                let next_chess = chess_map[next_x as usize][next_y as usize];
                if next_chess == Chess::nothing {
                    break;
                } else if next_chess == Chess::white {
                    white_chess.push((next_x, next_y));
                } else {
                    for p in white_chess {
                        change_pos.push(p);
                    }
                    break;
                }
                curr_x = next_x;
                curr_y = next_y;
            }
        }

        if change_pos.is_empty() {
            return;
        }

        for p in change_pos.iter() {
            chess_map[p.0 as usize][p.1 as usize] = Chess::black;
            choose_pos.push(*p);
        }

        for p in change_pos.iter() {
            Self::oper(p.0, p.1, chess_map, choose_pos);
        }
    }
}

#[test]
fn test() {}
