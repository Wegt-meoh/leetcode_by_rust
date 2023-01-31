pub struct Solution {}

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for line in 0..n {
            for row in 0..n {
                if line == row || line + row == n - 1 {
                    if grid[line][row] == 0 {
                        return false;
                    }
                } else {
                    if grid[line][row] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
}
