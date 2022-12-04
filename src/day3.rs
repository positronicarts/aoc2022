pub struct Day3;

fn score(c: &char) -> i64 {
    let c64 = *c as i64;

    let lower_a = 'a' as i64;
    let lower_z = 'z' as i64;
    let upper_a = 'A' as i64;

    if c64 >= lower_a && c64 <= lower_z {
        c64 - lower_a + 1
    } else {
        c64 - upper_a + 27
    }
}

fn score_from_line(line: &str) -> i64 {
    let length = line.len();
    let half_length = length / 2;
    let first_half = &line[..half_length];
    let second_half = &line[half_length..];

    for c in first_half.chars() {
        if second_half.contains(c) {
            return score(&c);
        }
    }

    0
}

impl aoc22::DayInner<Day3, i64> for Day3 {
    fn day(&self) -> i32 {
        3
    }

    fn inner(&self, input: String) -> (i64, i64) {
        let lines: Vec<&str> = input.lines().collect();

        let score1 = lines.iter().map(|l| score_from_line(l)).sum();

        let number_of_lines = lines.len();
        let mut index = 0;
        let mut score2 = 0;
        loop {
            let line1 = lines[index];
            let line2 = lines[index + 1];
            let line3 = lines[index + 2];

            for c in line1.chars() {
                if line2.contains(c) && line3.contains(c) {
                    score2 += score(&c);
                    break;
                }
            }

            index += 3;
            if index >= number_of_lines {
                break;
            }
        }

        // And we're done!
        (score1, score2)
    }
}
