use std::collections::HashSet;

use regex::Regex;

pub struct Day18;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

        // Determine range of the cube
        let xmin = cubes.iter().map(|c| c.x).min().unwrap() - 1;
        let xmax = cubes.iter().map(|c| c.x).max().unwrap() + 1;
        let ymin = cubes.iter().map(|c| c.y).min().unwrap() - 1;
        let ymax = cubes.iter().map(|c| c.y).max().unwrap() + 1;
        let zmin = cubes.iter().map(|c| c.z).min().unwrap() - 1;
        let zmax = cubes.iter().map(|c| c.z).max().unwrap() + 1;

        // Walk within the cube, looking for space that is empty
        let seed = Cube { x: 0, y: 0, z: 0 };
        assert!(!cubes.contains(&seed));
        let mut open_set: HashSet<Cube> = HashSet::new();
        let mut reachable_set: HashSet<Cube> = HashSet::new();
        open_set.insert(seed);

        while !open_set.is_empty() {
            let test = open_set.iter().next().unwrap().clone();
            open_set.remove(&test);

            for test_adjacent in test.adjacent_cubes().iter() {
                if test_adjacent.x < xmin
                    || test_adjacent.x > xmax
                    || test_adjacent.y < ymin
                    || test_adjacent.y > ymax
                    || test_adjacent.z < zmin
                    || test_adjacent.z > zmax
                {
                    continue;
                }

                if !cubes.contains(test_adjacent)
                    && !open_set.contains(test_adjacent)
                    && !reachable_set.contains(test_adjacent)
                {
                    open_set.insert(test_adjacent.clone());
                }
            }
            reachable_set.insert(test.clone());
        }

        let p1 = cubes
            .iter()
            .map(|cube| {
                cube.adjacent_cubes()
                    .iter()
                    .filter(|adj| !cubes.contains(adj))
                    .count()
            })
            .sum();

        let p2 = cubes
            .iter()
            .map(|cube| {
                cube.adjacent_cubes()
                    .iter()
                    .filter(|adj| reachable_set.contains(adj))
                    .count()
            })
            .sum();

        (p1, p2)
    }
}
