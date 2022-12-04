use regex::Regex;

pub struct Range {
    start: i32,
    end: i32,
}

#[allow(clippy::nonminimal_bool)]
impl Range {
    fn subsets(self, other: Range) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }

    fn overlaps(self, other: Range) -> bool {
        (self.start <= other.end && self.end >= other.start)
            || (other.start <= self.end && other.end >= self.start)
    }
}

pub struct Day4;

impl aoc22::DayInner<Day4, i64> for Day4 {
    fn day(&self) -> i32 {
        4
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

        let lines: Vec<&str> = input.lines().collect();

        let score1 = lines
            .iter()
            .filter(|line| {
                let cap = re.captures_iter(line).next().unwrap();
                let r1 = Range {
                    start: cap[1].parse().unwrap(),
                    end: cap[2].parse().unwrap(),
                };
                let r2 = Range {
                    start: cap[3].parse().unwrap(),
                    end: cap[4].parse().unwrap(),
                };

                r1.subsets(r2)
            })
            .count();

        let score2 = lines
            .iter()
            .filter(|line| {
                let cap = re.captures_iter(line).next().unwrap();
                let r1 = Range {
                    start: cap[1].parse().unwrap(),
                    end: cap[2].parse().unwrap(),
                };
                let r2 = Range {
                    start: cap[3].parse().unwrap(),
                    end: cap[4].parse().unwrap(),
                };

                r1.overlaps(r2)
            })
            .count();

        // And we're done!
        (score1 as i64, score2 as i64)
    }
}
