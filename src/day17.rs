use std::{fmt::Debug, collections::HashMap};

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

        let mut floor_repeater: Option<(i64, i64, i64)> = None;
        let mut blows = 0;

        // let rocks = 2022;
        // let rocks = 1000000000000;

        let mut dict: HashMap<(i64, i64, Vec<bool>), (i64, i64)> = HashMap::new();

        'mainloop: for ii in 0..rocks {
            // println!("Max height is {}", max_height);

            let mut falling_piece = FallingPiece {
                shape: pieces[ii % pieces.len()].clone(),
                location: (
                    3,
                    max_height as i32 + 4 + pieces[ii % pieces.len()].cells.len() as i32 - 1,
                ),
            };

            if ii > 0 && 
            //    blows % directions.len() == 0 && 
            //    ii % pieces.len() == 0 && 
               grid.cells[max_height as usize].iter().filter(|b| **b).count() > 5 {
                // We've done a full set - check/cache state
                if dict.contains_key(&(ii as i64 % pieces.len() as i64, blows as i64 % directions.len() as i64, grid.cells[max_height as usize].clone())) {
                    println!("SCENES!");

                    let val = dict[&(ii as i64 % pieces.len() as i64, blows as i64 % directions.len() as i64, grid.cells[max_height as usize].clone())];

                    floor_repeater = Some((ii as i64, val.0, real_max_height - val.1));
                    // println!("max_height is {}", max_height);
                    
                    break 'mainloop;                    

                } else {
                    println!("Caching");
                    dict.insert((ii as i64 % pieces.len() as i64, blows as i64 % directions.len() as i64, grid.cells[max_height as usize].clone()), (ii as i64, real_max_height));
                }
            }

            loop {
                // let next_direction = {
                //     let d = directions_iter.next();
                //     if let Some(dir) = d {
                //         dir
                //     } else {
                //         directions_iter = directions.iter();
                //         directions_iter.next().unwrap()
                //     }
                // };
                let next_dir = directions_iter.next().unwrap();
                // println!("{:?}", next_dir);

                // print!(".");
                falling_piece.try_move(next_dir.to_vector(), &mut grid, false, false);
                blows += 1;
                // print!("+");

                if !falling_piece.try_move(Direction::Down.to_vector(), &mut grid, true, false) {
                    // println!("Settled");
                    let new_candidate_max_height = falling_piece.location.1 as i64;
                    let extra_rows = new_candidate_max_height - max_height;

                    if extra_rows > 0 {
                        for _ in 0..extra_rows {
                            // println!("Adding row");
                            grid.add_row();
                            max_height += 1;
                            real_max_height += 1;
                        }
                    }

                    // see if the bottom row is solid...
                    let mut solid = true;
                    for ii in 1..9 {
                        if !grid.cells[new_candidate_max_height as usize][ii] {
                            solid = false;
                            break;
                        }
                        
                    }
                    if solid {
                        
                        // println!("Solid!");



                        // println!("{:?}", grid);
                        // println!("{}", new_candidate_max_height);
                        // return (0, 0);
                        // delete lower rows
                        for _ in 0..new_candidate_max_height {
                            grid.cells.remove(1);
                            max_height -= 1;
                        }

                        if extra_rows == 0 {
                            // println!("Floor!!! {ii}");
                            // println!("{:?}", grid); 
                            assert!(max_height == 0);
                            
                            // Let's fast-forward!!
                            if (ii + 1) % directions.len() == 0 {
                                // floor_repeater = Some((ii + 1) as i64);
                                // // println!("max_height is {}", max_height);
                                
                                // break 'mainloop;
                            }
                        }                        
                    }
                    break;
                }
            }
            // println!("{:?}", grid);
        }

        if let Some(sneaky) = floor_repeater {
            println!("Being sneaky");

            let difference = sneaky.0 - sneaky.1;
            let dh = sneaky.2;
            let left = rocks as i64 - sneaky.0;

            // let sneaky = sneaky + 1;
            let repeats = left / difference;
            let mut index = sneaky.0;

            real_max_height += repeats * dh;
            index += difference * repeats;

            // for _ in 0..repeats {
            //     real_max_height += dh;
            //     index += difference;
            // }
            // let pretend = repeats * sneaky;
            // let remainder = rocks as i64 - pretend;
            
            // real_max_height *= repeats;

            // let dir_offset = (repeats * blows) % directions.len() as i64;
            // // let pieces_offset = pretend % pieces.len() as i64;

            println!("There are {} directions, {} blows, {} pieces", directions.len(), blows, pieces.len());
            println!("Sneaky is {:?}, repeats {}, difference {}, dh {}, left {}, new left {}, height before remainder {}", sneaky, repeats, difference, dh, left, rocks as i64 - index, real_max_height);

            // let mut directions_iter = directions.iter().cycle().skip(dir_offset as usize);

            // max_height = 0;
            // grid = Grid::new('#');

            for ii in (index as usize)..rocks {
                // println!("Max height is {}", max_height);
                // println!("ii is {}", ii);
    
                let mut falling_piece = FallingPiece {
                    shape: pieces[ii % pieces.len()].clone(),
                    location: (
                        3,
                        max_height as i32 + 4 + pieces[ii % pieces.len()].cells.len() as i32 - 1,
                    ),
                };
    
                loop {
                    
                    // let next_direction = {
                    //     let d = directions_iter.next();
                    //     if let Some(dir) = d {
                    //         dir
                    //     } else {
                    //         directions_iter = directions.iter();
                    //         directions_iter.next().unwrap()
                    //     }
                    // };
                    let next_dir = directions_iter.next().unwrap();
                    // println!("{:?}", next_dir);
    
                    // print!(".");
                    falling_piece.try_move(next_dir.to_vector(), &mut grid, false, false);
                    // print!("+");
                    // println!("{:?}", falling_piece.location);
    
                    if !falling_piece.try_move(Direction::Down.to_vector(), &mut grid, true, false) {
                        // println!("Settled");
                        let new_candidate_max_height = falling_piece.location.1 as i64;
                        let extra_rows = new_candidate_max_height - max_height;
    
                        if extra_rows > 0 {
                            for _ in 0..extra_rows {
                                // println!("Adding row");
                                grid.add_row();
                                max_height += 1;
                                real_max_height += 1;
                                // println!("Max height now {} -> {}", max_height, real_max_height);
                            }
                        }
                        // println!("{:?}", grid);
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
    fn try_move(&mut self, dir: (i32, i32), grid: &mut Grid, settle: bool, print: bool) -> bool {
        let new_location = (self.location.0 + dir.0, self.location.1 + dir.1);

        if print && dir.1 < 0 {
            println!(
                "Checking move from {:?} to {:?}",
                self.location, new_location
            );
        }

        let can_move = !grid.overlaps(self, new_location);
        if can_move {
            if print {
                println!("Can move");
            }
            self.location = new_location;
        } else if settle {
            if print {
                println!("Settling");
            }
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
        

        // println!("{:?}", grid);

        (Grid::run(&directions, 2022), Grid::run(&directions, 1000000000000))
    }
}

// 1514285714288 example
// 1514285714288 out!!

// 1521739130437 1 - too low
// 1521739130440 2 - too low
// 1518438177884
// 1537409572839
// 1537097413557
// 1537175792495