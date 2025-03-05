use std::ops::{Add, Div};

struct NeighborSum {
    grid: Vec<Vec<i32>>,
    n: usize,
}

static ADJACENT_DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
static DIAGONAL_DIR: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        return Self {
            n: grid.len(),
            grid,
        };
    }

    fn get_pos(&self, value: i32) -> (usize, usize) {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.grid[i][j] == value {
                    return (i, j);
                }
            }
        }

        return (0, 0);
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let pos = self.get_pos(value);
        let mut sum = 0;

        ADJACENT_DIR.iter().for_each(|x| {
            let n_i = (pos.0 as i32) + x.0;
            let n_j = (pos.1 as i32) + x.1;

            if n_i < 0 || n_i >= self.n as i32 || n_j < 0 || n_j >= self.n as i32 {
            } else {
                sum += self.grid[n_i as usize][n_j as usize];
            }
        });

        return sum;
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let pos = self.get_pos(value);
        let mut sum = 0;

        DIAGONAL_DIR.iter().for_each(|x| {
            let n_i = (pos.0 as i32) + x.0;
            let n_j = (pos.1 as i32) + x.1;

            if n_i < 0 || n_i >= self.n as i32 || n_j < 0 || n_j >= self.n as i32 {
            } else {
                sum += self.grid[n_i as usize][n_j as usize];
            }
        });

        return sum;
    }
}
