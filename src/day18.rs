use regex::Regex;

pub struct Day18;

#[derive(Debug, Clone, PartialEq)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn parse(re: &Regex, input: &str) -> Self {
        let cap = re.captures_iter(input).next().unwrap();
        Cube {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap(),
        }
    }

    fn adjacent_cubes(&self) -> Vec<Cube> {
        vec![
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
    }
}

impl aoc22::DayInner<Day18, usize> for Day18 {
    fn day(&self) -> i32 {
        18
    }

    fn inner(&self, input: String) -> (usize, usize) {
        let re = Regex::new(r"(\d*),(\d*),(\d*)").unwrap();
        let cubes: Vec<Cube> = input.lines().map(|line| Cube::parse(&re, line)).collect();

        let surface_area = cubes
            .iter()
            .map(|cube| {
                cube.adjacent_cubes()
                    .iter()
                    .filter(|adj| !cubes.contains(adj))
                    .count()
            })
            .sum();

        (surface_area, 0)
    }
}
