use std::fmt::Debug;

pub struct Day17;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn parse(input: char) -> Self {
        if input == '<' {
            Self::Left
        } else {
            Self::Right
        }
    }

    fn to_vector(&self) -> (i32, i32) {
        match self {
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<bool>>,
    show_char: char,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (self.cells.iter()).rev() {
            for b in row.iter() {
                f.write_fmt(format_args!("{}", if *b { self.show_char } else { '.' }))?;
            }
            println!();
        }
        Ok(())
    }
}

impl Grid {
    fn new(show_char: char) -> Self {
        let cells = vec![vec![true, true, true, true, true, true, true, true, true]];
        let mut grid = Grid { cells, show_char };
        for _ in 0..7 {
            grid.add_row();
        }
        grid
    }

    fn add_row(&mut self) {
        self.cells.push(vec![
            true, false, false, false, false, false, false, false, true,
        ])
    }

    fn overlaps(&self, piece: &FallingPiece, offset: (i32, i32)) -> bool {
        for (yy, row) in piece.shape.cells.iter().enumerate() {
            for (xx, _b) in row.iter().enumerate() {
                // println!("Trying ({}, {}) = {}+{}", offset.0 + xx as i32, offset.0 - yy as i32, self.cells[(offset.1 - yy as i32) as usize][(offset.0 + xx as i32) as usize], row[xx]);
                if self.cells[(offset.1 - yy as i32) as usize][(offset.0 + xx as i32) as usize]
                    && row[xx]
                {
                    // println!("Overlap!");
                    return true;
                }
            }
        }
        false
    }

    fn settle(&mut self, piece: &FallingPiece) {
        for (yy, row) in piece.shape.cells.iter().enumerate() {
            for (xx, _b) in row.iter().enumerate() {
                // println!("Putting ({}, {}) = {}", piece.location.0 + xx as i32, piece.location.0 - yy as i32, row[xx]);
                self.cells[(piece.location.1 - yy as i32) as usize]
                    [(piece.location.0 + xx as i32) as usize] = row[xx];
            }
        }
    }
}

struct FallingPiece {
    shape: Grid,
    location: (i32, i32),
}

impl FallingPiece {
    fn try_move(&mut self, dir: (i32, i32), grid: &mut Grid, settle: bool) -> bool {
        let new_location = (self.location.0 + dir.0, self.location.1 + dir.1);
        // println!("Checking move from {:?} to {:?}", self.location, new_location);

        let can_move = !grid.overlaps(self, new_location);
        if can_move {
            // println!("Can move");
            self.location = new_location;
        } else if settle {
            // println!("Settling");
            grid.settle(self);
        }
        can_move
    }
}

impl aoc22::DayInner<Day17, i32> for Day17 {
    fn day(&self) -> i32 {
        17
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let directions: Vec<Direction> = input.chars().map(Direction::parse).collect();
        let mut directions_iter = directions.iter();
        // println!("Directions: {:?}", directions);

        let pieces = vec![
            Grid {
                cells: vec![vec![true, true, true, true]],
                show_char: '@',
            },
            Grid {
                cells: vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, true, false],
                ],
                show_char: '@',
            },
            Grid {
                cells: vec![
                    vec![false, false, true],
                    vec![false, false, true],
                    vec![true, true, true],
                ],
                show_char: '@',
            },
            Grid {
                cells: vec![vec![true], vec![true], vec![true], vec![true]],
                show_char: '@',
            },
            Grid {
                cells: vec![vec![true, true], vec![true, true]],
                show_char: '@',
            },
        ];

        let mut grid = Grid::new('#');
        let mut max_height: i32 = (grid.cells.len() - 4) as i32;

        for ii in 0..2022 {
            // println!("Max height is {}", max_height);

            let mut falling_piece = FallingPiece {
                shape: pieces[ii % pieces.len()].clone(),
                location: (
                    3,
                    max_height + pieces[ii % pieces.len()].cells.len() as i32 - 1,
                ),
            };

            loop {
                let next_direction = {
                    let d = directions_iter.next();
                    if let Some(dir) = d {
                        dir
                    } else {
                        directions_iter = directions.iter();
                        directions_iter.next().unwrap()
                    }
                };

                falling_piece.try_move(next_direction.to_vector(), &mut grid, false);

                if !falling_piece.try_move(Direction::Down.to_vector(), &mut grid, true) {
                    // println!("Settled");
                    let new_candidate_max_height = falling_piece.location.1 + 4;
                    let extra_rows = new_candidate_max_height - max_height;

                    if extra_rows > 0 {
                        for _ in 0..extra_rows {
                            // println!("Adding row");
                            grid.add_row();
                            max_height += 1;
                        }
                    }
                    break;
                }
            }
        }

        println!("{:?}", grid);

        (max_height - 4, 0)
    }
}
