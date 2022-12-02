pub struct Day2;

#[derive(Debug)]
enum RPSOptions {
    Rock,
    Paper,
    Scissors,
}

impl RPSOptions {
    fn parse(c: char) -> Self {
        match c {
            'A' | 'X' => RPSOptions::Rock,
            'B' | 'Y' => RPSOptions::Paper,
            'C' | 'Z' => RPSOptions::Scissors,
            _ => panic!("Invalid character {c}"),
        }
    }

    fn individual_score(&self) -> i64 {
        match self {
            RPSOptions::Rock => 1,
            RPSOptions::Paper => 2,
            RPSOptions::Scissors => 3,
        }
    }

    fn score_against(&self, other: &Self) -> i64 {
        let win = matches!(
            (self, other),
            (RPSOptions::Paper, RPSOptions::Rock)
                | (RPSOptions::Scissors, RPSOptions::Paper)
                | (RPSOptions::Rock, RPSOptions::Scissors)
        );

        let draw = matches!(
            (self, other),
            (RPSOptions::Rock, RPSOptions::Rock)
                | (RPSOptions::Paper, RPSOptions::Paper)
                | (RPSOptions::Scissors, RPSOptions::Scissors)
        );

        self.individual_score()
            + if win {
                6
            } else if draw {
                3
            } else {
                0
            }
    }
}

impl aoc22::DayInner<Day2, i64> for Day2 {
    fn day(&self) -> i32 {
        2
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let lines: Vec<&str> = input.lines().collect();
        println!("{}", lines.len());
        let score: i64 = lines
            .iter()
            .map(|line| {
                let p1 = RPSOptions::parse(line.chars().next().unwrap());
                let p2 = RPSOptions::parse(line.chars().nth(2).unwrap());
                (p1, p2)
            })
            .map(|(p1, p2)| p2.score_against(&p1))
            .sum();

        // And we're done!
        (score, 0)
    }
}
