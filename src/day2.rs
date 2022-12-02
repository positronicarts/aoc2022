pub struct Day2;

#[derive(Clone)]
enum RPSOptions {
    Rock,
    Paper,
    Scissors,
}

enum DesiredResult {
    Win,
    Lose,
    Draw,
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

    fn should_play(against: &Self, desired_result: &DesiredResult) -> Self {
        match (against, desired_result) {
            (RPSOptions::Rock, DesiredResult::Draw)
            | (RPSOptions::Paper, DesiredResult::Lose)
            | (RPSOptions::Scissors, DesiredResult::Win) => RPSOptions::Rock,
            (RPSOptions::Paper, DesiredResult::Draw)
            | (RPSOptions::Scissors, DesiredResult::Lose)
            | (RPSOptions::Rock, DesiredResult::Win) => RPSOptions::Paper,
            (RPSOptions::Scissors, DesiredResult::Draw)
            | (RPSOptions::Rock, DesiredResult::Lose)
            | (RPSOptions::Paper, DesiredResult::Win) => RPSOptions::Scissors,
        }
    }
}

impl DesiredResult {
    fn parse(c: char) -> Self {
        match c {
            'X' => DesiredResult::Lose,
            'Y' => DesiredResult::Draw,
            'Z' => DesiredResult::Win,
            _ => panic!("Invalid character {c}"),
        }
    }
}

impl aoc22::DayInner<Day2, i64> for Day2 {
    fn day(&self) -> i32 {
        2
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let lines: Vec<&str> = input.lines().collect();

        let score1: i64 = lines
            .iter()
            .map(|line| {
                let p1 = RPSOptions::parse(line.chars().next().unwrap());
                let p2 = RPSOptions::parse(line.chars().nth(2).unwrap());
                (p1, p2)
            })
            .map(|(p1, p2)| p2.score_against(&p1))
            .sum();

        let score2: i64 = lines
            .iter()
            .map(|line| {
                let p1 = RPSOptions::parse(line.chars().next().unwrap());
                let desired_result = DesiredResult::parse(line.chars().nth(2).unwrap());
                (p1, desired_result)
            })
            .map(|(p1, desired_result)| (p1.clone(), RPSOptions::should_play(&p1, &desired_result)))
            .map(|(p1, p2)| p2.score_against(&p1))
            .sum();

        // And we're done!
        (score1, score2)
    }
}
