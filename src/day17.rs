use std::{collections::HashMap, fmt::Debug};

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
                if self.cells[(offset.1 - yy as i32) as usize][(offset.0 + xx as i32) as usize]
                    && row[xx]
                {
                    return true;
                }
            }
        }
        false
    }

    fn settle(&mut self, piece: &FallingPiece) {
        for (yy, row) in piece.shape.cells.iter().enumerate() {
            for (xx, _b) in row.iter().enumerate() {
                self.cells[(piece.location.1 - yy as i32) as usize]
                    [(piece.location.0 + xx as i32) as usize] = row[xx]
                    || self.cells[(piece.location.1 - yy as i32) as usize]
                        [(piece.location.0 + xx as i32) as usize];
            }
        }
    }

    fn get_pieces() -> Vec<Grid> {
        vec![
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
        ]
    }

    fn run(directions: &Vec<Direction>, rocks: usize) -> i64 {
        let mut directions_iter = directions.iter().cycle();

        let pieces = Grid::get_pieces();

        let mut grid = Grid::new('#');
        let mut max_height: i64 = 0;
        let mut real_max_height: i64 = 0;

        let mut shortcut: Option<(i64, i64, i64)> = None;
        let mut blows = 0;

        let mut cache: HashMap<(i64, i64, Vec<bool>), (i64, i64)> = HashMap::new();

        'mainloop: for ii in 0..rocks {
            let mut falling_piece = FallingPiece {
                shape: pieces[ii % pieces.len()].clone(),
                location: (
                    3,
                    max_height as i32 + 4 + pieces[ii % pieces.len()].cells.len() as i32 - 1,
                ),
            };

            if ii > 0
                && grid.cells[max_height as usize]
                    .iter()
                    .filter(|b| **b)
                    .count()
                    > 5
            {
                // Pretty full row - use a cache to look for effective repeats, as a shortcut to completion
                if cache.contains_key(&(
                    ii as i64 % pieces.len() as i64,
                    blows as i64 % directions.len() as i64,
                    grid.cells[max_height as usize].clone(),
                )) {
                    let val = cache[&(
                        ii as i64 % pieces.len() as i64,
                        blows as i64 % directions.len() as i64,
                        grid.cells[max_height as usize].clone(),
                    )];
                    shortcut = Some((ii as i64, val.0, real_max_height - val.1));
                    break 'mainloop;
                } else {
                    cache.insert(
                        (
                            ii as i64 % pieces.len() as i64,
                            blows as i64 % directions.len() as i64,
                            grid.cells[max_height as usize].clone(),
                        ),
                        (ii as i64, real_max_height),
                    );
                }
            }

            loop {
                let next_dir = directions_iter.next().unwrap();
                falling_piece.try_move(next_dir.to_vector(), &mut grid, false);
                blows += 1;

                if !falling_piece.try_move(Direction::Down.to_vector(), &mut grid, true) {
                    let new_candidate_max_height = falling_piece.location.1 as i64;
                    let extra_rows = new_candidate_max_height - max_height;

                    if extra_rows > 0 {
                        for _ in 0..extra_rows {
                            grid.add_row();
                            max_height += 1;
                            real_max_height += 1;
                        }
                    }
                    break;
                }
            }
        }

        if let Some(shortcut_inner) = shortcut {
            let difference = shortcut_inner.0 - shortcut_inner.1;
            let dh = shortcut_inner.2;
            let left = rocks as i64 - shortcut_inner.0;
            let repeats = left / difference;
            let mut index = shortcut_inner.0;

            real_max_height += repeats * dh;
            index += difference * repeats;

            for ii in (index as usize)..rocks {
                let mut falling_piece = FallingPiece {
                    shape: pieces[ii % pieces.len()].clone(),
                    location: (
                        3,
                        max_height as i32 + 4 + pieces[ii % pieces.len()].cells.len() as i32 - 1,
                    ),
                };

                loop {
                    let next_dir = directions_iter.next().unwrap();
                    falling_piece.try_move(next_dir.to_vector(), &mut grid, false);
                    if !falling_piece.try_move(Direction::Down.to_vector(), &mut grid, true)
                    {
                        let new_candidate_max_height = falling_piece.location.1 as i64;
                        let extra_rows = new_candidate_max_height - max_height;

                        if extra_rows > 0 {
                            for _ in 0..extra_rows {
                                grid.add_row();
                                max_height += 1;
                                real_max_height += 1;
                            }
                        }
                        break;
                    }
                }
            }
        }

        real_max_height
    }
}

struct FallingPiece {
    shape: Grid,
    location: (i32, i32),
}

impl FallingPiece {
    fn try_move(&mut self, dir: (i32, i32), grid: &mut Grid, settle: bool) -> bool {
        let new_location = (self.location.0 + dir.0, self.location.1 + dir.1);
        let can_move = !grid.overlaps(self, new_location);
        if can_move {
            self.location = new_location;
        } else if settle {
            grid.settle(self);
        }
        can_move
    }
}

impl aoc22::DayInner<Day17, i64> for Day17 {
    fn day(&self) -> i32 {
        17
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let directions: Vec<Direction> = input.chars().map(Direction::parse).collect();
        (
            Grid::run(&directions, 2022),
            Grid::run(&directions, 1000000000000),
        )
    }
}
