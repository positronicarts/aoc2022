use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"^(\S) (\d*)$").unwrap();
        let gps = re.captures_iter(input).next().unwrap();
        let dist = gps[2].parse().unwrap();
        match &gps[1] {
            "U" => Instruction::Up(dist),
            "D" => Instruction::Down(dist),
            "L" => Instruction::Left(dist),
            "R" => Instruction::Right(dist),
            d => panic!("Unknown direction {d}"),
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up(_) => (0, 1),
            Self::Down(_) => (0, -1),
            Self::Left(_) => (-1, 0),
            Self::Right(_) => (1, 0),
        }
    }
}

#[derive(Debug, Default)]
struct Rope {
    locations: Vec<(i32, i32)>,
    visited_locations_p1: HashSet<(i32, i32)>,
    visited_locations_p2: HashSet<(i32, i32)>,
}

impl Rope {
    fn handle_instruction(&mut self, instruction: &Instruction) {
        let dist = match instruction {
            Instruction::Up(d)
            | Instruction::Down(d)
            | Instruction::Left(d)
            | Instruction::Right(d) => d,
        };
        let dir = instruction.delta();
        for _ in 0..*dist {
            self.locations[0].0 += dir.0;
            self.locations[0].1 += dir.1;

            for ii in 1..10 {
                if (self.locations[ii].0 - self.locations[ii - 1].0)
                    * (self.locations[ii].0 - self.locations[ii - 1].0)
                    > 1
                    || (self.locations[ii].1 - self.locations[ii - 1].1)
                        * (self.locations[ii].1 - self.locations[ii - 1].1)
                        > 1
                {
                    let dx = self.locations[ii].0 - self.locations[ii - 1].0;
                    let dy = self.locations[ii].1 - self.locations[ii - 1].1;
                    if dx != 0 {
                        if dx > 0 {
                            self.locations[ii].0 -= 1;
                        } else {
                            self.locations[ii].0 += 1;
                        }
                    }
                    if dy != 0 {
                        if dy > 0 {
                            self.locations[ii].1 -= 1;
                        } else {
                            self.locations[ii].1 += 1;
                        }
                    }
                }
            }

            self.visited_locations_p1
                .insert((self.locations[1].0, self.locations[1].1));
            self.visited_locations_p2
                .insert((self.locations[9].0, self.locations[9].1));
        }
    }
}
pub struct Day9;

impl aoc22::DayInner<Day9, usize> for Day9 {
    fn day(&self) -> i32 {
        9
    }

    fn inner(&self, input: String) -> (usize, usize) {
        let instructions: Vec<Instruction> = input.lines().map(Instruction::parse).collect();
        let mut rope = Rope {
            ..Default::default()
        };
        for _ in 0..10 {
            rope.locations.push((0, 0));
        }
        for instruction in instructions {
            rope.handle_instruction(&instruction);
        }

        (
            rope.visited_locations_p1.len(),
            rope.visited_locations_p2.len(),
        )
    }
}
