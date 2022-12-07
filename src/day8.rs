use std::collections::HashSet;

pub struct Day8;

impl aoc22::DayInner<Day8, usize> for Day8 {
    fn day(&self) -> i32 {
        8
    }

    fn inner(&self, input: String) -> (usize, usize) {
        let lines: Vec<_> = input.lines().collect();
        let mut grid: Vec<Vec<i32>> = vec![];
        let mut coords: HashSet<(usize, usize)> = HashSet::<(usize, usize)>::new();

        for line in lines {
            let row = line
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect();
            grid.push(row);
        }

        let grid_width = grid[0].len();
        let grid_height = grid.len();

        // Vertical rays
        for xx in 0..grid_width {
            let mut height = -1;

            for (yy, row) in grid.iter().enumerate().take(grid_height) {
                let check_height = row[xx];
                if check_height > height {
                    coords.insert((xx, yy));
                    height = check_height;
                }
            }

            height = -1;
            for yy in 0..grid_height {
                let check_height = grid[grid_height - 1 - yy][xx];
                if check_height > height {
                    coords.insert((xx, grid_height - 1 - yy));
                    height = check_height;
                }
            }
        }

        // Hoirzontal rays
        for (yy, row) in grid.iter().enumerate().take(grid_height) {
            let mut height = -1;

            for (xx, check_height) in row.iter().enumerate().take(grid_width) {
                //let check_height = row[xx];
                if *check_height > height {
                    coords.insert((xx, yy));
                    height = *check_height;
                }
            }

            height = -1;
            for xx in 0..grid_width {
                let check_height = grid[yy][grid_width - 1 - xx];
                if check_height > height {
                    coords.insert((grid_width - 1 - xx, yy));
                    height = check_height;
                }
            }
        }

        let mut max_score = 0;

        for check_x in 1..grid_width - 1 {
            for check_y in 1..grid_height - 1 {
                let height = grid[check_y][check_x];

                let mut scores = vec![];

                // Vertical rays
                scores.insert(0, 0);

                for row in grid.iter().take(grid_height).skip(check_y + 1) {
                    let check_height = row[check_x];
                    if check_height < height {
                        scores[0] += 1;
                    } else {
                        scores[0] += 1;
                        break;
                    }
                }

                scores.insert(0, 0);
                for yy in 0..check_y {
                    let check_height = grid[check_y - 1 - yy][check_x];
                    if check_height < height {
                        scores[0] += 1;
                    } else {
                        scores[0] += 1;
                        break;
                    }
                }

                // Hoirzontal rays
                scores.insert(0, 0);

                for xx in check_x + 1..grid_width {
                    let check_height = grid[check_y][xx];
                    if check_height < height {
                        scores[0] += 1;
                    } else {
                        scores[0] += 1;
                        break;
                    }
                }

                scores.insert(0, 0);

                for xx in 0..check_x {
                    let check_height = grid[check_y][check_x - 1 - xx];
                    if check_height < height {
                        scores[0] += 1;
                    } else {
                        scores[0] += 1;
                        break;
                    }
                }

                let new_score: i32 = scores.iter().product();
                if new_score > max_score {
                    max_score = new_score;
                }
            }
        }

        (coords.len(), max_score as usize)
    }
}
