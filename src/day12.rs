pub struct Day12;

#[derive(Debug, Default, Clone)]
struct Grid {
    heights: Vec<Vec<i32>>,
    best: Vec<Vec<Option<i32>>>,
}

impl Grid {
    fn char_to_height(c: char) -> i32 {
        match c {
            'S' => 0,
            'E' => 27,
            other => other as i32 - 'a' as i32 + 1,
        }
    }

    #[allow(clippy::collapsible_if)]
    fn walk(&mut self, x: usize, y: usize) {
        if x > 0 {
            if (self.best[y][x - 1].is_none()
                || self.best[y][x - 1].unwrap() > self.best[y][x].unwrap() + 1)
                && self.heights[y][x - 1] <= self.heights[y][x] + 1
            {
                self.best[y][x - 1] = Some(self.best[y][x].unwrap() + 1);
                self.walk(x - 1, y);
            }
        }

        if x < self.heights[0].len() - 1 {
            if (self.best[y][x + 1].is_none()
                || self.best[y][x + 1].unwrap() > self.best[y][x].unwrap() + 1)
                && self.heights[y][x + 1] <= self.heights[y][x] + 1
            {
                self.best[y][x + 1] = Some(self.best[y][x].unwrap() + 1);
                self.walk(x + 1, y);
            }
        }

        if y > 0 {
            if (self.best[y - 1][x].is_none()
                || self.best[y - 1][x].unwrap() > self.best[y][x].unwrap() + 1)
                && self.heights[y - 1][x] <= self.heights[y][x] + 1
            {
                self.best[y - 1][x] = Some(self.best[y][x].unwrap() + 1);
                self.walk(x, y - 1);
            }
        }

        if y < self.heights.len() - 1 {
            if (self.best[y + 1][x].is_none()
                || self.best[y + 1][x].unwrap() > self.best[y][x].unwrap() + 1)
                && self.heights[y + 1][x] <= self.heights[y][x] + 1
            {
                self.best[y + 1][x] = Some(self.best[y][x].unwrap() + 1);
                self.walk(x, y + 1);
            }
        }
    }
}

impl aoc22::DayInner<Day12, i32> for Day12 {
    fn day(&self) -> i32 {
        12
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut grid = Grid::default();

        for line in input.lines() {
            grid.heights
                .push(line.chars().map(Grid::char_to_height).collect());
            grid.best
                .push(grid.heights[0].iter().map(|_| None).collect());
        }

        let mut cx = 0;
        let mut cy = 0;
        let mut sx = 0;
        let mut sy = 0;

        let mut starts = vec![];

        for yy in 0..grid.heights.len() {
            for xx in 0..grid.heights[0].len() {
                if grid.heights[yy][xx] == 27 {
                    cx = xx;
                    cy = yy;
                }
                if grid.heights[yy][xx] == 0 {
                    sx = xx;
                    sy = yy;
                }
                if grid.heights[yy][xx] == 1
                    && (xx == 0
                        || yy == 0
                        || xx == grid.heights[0].len() - 1
                        || yy == grid.heights.len() - 1)
                {
                    starts.push((xx, yy));
                }
            }
        }

        starts.insert(0, (sx, sy));

        let results: Vec<i32> = starts
            .iter()
            .filter_map(|(xx, yy)| {
                let mut tgrid = grid.clone();
                tgrid.best[*yy][*xx] = Some(0);
                tgrid.walk(*xx, *yy);
                tgrid.best[cy][cx]
            })
            .collect();

        (results[0], *results.iter().min().unwrap())
    }
}
