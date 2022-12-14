use std::fmt::{Debug, Write};

pub struct Day14;

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Sand => 'o',
        };
        f.write_char(c)?;
        Ok(())
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                f.write_fmt(format_args!("{:?}", cell))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Cell {
        if x < 0 || x >= self.cells[0].len() as i32 || y >= self.cells.len() as i32 {
            Cell::Empty
        } else {
            self.cells[y as usize][x as usize]
        }
    }

    fn settle(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.cells[0].len() as i32 || y >= self.cells.len() as i32 {
        } else {
            self.cells[y as usize][x as usize] = Cell::Sand;
        }
    }

    fn add_sand(&mut self, drop_x: i32) -> bool {
        let mut trial: (i32, i32) = (drop_x as i32, 0);
        loop {
            if self.get(trial.0, trial.1 + 1) != Cell::Empty
                && self.get(trial.0 - 1, trial.1 + 1) != Cell::Empty
                && self.get(trial.0 + 1, trial.1 + 1) != Cell::Empty
            {
                // Settle here
                self.settle(trial.0, trial.1);

                if trial.1 == 0 && trial.0 == drop_x {
                    return true;
                }

                return false;
            } else if self.get(trial.0, trial.1 + 1) != Cell::Empty
                && self.get(trial.0 - 1, trial.1 + 1) != Cell::Empty
            {
                // Go right
                trial = (trial.0 + 1, trial.1 + 1);
            } else if self.get(trial.0, trial.1 + 1) != Cell::Empty {
                // Go left
                trial = (trial.0 - 1, trial.1 + 1);
            } else {
                // Go down
                trial = (trial.0, trial.1 + 1);
            }

            // If OOB, we're done
            if trial.0 < 0
                || trial.0 as usize >= self.cells[0].len()
                || trial.1 as usize >= self.cells.len()
            {
                return true;
            }
        }
    }
}

impl aoc22::DayInner<Day14, i32> for Day14 {
    fn day(&self) -> i32 {
        14
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut walls: Vec<Vec<(i32, i32)>> = input
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|s| {
                        let mut split = s.split(',');
                        (
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                        )
                    })
                    .collect()
            })
            .collect();

        let mut xmin = walls
            .iter()
            .map(|w| w.iter().map(|ww| ww.0).min().unwrap())
            .min()
            .unwrap();
        let mut xmax = walls
            .iter()
            .map(|w| w.iter().map(|ww| ww.0).max().unwrap())
            .max()
            .unwrap();

        let mut ymax = walls
            .iter()
            .map(|w| w.iter().map(|ww| ww.1).max().unwrap())
            .max()
            .unwrap();

        let p1 = {
            let mut grid = Grid::default();

            for _ in 0..ymax + 1 {
                let mut row = vec![];
                for _ in xmin..xmax + 1 {
                    row.push(Cell::Empty);
                }
                grid.cells.push(row);
            }

            for wall in walls.iter() {
                let mut wall_iter = wall.iter();
                let mut start = wall_iter.next().unwrap();
                loop {
                    let end = wall_iter.next();
                    if end.is_none() {
                        break;
                    }
                    let end = end.unwrap();

                    let start_x = std::cmp::min(start.0, end.0);
                    let end_x = std::cmp::max(start.0, end.0);
                    let start_y = std::cmp::min(start.1, end.1);
                    let end_y = std::cmp::max(start.1, end.1);

                    for yy in start_y..end_y + 1 {
                        grid.cells[yy as usize][start.0 as usize - xmin as usize] = Cell::Wall;
                    }
                    for xx in start_x..end_x + 1 {
                        grid.cells[start.1 as usize][xx as usize - xmin as usize] = Cell::Wall;
                    }
                    start = end;
                }
            }

            let mut count = 0;
            while !grid.add_sand(500 - xmin) {
                count += 1;
            }

            count
        };

        let p2 = {
            ymax += 2;
            xmin -= ymax;
            xmax += ymax;
            walls.push(vec![(xmin, ymax), (xmax, ymax)]);

            let mut grid = Grid::default();

            for _ in 0..ymax + 1 {
                let mut row = vec![];
                for _ in xmin..xmax + 1 {
                    row.push(Cell::Empty);
                }
                grid.cells.push(row);
            }

            for wall in walls {
                let mut wall_iter = wall.iter();
                let mut start = wall_iter.next().unwrap();
                loop {
                    let end = wall_iter.next();
                    if end.is_none() {
                        break;
                    }
                    let end = end.unwrap();

                    let start_x = std::cmp::min(start.0, end.0);
                    let end_x = std::cmp::max(start.0, end.0);
                    let start_y = std::cmp::min(start.1, end.1);
                    let end_y = std::cmp::max(start.1, end.1);

                    for yy in start_y..end_y + 1 {
                        grid.cells[yy as usize][start.0 as usize - xmin as usize] = Cell::Wall;
                    }
                    for xx in start_x..end_x + 1 {
                        grid.cells[start.1 as usize][xx as usize - xmin as usize] = Cell::Wall;
                    }
                    start = end;
                }
            }

            let mut count = 0;
            while !grid.add_sand(500 - xmin) {
                count += 1;
            }

            count + 1
        };

        (p1, p2)
    }
}
