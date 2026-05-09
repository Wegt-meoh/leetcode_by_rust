pub struct Solution;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut start_x = 0;
        let mut start_y = 0;
        let mut len_x = m;
        let mut len_y = n;
        while len_x > 0 && len_y > 0 {
            let mut collect = vec![];
            let mut pos_vec = vec![];
            let mut pos_x = start_x;
            let mut pos_y = start_y;

            // down
            while pos_x - start_x < len_x {
                collect.push(grid[pos_x][pos_y]);
                pos_vec.push([pos_x, pos_y]);
                pos_x += 1;
            }
            pos_y += 1;
            pos_x -= 1;
            // right
            while pos_y - start_y < len_y {
                collect.push(grid[pos_x][pos_y]);
                pos_vec.push([pos_x, pos_y]);
                pos_y += 1;
            }
            pos_y -= 1;
            pos_x -= 1;
            // up
            while pos_x >= start_x {
                collect.push(grid[pos_x][pos_y]);
                pos_vec.push([pos_x, pos_y]);
                if pos_x == start_x {
                    break;
                }
                pos_x -= 1;
            }
            pos_y -= 1;
            // left
            while pos_y > start_y {
                collect.push(grid[pos_x][pos_y]);
                pos_vec.push([pos_x, pos_y]);
                if pos_y == start_y {
                    break;
                }
                pos_y -= 1;
            }

            let times = k as usize % collect.len();
            if times == 0 {
                start_x += 1;
                start_y += 1;
                len_x -= 2;
                len_y -= 2;
                continue;
            }

            let mut index = collect.len() - times;

            for [x, y] in pos_vec {
                grid[x][y] = collect[index];
                index += 1;
                if index == collect.len() {
                    index = 0;
                }
            }

            start_x += 1;
            start_y += 1;
            len_x -= 2;
            len_y -= 2;
        }
        grid
    }
}
